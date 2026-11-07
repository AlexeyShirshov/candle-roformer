use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ConfigError {
    #[error("invalid separator config: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AssetError {
    #[error("invalid local asset path: {0}")]
    InvalidPath(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DeviceError {
    #[error("requested device `{backend}` is unavailable: {reason}")]
    Unavailable {
        backend: &'static str,
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DspError {
    #[error("invalid DSP parameters: {0}")]
    InvalidParams(String),
    #[error("STFT/ISTFT shape mismatch: {0}")]
    ShapeMismatch(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ConverterError {
    #[error("converter error: {0}")]
    Failed(String),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Asset(#[from] AssetError),
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error(transparent)]
    Dsp(#[from] DspError),
    #[error(transparent)]
    Converter(#[from] ConverterError),
    #[error("candle error: {0}")]
    Candle(#[from] candle_core::Error),
}
