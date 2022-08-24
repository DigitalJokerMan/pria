use std::path::{Path, PathBuf};

/// A type which handles the processed output and stores/packages it to the destination.
pub trait Packager {
    fn write_file(&mut self, path: &Path, bytes: Vec<u8>) -> Result<(), anyhow::Error>;

    /// Called when writing is finished. Optional to implement.
    fn flush(self)
    where
        Self: Sized,
    {
    }
}

/// Writes destination files to a directory.
pub struct FSPackager {
    pub destination: PathBuf,
}

impl FSPackager {
    pub fn new(destination: &Path) -> Self {
        Self {
            destination: destination.to_path_buf(),
        }
    }
}

impl Packager for FSPackager {
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
