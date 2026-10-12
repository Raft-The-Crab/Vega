//! VEGA native file format (.vega)
//!
//! Provides the native .vega file format implementation with ZIP container,
//! atomic saves, and integrity checking.

use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;
use zip::{write::FileOptions, ZipArchive, ZipWriter};
use vega_core::{Document, Result as CoreResult};

/// VEGA file format version.
pub const FORMAT_VERSION: &str = "1.0.0";

/// File format errors.
#[derive(Debug, Error)]
pub enum FormatError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid format version: {0}")]
    InvalidVersion(String),

    #[error("Missing required file: {0}")]
    MissingFile(String),

    #[error("Corrupted file: {0}")]
    Corrupted(String),

    #[error("Invalid document: {0}")]
    InvalidDocument(String),
}

/// Result type for format operations.
pub type Result<T> = std::result::Result<T, FormatError>;

/// VEGA project manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// Format version.
    pub format_version: String,
    /// Created timestamp.
    pub created_at: String,
    /// Modified timestamp.
    pub modified_at: String,
    /// VEGA core version.
    pub vega_version: String,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            format_version: FORMAT_VERSION.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            modified_at: chrono::Utc::now().to_rfc3339(),
            vega_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

/// VEGA project file handler.
pub struct VegaProject {
    path: PathBuf,
}

impl VegaProject {
    /// Creates a new project handler for the given path.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    /// Saves a document to a .vega file with atomic write.
    ///
    /// # Arguments
    /// * `document` - The document to save
    ///
    /// # Errors
    /// Returns an error if the file cannot be written.
    pub fn save(&self, document: &Document) -> Result<()> {
        // Create temporary file for atomic write
        let temp_path = self.path.with_extension("vega.tmp");
        
        // Write to temporary file
        let file = File::create(&temp_path)?;
        let mut zip = ZipWriter::new(BufWriter::new(file));

        // Write manifest
        let manifest = Manifest::default();
        let manifest_json = serde_json::to_string_pretty(&manifest)?;
        zip.start_file("manifest.json", FileOptions::default())?;
        zip.write_all(manifest_json.as_bytes())?;

        // Write document
        let document_json = serde_json::to_string_pretty(document)?;
        zip.start_file("document.json", FileOptions::default())?;
        zip.write_all(document_json.as_bytes())?;

        // Finish the ZIP
        zip.finish()?;

        // Atomically replace the original file
        if self.path.exists() {
            fs::remove_file(&self.path)?;
        }
        fs::rename(&temp_path, &self.path)?;

        Ok(())
    }

    /// Loads a document from a .vega file.
    ///
    /// # Errors
    /// Returns an error if the file cannot be read or is corrupted.
    pub fn load(&self) -> Result<Document> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let mut zip = ZipArchive::new(reader)?;

        // Read and validate manifest
        let manifest = self.read_manifest(&mut zip)?;
        if manifest.format_version != FORMAT_VERSION {
            return Err(FormatError::InvalidVersion(manifest.format_version));
        }

        // Read document
        let document = self.read_document(&mut zip)?;

        Ok(document)
    }

    /// Reads the manifest from the ZIP archive.
    fn read_manifest(&self, zip: &mut ZipArchive<BufReader<File>>) -> Result<Manifest> {
        let manifest_file = zip.by_name("manifest.json")
            .map_err(|_| FormatError::MissingFile("manifest.json".to_string()))?;

        let mut manifest_json = String::new();
        BufReader::new(manifest_file).read_to_string(&mut manifest_json)?;

        let manifest: Manifest = serde_json::from_str(&manifest_json)?;
        Ok(manifest)
    }

    /// Reads the document from the ZIP archive.
    fn read_document(&self, zip: &mut ZipArchive<BufReader<File>>) -> Result<Document> {
        let document_file = zip.by_name("document.json")
            .map_err(|_| FormatError::MissingFile("document.json".to_string()))?;

        let mut document_json = String::new();
        BufReader::new(document_file).read_to_string(&mut document_json)?;

        let document: Document = serde_json::from_str(&document_json)
            .map_err(|e| FormatError::InvalidDocument(e.to_string()))?;

        Ok(document)
    }

    /// Validates the integrity of a .vega file.
    ///
    /// # Errors
    /// Returns an error if the file is corrupted or invalid.
    pub fn validate(&self) -> Result<()> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let mut zip = ZipArchive::new(reader)?;

        // Check for required files
        let manifest = self.read_manifest(&mut zip)?;
        let _document = self.read_document(&mut zip)?;

        // Validate format version
        if manifest.format_version != FORMAT_VERSION {
            return Err(FormatError::InvalidVersion(manifest.format_version));
        }

        Ok(())
    }

    /// Returns the file path.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vega_core::{ColorSpace, Size};

    #[test]
    fn test_manifest_default() {
        let manifest = Manifest::default();
        assert_eq!(manifest.format_version, FORMAT_VERSION);
    }

    #[test]
    fn test_vega_project_save_load() {
        use tempfile::NamedTempFile;

        let temp_file = NamedTempFile::new().unwrap();
        let project = VegaProject::new(temp_file.path());

        // Create a document
        let document = Document::new(
            "Test".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        // Save
        project.save(&document).unwrap();

        // Load
        let loaded = project.load().unwrap();
        assert_eq!(loaded.name(), "Test");
    }

    #[test]
    fn test_vega_project_validate() {
        use tempfile::NamedTempFile;

        let temp_file = NamedTempFile::new().unwrap();
        let project = VegaProject::new(temp_file.path());

        let document = Document::new(
            "Test".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        project.save(&document).unwrap();
        project.validate().unwrap();
    }
}
