pub mod handler;

use handler::{FileHandler, HandlerCriteria};
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Default)]
pub struct Processor {
    file_handlers: Vec<Box<dyn FileHandler>>,
}

#[derive(Debug, Clone)]
pub struct HandlerCriteriaConflict {
    new: HandlerCriteria,
    existing: HandlerCriteria,
}

impl Display for HandlerCriteriaConflict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cannot insert handler: its criteria ({:?}) conflicts with an already existing criteria ({:?})", self.new, self.existing)
    }
}

#[allow(clippy::borrowed_box)]
impl Processor {
    pub fn new() -> Self {
        Self {
            file_handlers: Vec::new(),
        }
    }

    pub fn get_fallback_handler(&self) -> Option<&Box<dyn FileHandler>> {
        self.file_handlers
            .iter()
            .find(|handler| handler.criteria() == HandlerCriteria::EverythingElse)
    }

    pub fn get_handler_for_extension(&self, extension: &str) -> Option<&Box<dyn FileHandler>> {
        self.file_handlers.iter().find(|handler| {
            if let HandlerCriteria::Extensions(extensions) = handler.criteria() {
                extensions.contains(&extension)
            } else {
                false
            }
        })
    }

    pub fn add_file_handler(
        &mut self,
        file_handler: Box<dyn FileHandler>,
    ) -> Result<(), HandlerCriteriaConflict> {
        if file_handler.criteria() == HandlerCriteria::EverythingElse
            && self.get_fallback_handler().is_some()
        // Can't use "let Some(fallback_handler) = self.get_fallback_handler()" as that's considered unstable.
        {
            Err(HandlerCriteriaConflict {
                new: file_handler.criteria(),
                existing: self.get_fallback_handler().unwrap().criteria(),
            })
        } else {
            self.file_handlers.push(file_handler);
            Ok(())
        }
    }

    pub fn process_folder_recursively(&self, source_path: &Path, destination_path: &Path) {
        for entry in WalkDir::new(source_path)
            .into_iter()
            .filter_map(|x| x.ok())
            .filter(|x| x.metadata().unwrap().is_file())
        {
            if let Some(handler) = entry
                .path()
                .extension()
                .and_then(|ext| self.get_handler_for_extension(ext.to_str().unwrap()))
                .or_else(|| self.get_fallback_handler())
            {
                let bytes = std::fs::read(entry.path()).unwrap();
                let output = handler.process(&bytes, entry.path()).unwrap();
                let mut path = PathBuf::new();
                path.push(&destination_path);
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

                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent).unwrap();
                }

                std::fs::write(path, output.bytes).unwrap();
            }
        }
    }
}
