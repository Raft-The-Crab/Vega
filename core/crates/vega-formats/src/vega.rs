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
use crate::integrity;
use crate::layers::LayerSerializer;
use crate::thumbnail;

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
    /// SHA-256 checksum of document.json.
    pub document_checksum: String,
    /// SHA-256 checksum of thumbnail.png.
    pub thumbnail_checksum: String,
    /// Number of layers in the document.
    pub layer_count: usize,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            format_version: FORMAT_VERSION.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            modified_at: chrono::Utc::now().to_rfc3339(),
            vega_version: env!("CARGO_PKG_VERSION").to_string(),
            document_checksum: String::new(),
            thumbnail_checksum: String::new(),
            layer_count: 0,
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

        // Serialize document first to compute checksum
        let document_json = serde_json::to_string_pretty(document)?;
        let document_checksum = integrity::compute_checksum_bytes(document_json.as_bytes());

        // Generate thumbnail and compute checksum
        let thumbnail_generator = thumbnail::ThumbnailGenerator::with_defaults();
        let thumbnail = thumbnail_generator.generate_thumbnail(document)?;
        let thumbnail_png = thumbnail_generator.encode_png(&thumbnail)?;
        let thumbnail_checksum = integrity::compute_checksum_bytes(&thumbnail_png);

        // Write manifest with checksums
        let manifest = Manifest {
            document_checksum,
            thumbnail_checksum,
            layer_count: document.node_count(),
            ..Default::default()
        };
        let manifest_json = serde_json::to_string_pretty(&manifest)?;
        zip.start_file("manifest.json", FileOptions::default())?;
        zip.write_all(manifest_json.as_bytes())?;

        // Write document
        zip.start_file("document.json", FileOptions::default())?;
        zip.write_all(document_json.as_bytes())?;

        // Write thumbnail
        zip.start_file("previews/thumbnail.png", FileOptions::default())?;
        zip.write_all(&thumbnail_png)?;

        // Write layer metadata
        LayerSerializer::serialize_layers(document.nodes(), &mut zip)?;

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
        let document_json = self.read_document_json(zip)?;
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
        let document_json = self.read_document_json(&mut zip)?;

        // Validate format version
        if manifest.format_version != FORMAT_VERSION {
            return Err(FormatError::InvalidVersion(manifest.format_version));
        }

        // Verify document checksum
        let computed_checksum = integrity::compute_checksum_bytes(document_json.as_bytes());
        if computed_checksum != manifest.document_checksum {
            return Err(FormatError::Corrupted(
                format!("Document checksum mismatch: expected {}, got {}",
                    manifest.document_checksum, computed_checksum)
            ));
        }

        // Verify thumbnail checksum if present
        if let Ok(thumbnail_file) = zip.by_name("previews/thumbnail.png") {
            let mut thumbnail_data = Vec::new();
            BufReader::new(thumbnail_file).read_to_end(&mut thumbnail_data)?;
            let computed_checksum = integrity::compute_checksum_bytes(&thumbnail_data);
            if computed_checksum != manifest.thumbnail_checksum {
                return Err(FormatError::Corrupted(
                    format!("Thumbnail checksum mismatch: expected {}, got {}",
                        manifest.thumbnail_checksum, computed_checksum)
                ));
            }
        }

        Ok(())
    }

    /// Reads the document JSON from the ZIP archive.
    fn read_document_json(&self, zip: &mut ZipArchive<BufReader<File>>) -> Result<String> {
        let document_file = zip.by_name("document.json")
            .map_err(|_| FormatError::MissingFile("document.json".to_string()))?;

        let mut document_json = String::new();
        BufReader::new(document_file).read_to_string(&mut document_json)?;

        Ok(document_json)
    }

    /// Returns the file path.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::{BufReader, BufWriter, Write};
    use std::path::Path;
    use zip::{write::FileOptions, ZipWriter};
    use tempfile::TempDir;
    use vega_core::{ColorSpace, Size, Document, commands::base::CommandExecutor, layers::CreateLayerCommand, document::nodes::NodeType};

    #[test]
    fn test_manifest_default() {
        let manifest = Manifest::default();
        assert_eq!(manifest.format_version, FORMAT_VERSION);
        assert!(manifest.document_checksum.is_empty());
        assert!(manifest.thumbnail_checksum.is_empty());
        assert_eq!(manifest.layer_count, 0);
    }

    #[test]
    fn test_vega_project_save_load() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("test.vega");
        let project = VegaProject::new(&project_path);

        let document = Document::new(
            "Test".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        project.save(&document).unwrap();
        let loaded = project.load().unwrap();
        assert_eq!(loaded.name(), "Test");
    }

    #[test]
    fn test_vega_project_validate() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("test.vega");
        let project = VegaProject::new(&project_path);

        let document = Document::new(
            "Test".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        project.save(&document).unwrap();
        project.validate().unwrap();
    }

    #[test]
    fn test_vega_project_roundtrip() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("test.vega");
        let project = VegaProject::new(&project_path);

        let mut document = Document::new(
            "Test Document".to_string(),
            Size::new(1920, 1080),
            ColorSpace::default(),
        );

        let mut executor = CommandExecutor::new(100);
        let mut cmd1 = CreateLayerCommand::new(None, "Layer 1".to_string(), NodeType::Raster);
        executor.execute(&mut document, &mut cmd1).unwrap();

        let mut cmd2 = CreateLayerCommand::new(None, "Layer 2".to_string(), NodeType::Raster);
        executor.execute(&mut document, &mut cmd2).unwrap();

        project.save(&document).unwrap();
        let loaded = project.load().unwrap();

        assert_eq!(loaded.name(), "Test Document");
        assert_eq!(loaded.size(), Size::new(1920, 1080));
        assert_eq!(loaded.node_count(), 2);
    }

    #[test]
    fn test_vega_project_missing_manifest() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("test.vega");

        let file = File::create(&project_path).unwrap();
        let mut zip = ZipWriter::new(BufWriter::new(file));
        zip.finish().unwrap();

        let project = VegaProject::new(&project_path);
        let result = project.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_large_document_save_load() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("large.vega");
        let project = VegaProject::new(&project_path);

        let document = Document::new(
            "Large Document".to_string(),
            Size::new(100_000, 100_000),
            ColorSpace::default(),
        ).unwrap();

        project.save(&document).unwrap();
        let loaded = project.load().unwrap();

        assert_eq!(loaded.name(), "Large Document");
        assert_eq!(loaded.size(), Size::new(100_000, 100_000));
    }

    #[test]
    fn test_empty_document_save_load() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("empty.vega");
        let project = VegaProject::new(&project_path);

        let document = Document::new(
            "Empty".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        project.save(&document).unwrap();
        let loaded = project.load().unwrap();

        assert_eq!(loaded.node_count(), 0);
    }

    #[test]
    fn test_special_characters_in_name() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("special.vega");
        let project = VegaProject::new(&project_path);

        let document = Document::new(
            "Test & Document (2024) \"quotes\"".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        project.save(&document).unwrap();
        let loaded = project.load().unwrap();

        assert_eq!(loaded.name(), "Test & Document (2024) \"quotes\"");
    }
}
