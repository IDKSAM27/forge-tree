//! File and directory creation utilities
//! 
//! This module handles the actual filesystem operations for creating
//! directories and files, with support for force overwrite mode.

use crate::{Result, ForgeTreeError};
use std::fs;
use std::path::Path;

/// Handles creation of files and directories on the filesystem
/// 
/// FileGenerator provides safe file operations with configurable
/// overwrite behavior and proper error handling.
pub struct FileGenerator {
    /// Whether to overwrite existing files without error
    force_overwrite: bool,
}

impl FileGenerator {
    /// Create a new FileGenerator with default settings (no force overwrite)
    pub fn new() -> Self {
        Self {
            force_overwrite: false,
        }
    }

    /// Configure whether existing files should be overwritten
    /// 
    /// When force_overwrite is true, existing files will be replaced.
    /// When false, attempting to create existing files will return an error.
    pub fn with_force_overwrite(mut self, force: bool) -> Self {
        self.force_overwrite = force;
        self
    }

    /// Create a directory and all necessary parent directories
    /// 
    /// This method:
    /// - Creates the directory and any missing parent directories
    /// - Handles the case where the path already exists as a directory
    /// - Returns an error if the path exists but is not a directory
    pub fn create_directory<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        
        // Check if path exists and is not a directory (e.g., it's a file)
        if path.exists() && !path.is_dir() {
            return Err(ForgeTreeError::InvalidPath(
                format!("Path exists but is not a directory: {}", path.display())
            ));
        }

        // create_dir_all is idempotent - it won't error if directory already exists
        fs::create_dir_all(path)?;
        Ok(())
    }

    /// Create a file with the specified content
    /// 
    /// This method:
    /// - Checks for existing files and respects the force_overwrite setting
    /// - Creates parent directories if they don't exist
    /// - Writes the content to the file (overwrites if file exists and force is true)
    pub fn create_file<P: AsRef<Path>>(&self, path: P, content: &str) -> Result<()> {
        let path = path.as_ref();

        // Check if file already exists and we're not in force mode
        if path.exists() && !self.force_overwrite {
            return Err(ForgeTreeError::FileExists(path.display().to_string()));
        }

        // Ensure the parent directory exists before creating the file
        if let Some(parent) = path.parent() {
            self.create_directory(parent)?;
        }

        // fs::write automatically creates or overwrites the file
        fs::write(path, content)?;
        Ok(())
    }
}

impl Default for FileGenerator {
    fn default() -> Self {
        Self::new()
    }
}
