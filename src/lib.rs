//! Offline vocal/instrumental separation in Rust on top of Candle.
//!
//! The crate is the product: [`Separator`] loads a local MelBandRoformer
//! checkpoint once and separates in-memory audio buffers. File handling lives
//! in [`wav`] and [`pipeline`]; the `candle-roformer` binary is only a thin
//! CLI over this API.

pub mod assets;
pub mod config;
pub mod device;
pub mod dsp;
pub mod error;
pub mod mel;
pub mod model;
pub mod pipeline;
pub mod resample;
pub mod separate;
pub mod separator;
pub mod tensor_ops;
pub mod wav;
pub mod whisper;

pub use assets::{LocalSeparationAssets, LocalWhisperAssets};
pub use config::SeparatorConfig;
pub use device::DeviceChoice;
pub use error::{
    AsrError, AssetError, AudioError, ConfigError, ConverterError, DecodeError, DeviceError,
    DspError, Error, WavError,
};
pub use mel::mel_bin_membership;
pub use model::MelBandRoformer;
pub use separator::{Separator, Stems};
pub use tensor_ops::{average_by_counts, gather_1d, scatter_add_1d};
pub use wav::AudioBuffer;
pub use whisper::{TranscribeOptions, Transcriber, Transcript, TranscriptSegment};

// Re-export candle_core::Device for convenience.
pub use candle_core::Device;
