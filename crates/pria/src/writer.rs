use std::path::{Path, PathBuf};

pub trait Writer {
    fn write_file(&mut self, path: &Path, bytes: Vec<u8>) -> Result<(), anyhow::Error>;
    fn flush(&mut self) {}
}

pub struct FSWriter {
    pub destination: PathBuf,
}

impl FSWriter {
    pub fn new(destination: &Path) -> Self {
        Self {
            destination: destination.to_path_buf(),
        }
    }
}

impl Writer for FSWriter {
    fn write_file(&mut self, path: &Path, bytes: Vec<u8>) -> Result<(), anyhow::Error> {
        let mut final_path = PathBuf::new();
        final_path.push(&self.destination);
        final_path.push(path);

        if let Some(parent) = final_path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }

        std::fs::write(final_path, &bytes)?;
        Ok(())
    }
}
