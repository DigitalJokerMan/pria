use std::env;

use pria::{image::ImageHandler, packager::FSPackager, processor::CopyProcessor, Factory};

fn main() {
    const SRC_PATH: &str = "assets_src";

    println!("cargo:rerun-if-changed={}", SRC_PATH);

    let mut factory = Factory::new();
    factory.add_file_handler(Box::new(CopyProcessor)).unwrap();
    factory
        .add_file_handler(Box::new(ImageHandler::default()))
        .unwrap();
    factory.process_folder_recursively(
        SRC_PATH.as_ref(),
        FSPackager::new(env::var("OUT_DIR").unwrap().as_ref()),
    );
}
