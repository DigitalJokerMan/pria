use std::path::Path;

use crate::handler::{FileHandler, HandlerCriteria, ProcessOutput};

pub struct ImageHandler;

impl FileHandler for ImageHandler {
    fn criteria(&self) -> HandlerCriteria {
        HandlerCriteria::Extensions(&["png", "jpg"]) // TODO: Don't hard-code this, bound it by features.
    }

    fn process(&self, bytes: &[u8], _source_path: &Path) -> Result<ProcessOutput, anyhow::Error> {
        let source_image = image::load_from_memory(bytes)?;

        // Convert to QOI.
        let qoi_data = qoi::encode_to_vec(
            source_image.as_rgba8().unwrap().as_raw(),
            source_image.width(),
            source_image.height(),
        )?;

        Ok(ProcessOutput {
            bytes: qoi_data,
            preferred_extension: Some("qoi"),
        })
    }
}
