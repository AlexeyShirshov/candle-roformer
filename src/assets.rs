use std::path::PathBuf;

use crate::error::AssetError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSeparationAssets {
    pub checkpoint: PathBuf,
    pub config: PathBuf,
}

impl LocalSeparationAssets {
    pub fn new(checkpoint: impl Into<PathBuf>, config: impl Into<PathBuf>) -> Self {
        Self {
            checkpoint: checkpoint.into(),
            config: config.into(),
        }
    }

    pub fn validate(&self) -> Result<(), AssetError> {
        if self.checkpoint.as_os_str().is_empty() {
            return Err(AssetError::InvalidPath("checkpoint path is empty".into()));
        }
        if self.config.as_os_str().is_empty() {
            return Err(AssetError::InvalidPath("config path is empty".into()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalWhisperAssets {
    pub config: PathBuf,
    pub tokenizer: PathBuf,
    pub model: PathBuf,
}

impl LocalWhisperAssets {
    pub fn new(
        config: impl Into<PathBuf>,
        tokenizer: impl Into<PathBuf>,
        model: impl Into<PathBuf>,
    ) -> Self {
        Self {
            config: config.into(),
            tokenizer: tokenizer.into(),
            model: model.into(),
        }
    }

    pub fn validate(&self) -> Result<(), AssetError> {
        if self.config.as_os_str().is_empty()
            || self.tokenizer.as_os_str().is_empty()
            || self.model.as_os_str().is_empty()
        {
            return Err(AssetError::InvalidPath(
                "whisper asset path is empty".into(),
            ));
        }
        Ok(())
    }
}
