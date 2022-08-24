use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

/// Defines what scenarios the processor runs in, output of [`Processor::criteria`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessorCriteria {
    EverythingElse,
    Extensions(&'static [&'static str]),
}

#[derive(Debug, Clone)]
pub struct ProcessorCriteriaConflict {
    pub new: ProcessorCriteria,
    pub existing: ProcessorCriteria,
}

impl Display for ProcessorCriteriaConflict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cannot insert handler: its criteria ({:?}) conflicts with an already existing criteria ({:?})", self.new, self.existing)
    }
}

/// The output of [`Processor::process`].
pub struct ProcessOutput {
    pub bytes: Vec<u8>,
    /// The preferred file extension of the output, if ``None``, the extension will not change.
    pub preferred_extension: Option<&'static str>,
}

/// Handles input files, transforms it somehow and spits out the result.
pub trait Processor {
    fn criteria(&self) -> ProcessorCriteria;
    fn process(
        &self,
        bytes: Vec<u8>,
        parameters_bytes: Option<Vec<u8>>,
        source_path: &Path,
    ) -> Result<ProcessOutput, anyhow::Error>;
}

/// Simply copies the source data.
pub struct CopyProcessor;

impl Processor for CopyProcessor {
    fn criteria(&self) -> ProcessorCriteria {
        ProcessorCriteria::EverythingElse
    }

    fn process(
        &self,
        bytes: Vec<u8>,
        _parameters_bytes: Option<Vec<u8>>,
        _source_path: &Path,
    ) -> Result<ProcessOutput, anyhow::Error> {
        Ok(ProcessOutput {
            bytes,
            preferred_extension: None,
        })
    }
}
