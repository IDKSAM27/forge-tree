use crate::{Result, ScaffoldError};
use std::fs;
use std::path::Path;

pub struct FileGenerator {
    force_overwrite: bool,
}

impl FileGenerator {
    pub fn new() -> Self {
        Self {
            force_overwrite: false,
        }
    }

    pub fn with_force_overwrite(mut self, force: bool) -> Self {
        self.force_overwrite = force;
        self
    }

    pub fn create_directory<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        
        if path.exists() && !path.is_dir() {
            return Err(ScaffoldError::InvalidPath(
                format!("Path exists but is not a directory: {}", path.display())
            ));
        }

        fs::create_dir_all(path)?;
        Ok(())
    }

    pub fn create_file<P: AsRef<Path>>(&self, path: P, content: &str) -> Result<()> {
        let path = path.as_ref();

        if path.exists() && !self.force_overwrite {
            return Err(ScaffoldError::FileExists(path.display().to_string()));
        }

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            self.create_directory(parent)?;
        }

        fs::write(path, content)?;
        Ok(())
    }
}

impl Default for FileGenerator {
    fn default() -> Self {
        Self::new()
    }
}
