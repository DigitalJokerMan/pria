use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlerCriteria {
    EverythingElse,
    Extensions(&'static [&'static str]),
}

pub struct ProcessOutput {
    pub bytes: Vec<u8>,
    /// The preferred file extension of the output, if ``None``, the extension will not change.
    pub preferred_extension: Option<&'static str>,
}

pub trait FileHandler {
    fn criteria(&self) -> HandlerCriteria;
    fn process(&self, bytes: &[u8], source_path: &Path) -> Result<ProcessOutput, anyhow::Error>;
}

pub struct CopyHandler;

impl FileHandler for CopyHandler {
    fn criteria(&self) -> HandlerCriteria {
        HandlerCriteria::EverythingElse
    }

    fn process(&self, bytes: &[u8], _source_path: &Path) -> Result<ProcessOutput, anyhow::Error> {
        Ok(ProcessOutput {
            bytes: bytes.to_vec(),
            preferred_extension: None,
        })
    }
}
