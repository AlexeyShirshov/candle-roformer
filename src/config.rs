use crate::error::ConfigError;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SeparatorConfig {
    pub dim: usize,
    pub depth: usize,
    pub stereo: bool,
    pub num_stems: usize,
    pub num_bands: usize,
    pub dim_head: usize,
    pub heads: usize,
    #[serde(default = "default_true")]
    pub flash_attn: bool,
    pub dim_freqs_in: usize,
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
    /// The exact profile from `v7.yaml` (MelBandRoformer vocals).
    pub fn v7() -> Self {
        Self {
            dim: 256,
            depth: 12,
            stereo: true,
            num_stems: 1,
            num_bands: 60,
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
        if self.num_bands == 0 {
            return Err(ConfigError::Invalid("num_bands must be > 0".into()));
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
