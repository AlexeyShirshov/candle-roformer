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
pub enum MdxError {
    #[error("invalid MDX profile: {0}")]
    InvalidProfile(String),
    #[error("MDX model asset missing: {0}")]
    MissingAsset(String),
    #[error("MDX model checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },
    #[error("MDX inference failed: {0}")]
    Inference(String),
    #[error("unsupported MDX input: {0}")]
    UnsupportedInput(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ConverterError {
    #[error("converter error: {0}")]
    Failed(String),
}

/// Errors of the ensemble operators and preset resolution.
///
/// Every unknown operator or preset is a typed error: an id this crate does not
/// implement is never silently mapped onto a different computation.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum EnsembleError {
    #[error("unknown ensemble algorithm `{id}`: supported algorithms are {supported}")]
    UnknownAlgorithm { id: String, supported: String },
    #[error("unknown ensemble preset `{id}`: known presets are {known}")]
    UnknownPreset { id: String, known: String },
    #[error("ensemble needs at least one waveform")]
    EmptyWaveforms,
    #[error("ensemble channel mismatch: {0}")]
    ChannelMismatch(String),
    #[error("ensemble sample-rate mismatch: {0}")]
    SampleRateMismatch(String),
    #[error("invalid ensemble weights: {0}")]
    InvalidWeights(String),
    #[error("ensemble preset does not match the loaded separators: {0}")]
    MemberCount(String),
    #[error("invalid ensemble preset: {0}")]
    InvalidPreset(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AudioError {
    #[error("audio error: {0}")]
    Io(String),
    #[error("unsupported audio format: {0}")]
    UnsupportedFormat(String),
    #[error("invalid sample rate: {0}")]
    InvalidSampleRate(String),
    #[error("too many channels: {0}")]
    TooManyChannels(String),
    #[error("empty audio: {0}")]
    EmptyInput(String),
    #[error("non-finite sample: {0}")]
    NonFinite(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AsrError {
    #[error("whisper assets: {0}")]
    InvalidAssets(String),
    #[error("unsupported language: {0}")]
    UnsupportedLanguage(String),
    #[error("transcription failed: {0}")]
    Failed(String),
}

pub type WavError = AudioError;
pub type DecodeError = AudioError;

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
    Mdx(#[from] MdxError),
    #[error(transparent)]
    Converter(#[from] ConverterError),
    #[error(transparent)]
    Ensemble(#[from] EnsembleError),
    #[error(transparent)]
    Audio(#[from] AudioError),
    #[error(transparent)]
    Asr(#[from] AsrError),
    #[error("candle error: {0}")]
    Candle(#[from] candle_core::Error),
}
