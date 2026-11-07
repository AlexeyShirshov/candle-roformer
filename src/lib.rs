pub mod assets;
pub mod config;
pub mod device;
pub mod dsp;
pub mod error;
pub mod mel;
pub mod tensor_ops;

pub use assets::{LocalSeparationAssets, LocalWhisperAssets};
pub use config::SeparatorConfig;
pub use device::DeviceChoice;
pub use error::{AssetError, ConfigError, ConverterError, DeviceError, DspError, Error};
pub use mel::mel_bin_membership;
pub use tensor_ops::{average_by_counts, gather_1d, scatter_add_1d};

// Re-export candle_core::Device for convenience.
pub use candle_core::Device;
