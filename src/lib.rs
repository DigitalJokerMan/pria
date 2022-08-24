pub mod handler;

use handler::{FileHandler, HandlerCriteria};
use std::fmt::Display;

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
}
