//! Recovery UI helpers
//!
//! Provides helper structures for displaying recovery information to users.

use chrono::{DateTime, Utc};
use std::path::PathBuf;
use std::time::SystemTime;

/// Recovery file information for UI display.
#[derive(Debug, Clone)]
pub struct RecoveryFileInfo {
    /// File path.
    pub path: PathBuf,
    /// Modified timestamp.
    pub modified_at: DateTime<Utc>,
    /// File size in bytes.
    pub size: u64,
    /// Document name (extracted from path if available).
    pub document_name: Option<String>,
}

impl RecoveryFileInfo {
    /// Creates recovery file info from a path.
    pub fn from_path(path: PathBuf) -> Self {
        let metadata = std::fs::metadata(&path).ok();
        let modified_at = metadata
            .and_then(|m| m.modified().ok())
            .and_then(|t| DateTime::from_system_time(t))
            .unwrap_or_else(|_| Utc::now());

        let size = metadata.map(|m| m.len()).unwrap_or(0);

        // Try to extract document name from path
        let document_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string());

        Self {
            path,
            modified_at,
            size,
            document_name,
        }
    }

    /// Formats the file size for display.
    pub fn format_size(&self) -> String {
        if self.size < 1024 {
            format!("{} B", self.size)
        } else if self.size < 1024 * 1024 {
            format!("{:.1} KB", self.size as f64 / 1024.0)
        } else if self.size < 1024 * 1024 * 1024 {
            format!("{:.1} MB", self.size as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", self.size as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }

    /// Formats the modification time for display.
    pub fn format_modified_at(&self) -> String {
        self.modified_at.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

/// Recovery session information.
#[derive(Debug, Clone)]
pub struct RecoverySession {
    /// Session ID.
    pub id: String,
    /// Recovery files in this session.
    pub files: Vec<RecoveryFileInfo>,
    /// Timestamp when session was detected.
    pub detected_at: DateTime<Utc>,
}

impl RecoverySession {
    /// Creates a new recovery session.
    pub fn new(id: String, files: Vec<RecoveryFileInfo>) -> Self {
        Self {
            id,
            files,
            detected_at: Utc::now(),
        }
    }

    /// Returns the number of recovery files.
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Returns the total size of all recovery files.
    pub fn total_size(&self) -> u64 {
        self.files.iter().map(|f| f.size).sum()
    }

    /// Formats the total size for display.
    pub fn format_total_size(&self) -> String {
        let total = self.total_size();
        if total < 1024 {
            format!("{} B", total)
        } else if total < 1024 * 1024 {
            format!("{:.1} KB", total as f64 / 1024.0)
        } else if total < 1024 * 1024 * 1024 {
            format!("{:.1} MB", total as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", total as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_recovery_file_info() {
        let temp_file = NamedTempFile::new().unwrap();
        let info = RecoveryFileInfo::from_path(temp_file.path().to_path_buf());

        assert!(info.size >= 0);
        assert!(!info.format_size().is_empty());
        assert!(!info.format_modified_at().is_empty());
    }

    #[test]
    fn test_recovery_session() {
        let files = vec![];
        let session = RecoverySession::new("test-session".to_string(), files);

        assert_eq!(session.id, "test-session");
        assert_eq!(session.file_count(), 0);
        assert_eq!(session.total_size(), 0);
    }
}
