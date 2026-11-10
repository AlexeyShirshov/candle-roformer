//! MDX-Net (ONNX) separation on CPU via the pure-Rust `tract-onnx` runtime.
//!
//! The implementation mirrors `audio-separator` 0.48.0's `MDXSeparator` and
//! `STFT` for the pinned `UVR-MDX-NET-Inst_HQ_3` profile:
//!
//! * the published `.onnx` checkpoint is user-supplied and never downloaded,
//!   bundled or cached; its SHA-256 is verified before any inference runs;
//! * the input is peak-normalized (scale-down only) to the profile threshold,
//!   demixed in overlapping segments with a symmetric Hann window, divided by
//!   the accumulated window and trimmed back to the original length;
//! * the secondary stem is the reference arithmetic residual
//!   `-primary * compensate + mix`;
//! * each output stem is peak-normalized (scale-down only) independently.
//!
//! Everything runs on the CPU: the ONNX graph is not dispatched to CUDA/Metal.

use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use sha2::{Digest, Sha256};
use tract_onnx::prelude::*;

use crate::dsp;
use crate::error::{Error, MdxError};
use crate::separator::Stems;
use crate::wav::AudioBuffer;

/// The execution plan type returned by `tract`'s `into_runnable` for a typed ONNX graph.
type MdxPlan = Arc<tract_onnx::tract_core::model::typed::TypedSimplePlan>;

/// Architecture and window parameters of one published MDX-Net checkpoint.
///
/// These values are the `mdx_model_data.json` entries that `audio-separator`
/// 0.48.0 uses for the model, plus the pinned digest of the published file.
#[derive(Debug, Clone, PartialEq)]
pub struct MdxProfile {
    /// Human-readable profile name.
    pub name: &'static str,
    /// Published file name resolved inside the caller-supplied model directory.
    pub model_file: &'static str,
    /// Pinned SHA-256 of `model_file` (hex, lower case).
    pub model_sha256: &'static str,
    /// Name of the primary (target) stem.
    pub primary_stem: &'static str,
    /// Name of the secondary (residual) stem.
    pub secondary_stem: &'static str,
    /// Reference `compensate` multiplier for the secondary stem.
    pub compensate: f32,
    /// Frequency bins fed to the model (`mdx_dim_f_set`).
    pub dim_f: usize,
    /// Time frames fed to the model (`2 ** mdx_dim_t_set`).
    pub dim_t: usize,
    /// FFT size (`mdx_n_fft_scale_set`).
    pub n_fft: usize,
    /// STFT hop length.
    pub hop_length: usize,
    /// Segment size in hops (`segment_size`); the segment is `hop * (size - 1)` samples.
    pub segment_size: usize,
    /// Fractional overlap between consecutive segments.
    pub overlap: f32,
    /// Model sample rate in Hz.
    pub sample_rate: usize,
    /// Peak normalization threshold applied to the mix and to each stem.
    pub normalization_threshold: f32,
}

/// The pinned `UVR-MDX-NET-Inst_HQ_3` profile.
///
/// SHA-256 `317554b07fe1ea5279a77f2b1520a41ea4b93432560c4ffd08792c30fddf9adc`
/// (66,759,214 bytes), identical to `audio-separator` 0.48.0's own
/// `model-checksums.json` entry for `UVR-MDX-NET-Inst_HQ_3.onnx`.
pub const fn uvr_mdx_net_inst_hq_3() -> MdxProfile {
    MdxProfile {
        name: "UVR-MDX-NET Inst HQ 3",
        model_file: "UVR-MDX-NET-Inst_HQ_3.onnx",
        model_sha256: "317554b07fe1ea5279a77f2b1520a41ea4b93432560c4ffd08792c30fddf9adc",
        primary_stem: "Instrumental",
        secondary_stem: "Vocals",
        compensate: 1.022,
        dim_f: 3072,
        dim_t: 256,
        n_fft: 6144,
        hop_length: 1024,
        segment_size: 256,
        overlap: 0.25,
        sample_rate: 44100,
        normalization_threshold: 0.9,
    }
}

impl MdxProfile {
    /// Demix segment length in samples: `hop_length * (segment_size - 1)`.
    pub fn chunk_size(&self) -> usize {
        self.hop_length * (self.segment_size.saturating_sub(1))
    }

    /// Samples trimmed from each side of a segment: `n_fft / 2`.
    pub fn trim(&self) -> usize {
        self.n_fft / 2
    }

    /// New samples contributed by one segment: `chunk_size - 2 * trim`.
    pub fn gen_size(&self) -> usize {
        self.chunk_size().saturating_sub(2 * self.trim())
    }

    /// Overlap-add step: `int((1 - overlap) * chunk_size)`.
    pub fn step(&self) -> usize {
        ((1.0 - self.overlap) * self.chunk_size() as f32) as usize
    }

    /// Number of STFT frames produced by one demix segment; must equal `dim_t`.
    pub fn segment_frames(&self) -> usize {
        self.chunk_size() / self.hop_length + 1
    }

    /// Path of the model file inside a caller-supplied model directory.
    pub fn asset_path(&self, dir: impl AsRef<Path>) -> PathBuf {
        dir.as_ref().join(self.model_file)
    }

    /// Validate the internal consistency of the profile.
    pub fn validate(&self) -> Result<(), MdxError> {
        if self.model_file.is_empty() {
            return Err(MdxError::InvalidProfile("model_file is empty".into()));
        }
        if self.model_sha256.len() != 64
            || !self
                .model_sha256
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(MdxError::InvalidProfile(
                "model_sha256 must be 64 lower-case hex characters".into(),
            ));
        }
        if self.dim_f == 0 || self.dim_t == 0 {
            return Err(MdxError::InvalidProfile("dim_f/dim_t must be > 0".into()));
        }
        if self.n_fft == 0 || self.n_fft % 2 != 0 {
            return Err(MdxError::InvalidProfile(
                "n_fft must be positive even".into(),
            ));
        }
        if self.hop_length == 0 || self.segment_size < 2 {
            return Err(MdxError::InvalidProfile(
                "hop_length must be > 0 and segment_size >= 2".into(),
            ));
        }
        if self.dim_f != self.n_fft / 2 {
            return Err(MdxError::InvalidProfile(format!(
                "dim_f ({}) must equal n_fft / 2 ({})",
                self.dim_f,
                self.n_fft / 2
            )));
        }
        if self.segment_frames() != self.dim_t {
            return Err(MdxError::InvalidProfile(format!(
                "segment produces {} frames but dim_t is {}",
                self.segment_frames(),
                self.dim_t
            )));
        }
        if !(0.0..1.0).contains(&self.overlap) || self.step() == 0 {
            return Err(MdxError::InvalidProfile(
                "overlap must be in [0, 1) and yield a positive step".into(),
            ));
        }
        if self.sample_rate == 0 {
            return Err(MdxError::InvalidProfile("sample_rate must be > 0".into()));
        }
        if !self.normalization_threshold.is_finite() || self.normalization_threshold <= 0.0 {
            return Err(MdxError::InvalidProfile(
                "normalization_threshold must be finite and > 0".into(),
            ));
        }
        if !self.compensate.is_finite() {
            return Err(MdxError::InvalidProfile("compensate must be finite".into()));
        }
        Ok(())
    }

    /// Resolve `model_file` inside `dir` and verify its pinned SHA-256.
    pub fn resolve_verified_asset(&self, dir: impl AsRef<Path>) -> Result<PathBuf, MdxError> {
        let path = self.asset_path(dir);
        if !path.is_file() {
            return Err(MdxError::MissingAsset(format!(
                "{} (profile `{}`)",
                path.display(),
                self.name
            )));
        }
        verify_sha256(&path, self.model_sha256)?;
        Ok(path)
    }
}

/// SHA-256 of a file as lower-case hex.
pub fn sha256_hex(path: impl AsRef<Path>) -> Result<String, MdxError> {
    let mut file = std::fs::File::open(path.as_ref())
        .map_err(|e| MdxError::MissingAsset(format!("{}: {}", path.as_ref().display(), e)))?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = file
            .read(&mut buf)
            .map_err(|e| MdxError::Inference(format!("cannot read asset: {}", e)))?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

/// Verify a file's SHA-256 against `expected_hex` (case-insensitive).
pub fn verify_sha256(path: impl AsRef<Path>, expected_hex: &str) -> Result<(), MdxError> {
    let actual = sha256_hex(path)?;
    if actual.eq_ignore_ascii_case(expected_hex) {
        Ok(())
    } else {
        Err(MdxError::ChecksumMismatch {
            expected: expected_hex.to_ascii_lowercase(),
            actual,
        })
    }
}

fn infer_err<E: std::fmt::Display>(e: E) -> MdxError {
    MdxError::Inference(e.to_string())
}

/// A loaded MDX-Net ONNX graph bound to the CPU runtime.
pub struct MdxSeparator {
    profile: MdxProfile,
    plan: MdxPlan,
}

impl MdxSeparator {
    /// Load and verify a model from an explicit file path.
    ///
    /// The pinned SHA-256 is checked **before** the graph is parsed, so a
    /// replaced or truncated checkpoint fails with a typed error.
    pub fn load(profile: MdxProfile, model_path: impl AsRef<Path>) -> Result<Self, Error> {
        profile.validate()?;
        verify_sha256(model_path.as_ref(), profile.model_sha256)?;
        let plan = tract_onnx::onnx()
            .model_for_path(model_path.as_ref())
            .map_err(infer_err)?
            .into_optimized()
            .map_err(infer_err)?
            .into_runnable()
            .map_err(infer_err)?;
        Ok(Self { profile, plan })
    }

    /// Load the profile's published file from a caller-supplied model directory,
    /// verifying its checksum first.
    pub fn load_from_dir(profile: MdxProfile, dir: impl AsRef<Path>) -> Result<Self, Error> {
        let path = profile.resolve_verified_asset(dir)?;
        Self::load(profile, path)
    }

    pub fn profile(&self) -> &MdxProfile {
        &self.profile
    }

    /// The inference configuration matching this family's reference defaults.
    pub fn inference_config(&self) -> crate::config::InferenceConfig {
        crate::config::InferenceConfig::mdx()
    }

    /// Separate a stereo (or mono, duplicated) buffer at the profile sample rate.
    ///
    /// CPU only. The returned stems carry the profile's stem names and the
    /// input's exact length and sample rate.
    pub fn separate(&self, input: &AudioBuffer) -> Result<Stems, Error> {
        input.validate()?;
        let planar = planar_stereo(input, self.profile.sample_rate)?;
        let n = input.planar.len() / input.channels as usize;
        if n == 0 {
            return Err(MdxError::UnsupportedInput("empty audio".into()).into());
        }

        let mut mix = planar.clone();
        let peak = dsp::peak_abs(&mix);
        dsp::normalize_max_peak(&mut mix, self.profile.normalization_threshold);

        let demixed = self.demix(&mix, n)?;

        let frames = n;
        let mut primary = vec![0f32; 2 * frames];
        let mut secondary = vec![0f32; 2 * frames];
        for ch in 0..2 {
            for i in 0..frames {
                let p = demixed[ch * frames + i] * peak;
                primary[ch * frames + i] = p;
                secondary[ch * frames + i] = -p * self.profile.compensate + mix[ch * frames + i];
            }
        }
        dsp::normalize_max_peak(&mut primary, self.profile.normalization_threshold);
        dsp::normalize_max_peak(&mut secondary, self.profile.normalization_threshold);

        let sample_rate = input.sample_rate;
        // Mono input is duplicated for the model and downmixed back on output;
        // stereo stays stereo. The length always equals the input length.
        let (channels, primary, secondary) = if input.channels == 1 {
            let mono = |planar: &[f32]| -> Vec<f32> {
                (0..frames)
                    .map(|i| (planar[i] + planar[frames + i]) * 0.5)
                    .collect()
            };
            (1u16, mono(&primary), mono(&secondary))
        } else {
            (2u16, primary, secondary)
        };
        let primary = AudioBuffer {
            sample_rate,
            channels,
            planar: primary,
        };
        let secondary = AudioBuffer {
            sample_rate,
            channels,
            planar: secondary,
        };
        let stems = vec![
            (self.profile.primary_stem.to_string(), primary),
            (self.profile.secondary_stem.to_string(), secondary),
        ];
        // Two-stem convenience fields, resolved by stem name: the pinned
        // profile's primary stem is *Instrumental* (the model output) and its
        // secondary stem is the *Vocals* residual, so the fields must follow
        // the names rather than the primary/secondary ordering.
        let by_name = |name: &str| {
            stems
                .iter()
                .find(|(n, _)| n.eq_ignore_ascii_case(name))
                .map(|(_, b)| b.clone())
        };
        let vocals = by_name("vocals")
            .or_else(|| stems.get(1).map(|(_, b)| b.clone()))
            .unwrap_or_else(|| stems[0].1.clone());
        let instrumental = by_name("instrumental")
            .or_else(|| stems.first().map(|(_, b)| b.clone()))
            .unwrap_or_else(|| stems[0].1.clone());
        Ok(Stems {
            vocals,
            instrumental,
            stems,
        })
    }

    /// Reference demix: pad, run overlapping segments, window, overlap-add,
    /// divide by the accumulated window, trim, and return `2 * n` planar
    /// samples (channel 0 then channel 1).
    fn demix(&self, mix: &[f32], n: usize) -> Result<Vec<f32>, Error> {
        let p = &self.profile;
        let trim = p.trim();
        let chunk_size = p.chunk_size();
        let gen_size = p.gen_size();
        let step = p.step();
        let pad = gen_size + trim - (n % gen_size);
        let total = trim + n + pad;

        let mut mixture = vec![0f32; 2 * total];
        for ch in 0..2 {
            mixture[ch * total + trim..ch * total + trim + n]
                .copy_from_slice(&mix[ch * n..(ch + 1) * n]);
        }

        let mut result = vec![0f32; 2 * total];
        let mut divider = vec![0f32; total];

        let mut start = 0usize;
        while start < total {
            let end = (start + chunk_size).min(total);
            let chunk_len = end - start;

            let mut chunk = vec![0f32; 2 * chunk_size];
            for ch in 0..2 {
                chunk[ch * chunk_size..ch * chunk_size + chunk_len]
                    .copy_from_slice(&mixture[ch * total + start..ch * total + end]);
            }

            let mag = self.run_model(&chunk)?;

            let window = hann_window_symmetric(chunk_len);
            for ch in 0..2 {
                for i in 0..chunk_len {
                    result[ch * total + start + i] += mag[ch * chunk_size + i] * window[i];
                }
            }
            for i in 0..chunk_len {
                divider[start + i] += window[i];
            }
            start += step;
        }

        for ch in 0..2 {
            for i in 0..total {
                if divider[i].abs() > 1e-8 {
                    result[ch * total + i] /= divider[i];
                }
            }
        }

        let mut out = vec![0f32; 2 * n];
        for ch in 0..2 {
            out[ch * n..(ch + 1) * n]
                .copy_from_slice(&result[ch * total + trim..ch * total + trim + n]);
        }
        Ok(out)
    }

    /// One demix segment: STFT both channels, zero the first three bins, run the
    /// ONNX graph on the `(1, 4, dim_f, dim_t)` spectrum, and iSTFT back.
    fn run_model(&self, chunk: &[f32]) -> Result<Vec<f32>, Error> {
        let p = &self.profile;
        let chunk_size = p.chunk_size();

        let mut spectrum = vec![0f32; 4 * p.dim_f * p.dim_t];
        for ch in 0..2 {
            let samples = &chunk[ch * chunk_size..(ch + 1) * chunk_size];
            let spec = dsp::stft(samples, p.n_fft, p.hop_length, p.n_fft)?;
            if spec.frames != p.dim_t || spec.bins < p.dim_f {
                return Err(MdxError::InvalidProfile(format!(
                    "unexpected STFT shape: {} frames x {} bins (expected {} frames, >= {} bins)",
                    spec.frames, spec.bins, p.dim_t, p.dim_f
                ))
                .into());
            }
            for bin in 0..p.dim_f {
                for frame in 0..p.dim_t {
                    let src = frame * spec.bins + bin;
                    let (re, im) = if bin < 3 {
                        (0.0, 0.0)
                    } else {
                        (spec.re[src], spec.im[src])
                    };
                    spectrum[(2 * ch * p.dim_f + bin) * p.dim_t + frame] = re;
                    spectrum[((2 * ch + 1) * p.dim_f + bin) * p.dim_t + frame] = im;
                }
            }
        }

        let input = tract_ndarray::Array4::from_shape_vec((1, 4, p.dim_f, p.dim_t), spectrum)
            .map_err(infer_err)?;
        let outputs = self
            .plan
            .run(tvec![Tensor::from(input).into()])
            .map_err(infer_err)?;
        if outputs.is_empty() {
            return Err(MdxError::Inference("model returned no outputs".into()).into());
        }
        let view = outputs[0].to_plain_array_view::<f32>().map_err(infer_err)?;
        let predicted: Vec<f32> = view.iter().copied().collect();
        if predicted.len() != 4 * p.dim_f * p.dim_t {
            return Err(MdxError::Inference(format!(
                "unexpected model output length {} (expected {})",
                predicted.len(),
                4 * p.dim_f * p.dim_t
            ))
            .into());
        }

        let mut out = vec![0f32; 2 * chunk_size];
        for ch in 0..2 {
            let mut spec = dsp::StftResult {
                re: vec![0f32; p.dim_t * (p.n_fft / 2 + 1)],
                im: vec![0f32; p.dim_t * (p.n_fft / 2 + 1)],
                frames: p.dim_t,
                bins: p.n_fft / 2 + 1,
            };
            for bin in 0..p.dim_f {
                for frame in 0..p.dim_t {
                    let dst = frame * spec.bins + bin;
                    spec.re[dst] = predicted[((2 * ch) * p.dim_f + bin) * p.dim_t + frame];
                    spec.im[dst] = predicted[((2 * ch + 1) * p.dim_f + bin) * p.dim_t + frame];
                }
            }
            let wave = dsp::istft(&spec, p.n_fft, p.hop_length, p.n_fft, chunk_size)?;
            out[ch * chunk_size..(ch + 1) * chunk_size].copy_from_slice(&wave);
        }
        Ok(out)
    }
}

/// Coerce an input buffer to planar stereo at `sample_rate`. Mono is duplicated;
/// any other sample rate or channel count is rejected rather than resampled, so
/// the numerical comparison stays on the profile's grid.
fn planar_stereo(input: &AudioBuffer, sample_rate: usize) -> Result<Vec<f32>, MdxError> {
    if input.sample_rate as usize != sample_rate {
        return Err(MdxError::UnsupportedInput(format!(
            "sample rate {} Hz is not the MDX profile rate {} Hz",
            input.sample_rate, sample_rate
        )));
    }
    match input.channels {
        1 => {
            let mut v = Vec::with_capacity(input.planar.len() * 2);
            v.extend_from_slice(&input.planar);
            v.extend_from_slice(&input.planar);
            Ok(v)
        }
        2 => Ok(input.planar.clone()),
        other => Err(MdxError::UnsupportedInput(format!(
            "{} channels (stereo or mono required)",
            other
        ))),
    }
}

/// Symmetric Hann window, matching `numpy.hanning(len)` (endpoints zero).
fn hann_window_symmetric(len: usize) -> Vec<f32> {
    if len == 0 {
        return Vec::new();
    }
    if len == 1 {
        return vec![1.0];
    }
    let denom = (len - 1) as f32;
    (0..len)
        .map(|i| 0.5 - 0.5 * (2.0 * std::f32::consts::PI * i as f32 / denom).cos())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinned_profile_is_self_consistent() {
        let p = uvr_mdx_net_inst_hq_3();
        p.validate().unwrap();
        assert_eq!(p.model_file, "UVR-MDX-NET-Inst_HQ_3.onnx");
        assert_eq!(p.model_sha256.len(), 64);
        assert_eq!(p.primary_stem, "Instrumental");
        assert_eq!(p.secondary_stem, "Vocals");
    }

    #[test]
    fn geometry_matches_audio_separator() {
        let p = uvr_mdx_net_inst_hq_3();
        assert_eq!(p.trim(), 3072);
        assert_eq!(p.chunk_size(), 261_120);
        assert_eq!(p.gen_size(), 254_976);
        assert_eq!(p.step(), 195_840);
        assert_eq!(p.segment_frames(), 256);
        assert_eq!(p.dim_f, 3072);
        assert_eq!(p.n_fft / 2 + 1, 3073);
    }

    #[test]
    fn invalid_pin_is_rejected_by_validation() {
        let mut p = uvr_mdx_net_inst_hq_3();
        p.model_sha256 = "not-a-digest";
        assert!(matches!(p.validate(), Err(MdxError::InvalidProfile(_))));

        let mut p = uvr_mdx_net_inst_hq_3();
        p.dim_t = 128;
        assert!(matches!(p.validate(), Err(MdxError::InvalidProfile(_))));
    }

    #[test]
    fn checksum_mismatch_is_rejected() {
        let dir = std::env::temp_dir().join(format!(
            "candle-roformer-mdx-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("asset.bin");
        std::fs::write(&path, b"not the published checkpoint").unwrap();

        let err = verify_sha256(&path, uvr_mdx_net_inst_hq_3().model_sha256).unwrap_err();
        match err {
            MdxError::ChecksumMismatch { expected, actual } => {
                assert_eq!(actual.len(), 64);
                assert_eq!(expected, uvr_mdx_net_inst_hq_3().model_sha256);
            }
            other => panic!("unexpected error: {:?}", other),
        }

        let actual = sha256_hex(&path).unwrap();
        verify_sha256(&path, &actual).unwrap();
        verify_sha256(&path, &actual.to_uppercase()).unwrap();

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn missing_asset_is_rejected_before_inference() {
        let err = uvr_mdx_net_inst_hq_3()
            .resolve_verified_asset(std::env::temp_dir().join("candle-roformer-mdx-absent-dir"))
            .unwrap_err();
        assert!(matches!(err, MdxError::MissingAsset(_)));
    }

    #[test]
    fn symmetric_hann_matches_numpy_shape() {
        let w = hann_window_symmetric(8);
        assert_eq!(w.len(), 8);
        assert!(w[0].abs() < 1e-7);
        assert!(w[7].abs() < 1e-7);
        assert!((w[3] - 0.950_484).abs() < 1e-5);
        assert!((w[4] - 0.950_484).abs() < 1e-5);
        assert!((w[2] - 0.611_260).abs() < 1e-5);
        assert!((w[1] - 0.188_255).abs() < 1e-5);
        assert_eq!(hann_window_symmetric(1), vec![1.0]);
        assert!(hann_window_symmetric(0).is_empty());
    }

    #[test]
    fn stft_of_one_segment_has_model_shape() {
        let p = uvr_mdx_net_inst_hq_3();
        let chunk: Vec<f32> = (0..p.chunk_size())
            .map(|i| (i as f32 * 0.01).sin() * 0.5)
            .collect();
        let spec = dsp::stft(&chunk, p.n_fft, p.hop_length, p.n_fft).unwrap();
        assert_eq!(spec.frames, p.dim_t);
        assert_eq!(spec.bins, p.n_fft / 2 + 1);
        assert!(spec.re.iter().all(|x| x.is_finite()));
        assert!(spec.im.iter().all(|x| x.is_finite()));
    }

    #[test]
    fn planar_stereo_coerces_mono_and_rejects_other_rates() {
        let mono = AudioBuffer {
            sample_rate: 44100,
            channels: 1,
            planar: vec![0.1, -0.2, 0.3],
        };
        let stereo = planar_stereo(&mono, 44100).unwrap();
        assert_eq!(stereo.len(), 6);
        assert_eq!(&stereo[..3], &mono.planar);
        assert_eq!(&stereo[3..], &mono.planar);

        let wrong_rate = AudioBuffer {
            sample_rate: 48000,
            ..mono.clone()
        };
        assert!(matches!(
            planar_stereo(&wrong_rate, 44100),
            Err(MdxError::UnsupportedInput(_))
        ));

        let surround = AudioBuffer {
            sample_rate: 44100,
            channels: 4,
            planar: vec![0.0; 8],
        };
        assert!(matches!(
            planar_stereo(&surround, 44100),
            Err(MdxError::UnsupportedInput(_))
        ));
    }

    #[test]
    fn inference_config_for_family_is_mdx() {
        let cfg = crate::config::InferenceConfig::for_family(crate::config::ModelFamily::MdxNet);
        assert_eq!(cfg, crate::config::InferenceConfig::mdx());
        assert_eq!(cfg.segment_size, Some(256));
        assert_eq!(cfg.sample_rate, 44100);
        cfg.validate().unwrap();
    }
}
