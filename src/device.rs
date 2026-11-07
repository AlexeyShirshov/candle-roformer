use crate::error::DeviceError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceChoice {
    Cpu,
    Cuda,
    Metal,
}

impl DeviceChoice {
    pub fn resolve(&self) -> Result<candle_core::Device, DeviceError> {
        match self {
            Self::Cpu => Ok(candle_core::Device::Cpu),
            Self::Cuda => candle_core::Device::new_cuda(0).map_err(|e| DeviceError::Unavailable {
                backend: "Cuda",
                reason: e.to_string(),
            }),
            Self::Metal => {
                candle_core::Device::new_metal(0).map_err(|e| DeviceError::Unavailable {
                    backend: "Metal",
                    reason: e.to_string(),
                })
            }
        }
    }

    pub fn is_available(&self) -> bool {
        self.resolve().is_ok()
    }
}
