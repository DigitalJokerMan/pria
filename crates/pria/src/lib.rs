#[cfg(feature = "image")]
pub mod image;
pub mod packager;
pub mod processor;

use packager::Packager;
use processor::{Processor, ProcessorCriteria, ProcessorCriteriaConflict};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Default)]
pub struct Factory {
    file_handlers: Vec<Box<dyn Processor>>,
}

#[allow(clippy::borrowed_box)]
impl Factory {
    pub fn new() -> Self {
        Self {
            file_handlers: Vec::new(),
        }
    }

    pub fn get_fallback_handler(&self) -> Option<&Box<dyn Processor>> {
        self.file_handlers
            .iter()
            .find(|handler| handler.criteria() == ProcessorCriteria::EverythingElse)
    }

    pub fn get_handler_for_extension(&self, extension: &str) -> Option<&Box<dyn Processor>> {
        self.file_handlers.iter().find(|handler| {
            if let ProcessorCriteria::Extensions(extensions) = handler.criteria() {
                extensions.contains(&extension)
            } else {
                false
            }
        })
    }

    pub fn add_file_handler(
        &mut self,
        file_handler: Box<dyn Processor>,
    ) -> Result<(), ProcessorCriteriaConflict> {
        if file_handler.criteria() == ProcessorCriteria::EverythingElse
            && self.get_fallback_handler().is_some()
        // Can't use "let Some(fallback_handler) = self.get_fallback_handler()" as that's considered unstable.
        {
            Err(ProcessorCriteriaConflict {
                new: file_handler.criteria(),
                existing: self.get_fallback_handler().unwrap().criteria(),
            })
        } else {
            self.file_handlers.push(file_handler);
            Ok(())
        }
    }

    pub fn process_folder_recursively<W>(&self, source_path: &Path, mut destination: W)
    where
        W: Packager,
    {
        for entry in WalkDir::new(source_path)
            .into_iter()
            .filter_map(|x| x.ok())
            .filter(|x| x.metadata().unwrap().is_file())
            .filter(|x| x.path().extension() != Some(OsStr::new("param")))
        {
            if let Some(handler) = entry
                .path()
                .extension()
                .and_then(|ext| self.get_handler_for_extension(ext.to_str().unwrap()))
                .or_else(|| self.get_fallback_handler())
            {
                let bytes = std::fs::read(entry.path()).unwrap();
                let parameters_bytes =
                    std::fs::read(format!("{}.param", entry.path().display())).ok();

                let output = handler
                    .process(bytes, parameters_bytes, entry.path())
                    .unwrap();

                let mut path = PathBuf::new();
                path.push(
                    &entry
                        .path()
                        .display()
                        .to_string()
                        .strip_prefix(&source_path.display().to_string())
                        .unwrap()[1..], // Trim the first character because it'll be '\' or '/'(?)
                );

                if let Some(preferred_extension) = output.preferred_extension {
                    path.set_extension(preferred_extension);
                }

                destination
                    .write_file(path.as_path(), output.bytes)
                    .unwrap();
            }
        }

        destination.flush();
    }
}
