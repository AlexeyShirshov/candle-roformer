//! VR Arch (UVR) inference pipeline.
//!
//! Reproduces `audio-separator` 0.48.0 `VRSeparator` for the `4band_v2`
//! parameter profile: four-band analysis STFT, spectrogram combination with the
//! multi-band pre-filter, ROI-windowed mask inference through
//! [`CascadedAsppNet`], aggression adjustment, and band-wise synthesis with
//! `libsamplerate` (`sinc_fastest`) resampling.

use std::path::Path;

use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;

use crate::dsp::{istft, stft_zeros, StftResult};
use crate::error::VrError;
use crate::model::vr_arch::CascadedAsppNet;
use crate::model::vr_resample::{resample_poly, resample_sinc_fastest};

/// One frequency band of a VR model.
#[derive(Debug, Clone, Copy)]
pub struct VrBand {
    pub sr: usize,
    pub hl: usize,
    pub n_fft: usize,
    pub crop_start: usize,
    pub crop_stop: usize,
    pub lpf: Option<(usize, usize)>,
    pub hpf: Option<(usize, usize)>,
}

impl VrBand {
    fn bins(&self) -> usize {
        self.n_fft / 2 + 1
    }

    fn crop_len(&self) -> usize {
        self.crop_stop - self.crop_start
    }
}

/// The pinned `4band_v2` model parameters plus inference knobs.
#[derive(Debug, Clone)]
pub struct VrProfile {
    pub bins: usize,
    pub bands: [VrBand; 4],
    pub pre_filter_start: usize,
    pub pre_filter_stop: usize,
    pub window_size: usize,
    pub offset: usize,
    /// `0.05` for `aggression = 5` in the reference CLI.
    pub aggression: f64,
    pub split_bin: usize,
    pub nn_arch: usize,
    pub sample_rate: usize,
}

impl Default for VrProfile {
    fn default() -> Self {
        Self::four_band_v2()
    }
}

impl VrProfile {
    /// Parameters of `modelparams/4band_v2.json` (the published `2_HP-UVR` family).
    pub fn four_band_v2() -> Self {
        Self {
            bins: 672,
            bands: [
                VrBand {
                    sr: 7350,
                    hl: 80,
                    n_fft: 640,
                    crop_start: 0,
                    crop_stop: 85,
                    lpf: Some((25, 53)),
                    hpf: None,
                },
                VrBand {
                    sr: 7350,
                    hl: 80,
                    n_fft: 320,
                    crop_start: 4,
                    crop_stop: 87,
                    lpf: Some((31, 62)),
                    hpf: Some((25, 12)),
                },
                VrBand {
                    sr: 14700,
                    hl: 160,
                    n_fft: 512,
                    crop_start: 17,
                    crop_stop: 216,
                    lpf: Some((139, 210)),
                    hpf: Some((48, 24)),
                },
                VrBand {
                    sr: 44100,
                    hl: 480,
                    n_fft: 960,
                    crop_start: 78,
                    crop_stop: 383,
                    lpf: None,
                    hpf: Some((130, 86)),
                },
            ],
            pre_filter_start: 668,
            pre_filter_stop: 672,
            window_size: 512,
            offset: 128,
            aggression: 0.05,
            split_bin: 85,
            nn_arch: 123812,
            sample_rate: 44100,
        }
    }

    /// Validate that a [`crate::config::SeparatorConfig`] describes this profile.
    pub fn from_separator_config(config: &crate::config::SeparatorConfig) -> Result<Self, VrError> {
        if config.sample_rate != 44100 {
            return Err(VrError::InvalidProfile(format!(
                "VR 4band_v2 requires a 44100 Hz profile, got {}",
                config.sample_rate
            )));
        }
        Ok(Self::four_band_v2())
    }

    fn roi_size(&self) -> usize {
        let roi = self.window_size.saturating_sub(2 * self.offset);
        if roi == 0 {
            self.window_size
        } else {
            roi
        }
    }
}

/// A complex spectrogram of one channel, frame-major (`frames * bins`).
#[derive(Debug, Clone)]
struct Spec {
    bins: usize,
    frames: usize,
    re: Vec<f32>,
    im: Vec<f32>,
}

impl Spec {
    fn zeros(bins: usize, frames: usize) -> Self {
        Self {
            bins,
            frames,
            re: vec![0.0; bins * frames],
            im: vec![0.0; bins * frames],
        }
    }

    fn at(&self, bin: usize, frame: usize) -> usize {
        frame * self.bins + bin
    }
}

fn to_spec(result: &StftResult) -> Spec {
    Spec {
        bins: result.bins,
        frames: result.frames,
        re: result.re.clone(),
        im: result.im.clone(),
    }
}

fn stft_mono(x: &[f32], n_fft: usize, hop: usize) -> Result<Spec, VrError> {
    let result = stft_zeros(x, n_fft, hop, n_fft)
        .map_err(|err| VrError::Inference(format!("STFT failed: {err}")))?;
    Ok(to_spec(&result))
}

fn istft_mono(spec: &Spec, n_fft: usize, hop: usize) -> Result<Vec<f32>, VrError> {
    let length = spec.frames.saturating_sub(1) * hop;
    let result = StftResult {
        re: spec.re.clone(),
        im: spec.im.clone(),
        frames: spec.frames,
        bins: spec.bins,
    };
    istft(&result, n_fft, hop, n_fft, length)
        .map_err(|err| VrError::Inference(format!("iSTFT failed: {err}")))
}

/// Port of `spec_utils.fft_lp_filter` (in place, per channel).
fn lp_filter(spec: &mut Spec, bin_start: usize, bin_stop: usize) {
    let mut g = 1.0f32;
    let step = 1.0f32 / (bin_stop - bin_start) as f32;
    for bin in bin_start..bin_stop {
        g -= step;
        for frame in 0..spec.frames {
            let idx = spec.at(bin, frame);
            spec.re[idx] *= g;
            spec.im[idx] *= g;
        }
    }
    for bin in bin_stop..spec.bins {
        for frame in 0..spec.frames {
            let idx = spec.at(bin, frame);
            spec.re[idx] = 0.0;
            spec.im[idx] = 0.0;
        }
    }
}

/// Port of `spec_utils.fft_hp_filter` (in place, per channel).
fn hp_filter(spec: &mut Spec, bin_start: usize, bin_stop: usize) {
    let mut g = 1.0f32;
    let step = 1.0f32 / (bin_start - bin_stop) as f32;
    let mut bin = bin_start;
    while bin > bin_stop {
        g -= step;
        for frame in 0..spec.frames {
            let idx = spec.at(bin, frame);
            spec.re[idx] *= g;
            spec.im[idx] *= g;
        }
        bin -= 1;
    }
    for bin in 0..=bin_stop {
        for frame in 0..spec.frames {
            let idx = spec.at(bin, frame);
            spec.re[idx] = 0.0;
            spec.im[idx] = 0.0;
        }
    }
}

fn pre_filter(spec: &mut Spec, profile: &VrProfile) {
    let mut gp = 1.0f64;
    for bin in (profile.pre_filter_start + 1)..profile.pre_filter_stop {
        let g = 10f64.powf(-((bin - profile.pre_filter_start) as f64) * (3.5 - gp) / 20.0);
        gp = g;
        let g = g as f32;
        for frame in 0..spec.frames {
            let idx = spec.at(bin, frame);
            spec.re[idx] *= g;
            spec.im[idx] *= g;
        }
    }
}

/// Combined analysis result for one channel.
struct Analysis {
    spec: Spec,
    mag: Vec<f32>,
    phase: Vec<f32>,
}

fn analyse_channel(x: &[f32], profile: &VrProfile) -> Result<Analysis, VrError> {
    let b4 = profile.bands[3];
    let b3 = profile.bands[2];
    let b2 = profile.bands[1];
    let b1 = profile.bands[0];

    let s4 = stft_mono(x, b4.n_fft, b4.hl)?;
    let x3 = resample_poly(x, 1, 3);
    let s3 = stft_mono(&x3, b3.n_fft, b3.hl)?;
    let x2 = resample_poly(&x3, 1, 2);
    let s2 = stft_mono(&x2, b2.n_fft, b2.hl)?;
    let s1 = stft_mono(&x2, b1.n_fft, b1.hl)?;

    let frames = s1.frames.min(s2.frames).min(s3.frames).min(s4.frames);
    let mut combined = Spec::zeros(profile.bins + 1, frames);
    let mut offset = 0usize;
    for (band, spec) in [(b1, &s1), (b2, &s2), (b3, &s3), (b4, &s4)] {
        let len = band.crop_len();
        for bin in 0..len {
            for frame in 0..frames {
                let src = spec.at(band.crop_start + bin, frame);
                let dst = combined.at(offset + bin, frame);
                combined.re[dst] = spec.re[src];
                combined.im[dst] = spec.im[src];
            }
        }
        offset += len;
    }
    pre_filter(&mut combined, profile);

    let mut mag = vec![0.0f32; combined.re.len()];
    let mut phase = vec![0.0f32; combined.re.len()];
    for i in 0..combined.re.len() {
        let re = combined.re[i];
        let im = combined.im[i];
        mag[i] = (re * re + im * im).sqrt();
        phase[i] = im.atan2(re);
    }
    Ok(Analysis {
        spec: combined,
        mag,
        phase,
    })
}

/// `spec_utils.make_padding`
fn make_padding(width: usize, window: usize, offset: usize) -> (usize, usize, usize) {
    let left = offset;
    let mut roi_size = window.saturating_sub(offset * 2);
    if roi_size == 0 {
        roi_size = window;
    }
    let right = roi_size - (width % roi_size) + left;
    (left, right, roi_size)
}

/// `spec_utils.adjust_aggr` for the primary (non-accompaniment) stem.
fn adjust_aggr(
    mask: &mut [f32],
    bins: usize,
    frames: usize,
    profile: &VrProfile,
    split_bin: usize,
) {
    let aggr = profile.aggression * 2.0;
    if aggr == 0.0 {
        return;
    }
    // `CommonSeparator.NON_ACCOM_STEMS` is
    // `('Vocals', 'Other', 'Bass', 'Drums', ...)`; the primary stem of
    // `2_HP-UVR` is "Instrumental", so the reference keeps `aggr` unflipped.
    let aggr = aggr as f32;
    let low_exp = 1.0 + aggr / 3.0;
    let high_exp = 1.0 + aggr;
    for ch in 0..2 {
        for bin in 0..bins {
            let exp = if bin < split_bin { low_exp } else { high_exp };
            for frame in 0..frames {
                let idx = ch * bins * frames + bin * frames + frame;
                mask[idx] = mask[idx].powf(exp);
            }
        }
    }
}

/// VR Arch separator: owns the converted network and the pinned profile.
pub struct VrSeparator {
    net: CascadedAsppNet,
    profile: VrProfile,
    device: Device,
}

impl std::fmt::Debug for VrSeparator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VrSeparator")
            .field("profile", &self.profile)
            .finish_non_exhaustive()
    }
}

impl VrSeparator {
    /// Load a converted (safetensors) VR checkpoint with the pinned profile.
    pub fn load(path: &Path, profile: VrProfile) -> Result<Self, VrError> {
        let device = Device::Cpu;
        if !path.exists() {
            return Err(VrError::MissingAsset(path.display().to_string()));
        }
        // Validate the artifact is a real CascadedASPPNet state dict before
        // building the module: required prefixes must be present.
        // SAFETY: the artifact is opened read-only for validation; the same file
        // is memory-mapped below for the weights.
        let mapped = unsafe { candle_core::safetensors::MmapedSafetensors::new(path) }
            .map_err(|err| VrError::Inference(format!("cannot open {}: {err}", path.display())))?;
        let names: Vec<String> = mapped.tensors().into_iter().map(|(name, _)| name).collect();
        for required in [
            "stg1_low_band_net.enc1.conv1.conv.0.weight",
            "stg2_bridge.conv.0.weight",
            "stg3_full_band_net.aspp.bottleneck.0.conv.0.weight",
            "out.weight",
        ] {
            if !names.iter().any(|name| name == required) {
                return Err(VrError::InvalidProfile(format!(
                    "checkpoint lacks `{required}`; not a CascadedASPPNet artifact"
                )));
            }
        }

        // SAFETY: the file is opened read-only and validated above.
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[path.to_path_buf()], DType::F32, &device)
                .map_err(|err| VrError::Inference(format!("cannot memory-map weights: {err}")))?
        };
        let net = CascadedAsppNet::load(vb, profile.bins * 2, profile.nn_arch)
            .map_err(|err| VrError::Inference(format!("network construction failed: {err}")))?;
        Ok(Self {
            net,
            profile,
            device,
        })
    }

    pub fn profile(&self) -> &VrProfile {
        &self.profile
    }

    /// Diagnostic: combined complex analysis spectrogram for channel 0.
    pub fn debug_combined(&self, x: &[f32]) -> Result<(Vec<f32>, Vec<f32>, usize, usize), VrError> {
        let a = analyse_channel(x, &self.profile)?;
        Ok((a.spec.re, a.spec.im, a.spec.bins, a.spec.frames))
    }

    /// Diagnostic: per-band analysis spectrograms `[re, im]` with the band's
    /// own bin count, channel 0 only.
    pub fn debug_band_spec(
        &self,
        x: &[f32],
        band: usize,
    ) -> Result<(Vec<f32>, Vec<f32>, usize, usize), VrError> {
        let b = self.profile.bands[band];
        let s = match band {
            0 => {
                let x3 = resample_poly(x, 1, 3);
                let x2 = resample_poly(&x3, 1, 2);
                stft_mono(&x2, b.n_fft, b.hl)?
            }
            1 => {
                let x3 = resample_poly(x, 1, 3);
                let x2 = resample_poly(&x3, 1, 2);
                stft_mono(&x2, b.n_fft, b.hl)?
            }
            2 => {
                let x3 = resample_poly(x, 1, 3);
                stft_mono(&x3, b.n_fft, b.hl)?
            }
            _ => stft_mono(x, b.n_fft, b.hl)?,
        };
        Ok((s.re, s.im, s.bins, s.frames))
    }

    /// Diagnostic: synthesis with a unit mask (round-trip check).
    pub fn debug_full_mix(&self, left: &[f32], right: &[f32]) -> Result<Vec<f32>, VrError> {
        let a_left = analyse_channel(left, &self.profile)?;
        let a_right = analyse_channel(right, &self.profile)?;
        let frames = a_left.spec.frames.min(a_right.spec.frames);
        let mask = vec![1.0f32; 2 * self.profile.bins * frames];
        let p_l = self.synthesise_channel(&a_left.spec, &mask, 0)?;
        let p_r = self.synthesise_channel(&a_right.spec, &mask, 1)?;
        Ok(concat_stereo(&p_l, &p_r))
    }

    /// Diagnostic: per-band masked spectra (channel 0, primary stem) before the
    /// HP/LP filters, frame-major.
    pub fn debug_band_specs(
        &self,
        left: &[f32],
        right: &[f32],
    ) -> Result<Vec<(Vec<f32>, Vec<f32>, usize)>, VrError> {
        let a_left = analyse_channel(left, &self.profile)?;
        let a_right = analyse_channel(right, &self.profile)?;
        let frames = a_left.spec.frames.min(a_right.spec.frames);
        let bins = self.profile.bins;
        let mut mag = vec![0.0f32; 2 * bins * frames];
        for (ch, analysis) in [&a_left, &a_right].iter().enumerate() {
            for bin in 0..bins {
                for frame in 0..frames {
                    mag[ch * bins * frames + bin * frames + frame] =
                        analysis.mag[frame * analysis.spec.bins + bin];
                }
            }
        }
        let mut mask = self.run_mask(&mag, bins, frames)?;
        adjust_aggr(
            &mut mask,
            bins,
            frames,
            &self.profile,
            self.profile.split_bin,
        );
        let spec = &a_left.spec;
        let mut offset = 0usize;
        let mut out = Vec::new();
        for band in self.profile.bands.iter() {
            let mut band_spec = Spec::zeros(band.bins(), frames);
            for bin in 0..band.crop_len() {
                for frame in 0..frames {
                    let idx = frame * spec.bins + (offset + bin);
                    let m = mask[(offset + bin) * frames + frame];
                    let dst = band_spec.at(band.crop_start + bin, frame);
                    band_spec.re[dst] = spec.re[idx] * m;
                    band_spec.im[dst] = spec.im[idx] * m;
                }
            }
            offset += band.crop_len();
            out.push((band_spec.re, band_spec.im, band.bins()));
        }
        Ok(out)
    }

    /// Diagnostic: per-band synthesis waveforms (channel 0, primary stem),
    /// before the resampling chain.
    pub fn debug_band_waves(&self, left: &[f32], right: &[f32]) -> Result<Vec<Vec<f32>>, VrError> {
        let a_left = analyse_channel(left, &self.profile)?;
        let a_right = analyse_channel(right, &self.profile)?;
        let frames = a_left.spec.frames.min(a_right.spec.frames);
        let bins = self.profile.bins;
        let mut mag = vec![0.0f32; 2 * bins * frames];
        for (ch, analysis) in [&a_left, &a_right].iter().enumerate() {
            for bin in 0..bins {
                for frame in 0..frames {
                    mag[ch * bins * frames + bin * frames + frame] =
                        analysis.mag[frame * analysis.spec.bins + bin];
                }
            }
        }
        let mut mask = self.run_mask(&mag, bins, frames)?;
        adjust_aggr(
            &mut mask,
            bins,
            frames,
            &self.profile,
            self.profile.split_bin,
        );
        // The reference stage ladder records the *last* synthesis pass, which is
        // the secondary stem: use 1 - mask here.
        for value in mask.iter_mut() {
            *value = 1.0 - *value;
        }

        let spec = &a_left.spec;
        let mut offset = 0usize;
        let mut waves = Vec::new();
        for band in self.profile.bands.iter() {
            let mut band_spec = Spec::zeros(band.bins(), frames);
            for bin in 0..band.crop_len() {
                for frame in 0..frames {
                    let idx = frame * spec.bins + (offset + bin);
                    let m = mask[(offset + bin) * frames + frame];
                    let dst = band_spec.at(band.crop_start + bin, frame);
                    band_spec.re[dst] = spec.re[idx] * m;
                    band_spec.im[dst] = spec.im[idx] * m;
                }
            }
            offset += band.crop_len();
            if let Some((start, stop)) = band.hpf {
                // cmb_spectrogram_to_wave passes hpf_stop - 1 to the filter.
                hp_filter(&mut band_spec, start, stop - 1);
            }
            if let Some((start, stop)) = band.lpf {
                lp_filter(&mut band_spec, start, stop);
            }
            waves.push(istft_mono(&band_spec, band.n_fft, band.hl)?);
        }
        Ok(waves)
    }

    /// Diagnostic: masked complex spectrogram of the primary stem, channel 0,
    /// frame-major (`frame * bins + bin`).
    pub fn debug_yspec(
        &self,
        left: &[f32],
        right: &[f32],
    ) -> Result<(Vec<f32>, Vec<f32>, usize, usize), VrError> {
        let a_left = analyse_channel(left, &self.profile)?;
        let a_right = analyse_channel(right, &self.profile)?;
        let frames = a_left.spec.frames.min(a_right.spec.frames);
        let bins = self.profile.bins;
        let mut mag = vec![0.0f32; 2 * bins * frames];
        for (ch, analysis) in [&a_left, &a_right].iter().enumerate() {
            for bin in 0..bins {
                for frame in 0..frames {
                    mag[ch * bins * frames + bin * frames + frame] =
                        analysis.mag[frame * analysis.spec.bins + bin];
                }
            }
        }
        let mut mask = self.run_mask(&mag, bins, frames)?;
        adjust_aggr(
            &mut mask,
            bins,
            frames,
            &self.profile,
            self.profile.split_bin,
        );
        let spec = &a_left.spec;
        let mut re = vec![0.0f32; spec.bins * frames];
        let mut im = vec![0.0f32; spec.bins * frames];
        for bin in 0..bins {
            for frame in 0..frames {
                let idx = frame * spec.bins + bin;
                let m = mask[bin * frames + frame];
                re[idx] = spec.re[idx] * m;
                im[idx] = spec.im[idx] * m;
            }
        }
        Ok((re, im, spec.bins, frames))
    }

    /// Diagnostic: mask before and after the aggression adjustment.
    pub fn debug_mask(
        &self,
        left: &[f32],
        right: &[f32],
    ) -> Result<(Vec<f32>, Vec<f32>, usize, usize), VrError> {
        let a_left = analyse_channel(left, &self.profile)?;
        let a_right = analyse_channel(right, &self.profile)?;
        let frames = a_left.spec.frames.min(a_right.spec.frames);
        let bins = self.profile.bins;
        let mut mag = vec![0.0f32; 2 * bins * frames];
        for (ch, analysis) in [&a_left, &a_right].iter().enumerate() {
            for bin in 0..bins {
                for frame in 0..frames {
                    mag[ch * bins * frames + bin * frames + frame] =
                        analysis.mag[frame * analysis.spec.bins + bin];
                }
            }
        }
        let raw = self.run_mask(&mag, bins, frames)?;
        let mut adjusted = raw.clone();
        adjust_aggr(
            &mut adjusted,
            bins,
            frames,
            &self.profile,
            self.profile.split_bin,
        );
        Ok((raw, adjusted, bins, frames))
    }

    /// Diagnostic: combined analysis magnitude, `[channel][bin][frame]` with
    /// `profile.bins` bins, for comparing against the reference stage ladder.
    pub fn debug_mag(
        &self,
        left: &[f32],
        right: &[f32],
    ) -> Result<(Vec<f32>, usize, usize), VrError> {
        let a_left = analyse_channel(left, &self.profile)?;
        let a_right = analyse_channel(right, &self.profile)?;
        let frames = a_left.spec.frames.min(a_right.spec.frames);
        let bins = self.profile.bins;
        let mut mag = vec![0.0f32; 2 * bins * frames];
        for (ch, analysis) in [&a_left, &a_right].iter().enumerate() {
            for bin in 0..bins {
                for frame in 0..frames {
                    mag[ch * bins * frames + bin * frames + frame] =
                        analysis.mag[frame * analysis.spec.bins + bin];
                }
            }
        }
        Ok((mag, bins, frames))
    }

    fn run_mask(&self, mag: &[f32], bins: usize, frames: usize) -> Result<Vec<f32>, VrError> {
        let profile = &self.profile;
        let (pad_l, pad_r, roi) = make_padding(frames, profile.window_size, profile.offset);
        let padded = frames + pad_l + pad_r;
        // Padded magnitude, divided by its maximum (reference behaviour).
        let mut mag_pad = vec![0.0f32; 2 * bins * padded];
        for ch in 0..2 {
            for bin in 0..bins {
                for frame in 0..frames {
                    mag_pad[ch * bins * padded + bin * padded + pad_l + frame] =
                        mag[ch * bins * frames + bin * frames + frame];
                }
            }
        }
        let max = mag_pad.iter().fold(0.0f32, |a, b| a.max(b.abs()));
        if max > 0.0 {
            for v in mag_pad.iter_mut() {
                *v /= max;
            }
        }

        let patches = (padded - 2 * profile.offset) / roi;
        let mut mask = vec![0.0f32; 2 * bins * (patches * roi)];
        let window = profile.window_size;
        for patch in 0..patches {
            let start = patch * roi;
            let mut input = vec![0.0f32; 2 * bins * window];
            for ch in 0..2 {
                for bin in 0..bins {
                    for frame in 0..window {
                        input[ch * bins * window + bin * window + frame] =
                            mag_pad[ch * bins * padded + bin * padded + start + frame];
                    }
                }
            }
            let tensor = Tensor::from_vec(input, (1, 2, bins, window), &self.device)
                .map_err(|err| VrError::Inference(format!("input tensor: {err}")))?;
            let out = self
                .net
                .predict_mask(&tensor)
                .map_err(|err| VrError::Inference(format!("network forward failed: {err}")))?;
            let out = out
                .squeeze(0)
                .and_then(|t| t.to_dtype(DType::F32))
                .and_then(|t| t.flatten_all())
                .and_then(|t| t.to_vec1::<f32>())
                .map_err(|err| VrError::Inference(format!("mask extraction failed: {err}")))?;
            let out_bins = bins + 1;
            for ch in 0..2 {
                // Row `bins` (the replicate-padded bin) is dropped: the
                // combined spectrogram has no energy there, exactly as in the
                // reference. Keep the source stride at `out_bins`.
                for bin in 0..bins {
                    for frame in 0..roi {
                        let src = ch * out_bins * roi + bin * roi + frame;
                        let dst =
                            ch * bins * (patches * roi) + bin * (patches * roi) + start + frame;
                        mask[dst] = out[src];
                    }
                }
            }
        }
        // Trim to the analysis width.
        let mut trimmed = vec![0.0f32; 2 * bins * frames];
        for ch in 0..2 {
            for bin in 0..bins {
                for frame in 0..frames {
                    trimmed[ch * bins * frames + bin * frames + frame] =
                        mask[ch * bins * (patches * roi) + bin * (patches * roi) + frame];
                }
            }
        }
        Ok(trimmed)
    }

    fn synthesise_channel(
        &self,
        spec: &Spec,
        stem_mask: &[f32],
        ch: usize,
    ) -> Result<Vec<f32>, VrError> {
        let profile = &self.profile;
        let frames = spec.frames;
        let spec_bins = spec.bins;
        let mask_bins = profile.bins;
        let mut wave: Vec<f32> = Vec::new();
        let mut offset = 0usize;
        for (index, band) in profile.bands.iter().enumerate() {
            let mut band_spec = Spec::zeros(band.bins(), frames);
            for bin in 0..band.crop_len() {
                for frame in 0..frames {
                    let spec_idx = frame * spec_bins + (offset + bin);
                    let mask_idx = ch * mask_bins * frames + (offset + bin) * frames + frame;
                    let dst = band_spec.at(band.crop_start + bin, frame);
                    let m = stem_mask[mask_idx];
                    band_spec.re[dst] = spec.re[spec_idx] * m;
                    band_spec.im[dst] = spec.im[spec_idx] * m;
                }
            }
            offset += band.crop_len();

            if let Some((start, stop)) = band.hpf {
                // cmb_spectrogram_to_wave passes hpf_stop - 1 to the filter.
                hp_filter(&mut band_spec, start, stop - 1);
            }
            if let Some((start, stop)) = band.lpf {
                lp_filter(&mut band_spec, start, stop);
            }

            let w = istft_mono(&band_spec, band.n_fft, band.hl)?;
            let summed = add_waves(&wave, &w);
            // The reference resamples every band except the highest one up to
            // the next band's sample rate before summing.
            wave = if index + 1 < profile.bands.len() {
                let next = profile.bands[index + 1].sr;
                resample_sinc_fastest(&summed, band.sr as u32, next as u32)
                    .map_err(VrError::Inference)?
            } else {
                summed
            };
        }
        Ok(wave)
    }

    /// Separate a stereo 44.1 kHz signal into (primary, secondary) waveforms.
    ///
    /// The primary stem of the VR models is the non-accompaniment stem
    /// ("Instrumental" for `2_HP-UVR`), which is what `adjust_aggr` expects.
    pub fn separate(&self, left: &[f32], right: &[f32]) -> Result<(Vec<f32>, Vec<f32>), VrError> {
        if left.len() != right.len() {
            return Err(VrError::UnsupportedInput(
                "left and right channels must have the same length".into(),
            ));
        }
        let profile = &self.profile;
        let a_left = analyse_channel(left, profile)?;
        let a_right = analyse_channel(right, profile)?;
        let frames = a_left.spec.frames.min(a_right.spec.frames);
        let bins = profile.bins;

        let mut mag = vec![0.0f32; 2 * bins * frames];
        for (ch, analysis) in [&a_left, &a_right].iter().enumerate() {
            for bin in 0..bins {
                for frame in 0..frames {
                    mag[ch * bins * frames + bin * frames + frame] =
                        analysis.mag[frame * analysis.spec.bins + bin];
                }
            }
        }
        let mut mask = self.run_mask(&mag, bins, frames)?;
        adjust_aggr(&mut mask, bins, frames, profile, profile.split_bin);
        let inverse: Vec<f32> = mask.iter().map(|m| 1.0 - *m).collect();

        let primary_l = self.synthesise_channel(&a_left.spec, &mask, 0)?;
        let primary_r = self.synthesise_channel(&a_right.spec, &mask, 1)?;
        let secondary_l = self.synthesise_channel(&a_left.spec, &inverse, 0)?;
        let secondary_r = self.synthesise_channel(&a_right.spec, &inverse, 1)?;
        Ok((
            concat_stereo(&primary_l, &primary_r),
            concat_stereo(&secondary_l, &secondary_r),
        ))
    }
}

fn concat_stereo(left: &[f32], right: &[f32]) -> Vec<f32> {
    let len = left.len().min(right.len());
    let mut out = Vec::with_capacity(len * 2);
    for i in 0..len {
        out.push(left[i]);
        out.push(right[i]);
    }
    out
}

fn add_waves(a: &[f32], b: &[f32]) -> Vec<f32> {
    let len = a.len().max(b.len());
    let mut out = vec![0.0f32; len];
    for (i, slot) in out.iter_mut().enumerate() {
        *slot = a.get(i).copied().unwrap_or(0.0) + b.get(i).copied().unwrap_or(0.0);
    }
    out
}
