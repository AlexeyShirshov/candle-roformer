use std::path::Path;

use crate::error::ConfigError;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SeparatorConfig {
    pub dim: usize,
    pub depth: usize,
    pub stereo: bool,
    pub num_stems: usize,
    /// Mel-band band count. MelBandRoformer only; BS-RoFormer configs carry
    /// `freqs_per_bands` instead (default 0 = absent).
    #[serde(default)]
    pub num_bands: usize,
    /// Explicit family selector. When absent the family is inferred from the
    /// config shape: `freqs_per_bands` present => BS-RoFormer, otherwise
    /// MelBandRoformer.
    #[serde(default)]
    pub family: Option<ModelFamily>,
    /// BS-RoFormer splits the STFT frequency axis into explicit consecutive
    /// band widths, e.g. `(2, 2, ..., 128, 129)`. The widths must sum to
    /// `dim_freqs_in`.
    #[serde(default)]
    pub freqs_per_bands: Option<Vec<usize>>,
    pub dim_head: usize,
    pub heads: usize,
    #[serde(default = "default_true")]
    pub flash_attn: bool,
    pub dim_freqs_in: usize,
    /// Model sample rate. UVR profiles usually carry it in the `audio:` block;
    /// it is resolved from there when the `model:` block omits it.
    #[serde(default)]
    pub sample_rate: usize,
    pub stft_n_fft: usize,
    pub stft_hop_length: usize,
    pub stft_win_length: usize,
    #[serde(default)]
    pub stft_normalized: bool,
    pub mask_estimator_depth: usize,
    #[serde(default)]
    pub use_torch_checkpoint: bool,
    pub time_transformer_depth: usize,
    pub freq_transformer_depth: usize,
    #[serde(default = "default_mlp_expansion")]
    pub mlp_expansion_factor: usize,
    #[serde(default)]
    pub linear_transformer_depth: usize,
    #[serde(default = "default_true")]
    pub norm_output: bool,
    #[serde(default)]
    pub skip_connection: bool,
    #[serde(default = "default_true")]
    pub zero_dc: bool,
}

fn default_true() -> bool {
    true
}

fn default_mlp_expansion() -> usize {
    4
}

impl SeparatorConfig {
    /// Parse a v7-style YAML profile (`model:` section).
    ///
    /// The upstream UVR model configs use `!!python/tuple` tags for the
    /// `freqs_per_bands` sequence; the tag carries no information a YAML
    /// sequence does not already have, so it is stripped before parsing.
    pub fn from_yaml_str(text: &str) -> Result<Self, ConfigError> {
        #[derive(Deserialize)]
        struct AudioSection {
            #[serde(default)]
            sample_rate: Option<usize>,
        }
        #[derive(Deserialize)]
        struct Root {
            model: SeparatorConfig,
            #[serde(default)]
            audio: Option<AudioSection>,
        }
        let text = text.replace("!!python/tuple", "");
        let root: Root =
            serde_yaml::from_str(&text).map_err(|e| ConfigError::Invalid(e.to_string()))?;
        let mut model = root.model;
        // UVR profiles keep the sample rate in `audio:`; the reference loader
        // copies it into the model profile. Default to 44.1 kHz (the only rate
        // the published RoFormer profiles use).
        if model.sample_rate == 0 {
            model.sample_rate = root.audio.and_then(|a| a.sample_rate).unwrap_or(44_100);
        }
        Ok(model)
    }

    /// Read and parse a v7-style YAML profile from disk.
    pub fn from_yaml_file(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::Invalid(format!("cannot read config: {}", e)))?;
        Self::from_yaml_str(&text)
    }

    /// Resolve the model family: an explicit `family:` key wins; otherwise the
    /// config shape decides (`freqs_per_bands` => BS-RoFormer, else MelBand).
    pub fn family(&self) -> ModelFamily {
        self.family.unwrap_or(if self.freqs_per_bands.is_some() {
            ModelFamily::BSRoformer
        } else {
            ModelFamily::MelBandRoformer
        })
    }

    /// The exact profile from `v7.yaml` (MelBandRoformer vocals).
    pub fn v7() -> Self {
        Self {
            dim: 256,
            depth: 12,
            stereo: true,
            num_stems: 1,
            num_bands: 60,
            family: None,
            freqs_per_bands: None,
            dim_head: 64,
            heads: 8,
            flash_attn: true,
            dim_freqs_in: 1025,
            sample_rate: 44100,
            stft_n_fft: 2048,
            stft_hop_length: 441,
            stft_win_length: 2048,
            stft_normalized: false,
            mask_estimator_depth: 2,
            use_torch_checkpoint: true,
            time_transformer_depth: 1,
            freq_transformer_depth: 1,
            mlp_expansion_factor: 4,
            linear_transformer_depth: 0,
            norm_output: true,
            skip_connection: false,
            zero_dc: true,
        }
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.dim == 0 {
            return Err(ConfigError::Invalid("dim must be > 0".into()));
        }
        if self.depth == 0 {
            return Err(ConfigError::Invalid("depth must be > 0".into()));
        }
        if self.num_stems == 0 {
            return Err(ConfigError::Invalid("num_stems must be > 0".into()));
        }
        match self.family() {
            ModelFamily::MelBandRoformer => {
                if self.num_bands == 0 {
                    return Err(ConfigError::Invalid(
                        "num_bands must be > 0 for mel_band_roformer".into(),
                    ));
                }
            }
            ModelFamily::BSRoformer => {
                let freqs = self.freqs_per_bands.as_ref().ok_or_else(|| {
                    ConfigError::Invalid("freqs_per_bands is required for bs_roformer".into())
                })?;
                if freqs.len() < 2 {
                    return Err(ConfigError::Invalid(
                        "freqs_per_bands must have at least 2 bands".into(),
                    ));
                }
                if freqs.iter().any(|f| *f == 0) {
                    return Err(ConfigError::Invalid(
                        "freqs_per_bands entries must be > 0".into(),
                    ));
                }
                let total: usize = freqs.iter().sum();
                if total != self.dim_freqs_in {
                    return Err(ConfigError::Invalid(format!(
                        "freqs_per_bands sums to {} but dim_freqs_in is {}",
                        total, self.dim_freqs_in
                    )));
                }
            }
        }
        if self.dim_head == 0 || self.heads == 0 {
            return Err(ConfigError::Invalid(
                "dim_head and heads must be > 0".into(),
            ));
        }
        if self.dim_freqs_in == 0 {
            return Err(ConfigError::Invalid("dim_freqs_in must be > 0".into()));
        }
        if self.sample_rate == 0 {
            return Err(ConfigError::Invalid("sample_rate must be > 0".into()));
        }
        if self.stft_n_fft == 0 || self.stft_n_fft % 2 != 0 {
            return Err(ConfigError::Invalid(
                "stft_n_fft must be a positive even number".into(),
            ));
        }
        if self.stft_hop_length == 0 {
            return Err(ConfigError::Invalid("stft_hop_length must be > 0".into()));
        }
        if self.stft_win_length == 0 || self.stft_win_length > self.stft_n_fft {
            return Err(ConfigError::Invalid(
                "stft_win_length must be in 1..=stft_n_fft".into(),
            ));
        }
        if self.dim_freqs_in != self.stft_n_fft / 2 + 1 {
            return Err(ConfigError::Invalid(format!(
                "dim_freqs_in ({}) must equal stft_n_fft/2 + 1 ({})",
                self.dim_freqs_in,
                self.stft_n_fft / 2 + 1
            )));
        }
        if self.stereo != true {
            // The v7 profile is stereo; keep the check explicit but allow other profiles?
        }
        Ok(())
    }
}

// ---------- Inference configuration (issue #9) ----------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum ModelFamily {
    #[serde(rename = "mel_band_roformer", alias = "melband", alias = "mel_band")]
    MelBandRoformer,
    #[serde(rename = "bs_roformer", alias = "bsroformer", alias = "bs-roformer")]
    BSRoformer,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(default)]
pub struct InferenceConfig {
    pub segment_size: Option<usize>,
    pub chunk_hops: usize,
    pub num_overlap: usize,
    pub step_size: Option<usize>,
    pub overlap_size: Option<usize>,
    pub fade_size: Option<usize>,
    pub batch_size: usize,
    pub pitch_shift: f32,
    pub denoise: bool,
    pub tta: bool,
    pub sample_rate: usize,
    #[serde(default = "default_normalization")]
    pub normalization: f32,
    #[serde(default = "default_amplification")]
    pub amplification: f32,
    #[serde(default = "default_invert_spect")]
    pub invert_spect: bool,
}

fn default_normalization() -> f32 {
    0.9
}
fn default_amplification() -> f32 {
    0.0
}
fn default_invert_spect() -> bool {
    false
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self::melband()
    }
}

impl InferenceConfig {
    pub fn melband() -> Self {
        Self {
            segment_size: None,
            chunk_hops: 800,
            num_overlap: 2,
            step_size: None,
            overlap_size: None,
            fade_size: None,
            batch_size: 1,
            pitch_shift: 0.0,
            denoise: false,
            tta: false,
            sample_rate: 44100,
            normalization: 0.9,
            amplification: 0.0,
            invert_spect: false,
        }
    }

    pub fn bsroformer() -> Self {
        // Same streaming granularity as the MelBand profile: 800 STFT hops per
        // chunk (352800 samples = 8 s at 44.1 kHz) and the reference's
        // 4-way overlap for BS-RoFormer profiles.
        Self {
            num_overlap: 4,
            ..Self::melband()
        }
    }

    pub fn for_family(family: ModelFamily) -> Self {
        match family {
            ModelFamily::MelBandRoformer => Self::melband(),
            ModelFamily::BSRoformer => Self::bsroformer(),
        }
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if let Some(s) = self.segment_size {
            if s == 0 {
                return Err(ConfigError::Invalid("segment must be > 0".into()));
            }
        }
        if self.segment_size.is_none() && self.chunk_hops == 0 {
            return Err(ConfigError::Invalid("chunk_hops must be > 0".into()));
        }
        if let Some(s) = self.step_size {
            if s == 0 {
                return Err(ConfigError::Invalid("step must be > 0".into()));
            }
        }
        if let Some(o) = self.overlap_size {
            if o == 0 {
                return Err(ConfigError::Invalid("overlap must be > 0".into()));
            }
        }
        if self.num_overlap == 0 {
            return Err(ConfigError::Invalid("overlap must be > 0".into()));
        }
        if let Some(f) = self.fade_size {
            if f == 0 {
                return Err(ConfigError::Invalid("fade must be > 0".into()));
            }
        }
        if let (Some(seg), Some(fade)) = (self.segment_size, self.fade_size) {
            if fade > seg {
                return Err(ConfigError::Invalid("fade must be <= chunk size".into()));
            }
        }
        if self.batch_size == 0 {
            return Err(ConfigError::Invalid("batch must be > 0".into()));
        }
        if !self.pitch_shift.is_finite() {
            return Err(ConfigError::Invalid("pitch_shift must be finite".into()));
        }
        if self.sample_rate == 0 {
            return Err(ConfigError::Invalid("sample_rate must be > 0".into()));
        }
        if !self.normalization.is_finite() || self.normalization < 0.0 {
            return Err(ConfigError::Invalid(
                "normalization must be finite and >= 0".into(),
            ));
        }
        if !self.amplification.is_finite() || self.amplification < 0.0 {
            return Err(ConfigError::Invalid(
                "amplification must be finite and >= 0".into(),
            ));
        }
        Ok(())
    }

    pub fn chunk_size(&self, model: &dyn crate::model::RoformerModel) -> usize {
        if let Some(c) = self.segment_size {
            c.max(1)
        } else {
            let hops = std::env::var("CANDLE_ROFORMER_CHUNK_HOPS")
                .ok()
                .and_then(|v| v.parse::<usize>().ok())
                .filter(|v| *v > 0)
                .unwrap_or(self.chunk_hops);
            model.hop().max(1) * hops.max(1)
        }
    }

    pub fn step_for(&self, chunk: usize) -> usize {
        if let Some(s) = self.step_size {
            return s.max(1);
        }
        if let Some(k) = self.overlap_size {
            return (chunk / k.max(1)).max(1);
        }
        (chunk / self.num_overlap.max(1)).max(1)
    }

    pub fn fade_for(&self, chunk: usize) -> usize {
        self.fade_size.unwrap_or((chunk / 10).max(1)).max(1)
    }

    pub fn from_yaml_str(text: &str) -> Result<Self, ConfigError> {
        #[derive(serde::Deserialize)]
        struct Root {
            #[serde(default)]
            inference: Option<InferenceConfig>,
        }
        let root: Root =
            serde_yaml::from_str(text).map_err(|e| ConfigError::Invalid(e.to_string()))?;
        let cfg = root.inference.unwrap_or_default();
        cfg.validate()?;
        Ok(cfg)
    }

    pub fn from_yaml_file(path: impl AsRef<std::path::Path>) -> Result<Self, ConfigError> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::Invalid(format!("cannot read config: {}", e)))?;
        Self::from_yaml_str(&text)
    }
}
