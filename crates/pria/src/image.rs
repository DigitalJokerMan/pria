use image::ImageOutputFormat;
use serde::{Deserialize, Serialize};
use std::{io::Cursor, path::Path};

use crate::processor::{ProcessOutput, Processor, ProcessorCriteria};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum OutputType {
    PNG,
    JPG(u8),
    QOI,
}

impl Default for OutputType {
    fn default() -> Self {
        OutputType::QOI
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct ImageHandlerParameters {
    pub output: OutputType,
}

#[derive(Default)]
pub struct ImageHandler {
    pub default_parameters: ImageHandlerParameters,
}

impl Processor for ImageHandler {
    fn criteria(&self) -> ProcessorCriteria {
        ProcessorCriteria::Extensions(&["png", "jpg"]) // TODO: Don't hard-code this, bound it by features.
    }

    fn process(
        &self,
        bytes: Vec<u8>,
        parameters_bytes: Option<Vec<u8>>,
        source_path: &Path,
    ) -> Result<ProcessOutput, anyhow::Error> {
        let source_image = image::load_from_memory(&bytes)?;
        let parameters = parameters_bytes
            .map(|bytes| ron::de::from_bytes(&bytes).unwrap())
            .unwrap_or_else(|| self.default_parameters);

        match parameters.output {
            OutputType::PNG => {
                let mut buffer = Cursor::new(vec![]);
                source_image.write_to(&mut buffer, ImageOutputFormat::Png)?;

                Ok(ProcessOutput {
                    bytes: buffer.into_inner(),
                    preferred_extension: Some("png"),
                })
            }
            OutputType::JPG(quality) => {
                let mut buffer = Cursor::new(vec![]);
                source_image.write_to(&mut buffer, ImageOutputFormat::Jpeg(quality))?;

                Ok(ProcessOutput {
                    bytes: buffer.into_inner(),
                    preferred_extension: Some("jpg"),
                })
            }
            OutputType::QOI => {
                let qoi_data = qoi::encode_to_vec(
                    source_image.to_rgba8().as_raw(),
                    source_image.width(),
                    source_image.height(),
                )?;

                Ok(ProcessOutput {
                    bytes: qoi_data,
                    preferred_extension: Some("qoi"),
                })
            }
        }
    }
}
