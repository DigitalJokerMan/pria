use std::env;

use pria::{handler::CopyHandler, image::ImageHandler, writer::FSWriter, Processor};

fn main() {
    const SRC_PATH: &str = "assets_src";

    println!("cargo:rerun-if-changed={}", SRC_PATH);

    let mut processor = Processor::new();
    processor.add_file_handler(Box::new(CopyHandler)).unwrap();
    processor
        .add_file_handler(Box::new(ImageHandler::default()))
        .unwrap();
    processor.process_folder_recursively(
        SRC_PATH.as_ref(),
        &mut FSWriter::new(env::var("OUT_DIR").unwrap().as_ref()),
    );
}
