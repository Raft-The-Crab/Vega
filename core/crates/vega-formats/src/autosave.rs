//! Autosave and crash recovery
//!
//! Provides automatic saving and crash recovery mechanisms for VEGA documents.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use thiserror::Error;

/// Autosave errors.
#[derive(Debug, Error)]
pub enum AutosaveError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Autosave disabled")]
    Disabled,

    #[error("Autosave not ready: {0}")]
    NotReady(String),
}

/// Result type for autosave operations.
pub type Result<T> = std::result::Result<T, AutosaveError>;

/// Autosave configuration.
#[derive(Debug, Clone)]
pub struct AutosaveConfig {
    /// Enable autosave.
    pub enabled: bool,
    /// Autosave interval in seconds.
    pub interval_secs: u64,
    /// Maximum number of autosave backups to keep.
    pub max_backups: usize,
}

impl Default for AutosaveConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_secs: 300, // 5 minutes
            max_backups: 5,
        }
    }
}

/// Autosave manager.
pub struct AutosaveManager {
    config: AutosaveConfig,
    document_path: Option<PathBuf>,
    last_autosave: Option<Instant>,
    save_callback: Box<dyn Fn() -> Result<()> + Send>,
}

impl AutosaveManager {
    /// Creates a new autosave manager.
    pub fn new(config: AutosaveConfig) -> Self {
        Self {
            config,
            document_path: None,
            last_autosave: None,
            save_callback: Box::new(|| Err(AutosaveError::NotReady("No save callback".to_string()))),
        }
    }

    /// Sets the document path.
    pub fn set_document_path<P: AsRef<Path>>(&mut self, path: P) {
        self.document_path = Some(path.as_ref().to_path_buf());
    }

    /// Sets the save callback.
    pub fn set_save_callback<F>(&mut self, callback: F)
    where
        F: Fn() -> Result<()> + Send + 'static,
    {
        self.save_callback = Box::new(callback);
    }

    /// Checks if autosave should trigger.
    pub fn should_autosave(&self) -> bool {
        if !self.config.enabled {
            return false;
        }

        if self.document_path.is_none() {
            return false;
        }

        let interval = Duration::from_secs(self.config.interval_secs);
        if let Some(last) = self.last_autosave {
            last.elapsed() >= interval
        } else {
            true
        }
    }

    /// Performs an autosave.
    pub fn autosave(&mut self) -> Result<()> {
        if !self.config.enabled {
            return Err(AutosaveError::Disabled);
        }

        if self.document_path.is_none() {
            return Err(AutosaveError::NotReady("No document path".to_string()));
        }

        // Call the save callback
        (self.save_callback)()?;

        // Update last autosave time
        self.last_autosave = Some(Instant::now());

        // Clean up old backups
        self.cleanup_old_backups()?;

        Ok(())
    }

    /// Gets the autosave backup directory for a document.
    fn autosave_dir(&self) -> Option<PathBuf> {
        let doc_path = self.document_path.as_ref()?;
        let mut autosave_dir = doc_path.parent()?.to_path_buf();
        autosave_dir.push(".vega_autosave");
        Some(autosave_dir)
    }

    /// Cleans up old autosave backups.
    fn cleanup_old_backups(&self) -> Result<()> {
        let autosave_dir = match self.autosave_dir() {
            Some(dir) => dir,
            None => return Ok(()),
        };

        if !autosave_dir.exists() {
            return Ok(());
        }

        let mut backups: Vec<_> = fs::read_dir(&autosave_dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "autosave"))
            .collect();

        // Sort by modification time (oldest first)
        backups.sort_by_key(|entry| {
            entry.metadata().and_then(|m| m.modified()).ok()
        });

        // Remove excess backups
        while backups.len() > self.config.max_backups {
            if let Some(oldest) = backups.first() {
                fs::remove_file(oldest.path())?;
                backups.remove(0);
            }
        }

        Ok(())
    }

    /// Returns the configuration.
    pub fn config(&self) -> &AutosaveConfig {
        &self.config
    }

    /// Updates the configuration.
    pub fn set_config(&mut self, config: AutosaveConfig) {
        self.config = config;
    }
}

impl Default for AutosaveManager {
    fn default() -> Self {
        Self::new(AutosaveConfig::default())
    }
}

/// Crash recovery manager.
pub struct CrashRecoveryManager {
    autosave_dir: PathBuf,
}

impl CrashRecoveryManager {
    /// Creates a new crash recovery manager.
    pub fn new<P: AsRef<Path>>(autosave_dir: P) -> Self {
        Self {
            autosave_dir: autosave_dir.as_ref().to_path_buf(),
        }
    }

    /// Checks for crash recovery files.
    pub fn has_recovery_files(&self) -> bool {
        if !self.autosave_dir.exists() {
            return false;
        }

        fs::read_dir(&self.autosave_dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .any(|entry| entry.path().extension().map_or(false, |ext| ext == "autosave"))
            })
            .unwrap_or(false)
    }

    /// Lists available recovery files.
    pub fn list_recovery_files(&self) -> Result<Vec<PathBuf>> {
        if !self.autosave_dir.exists() {
            return Ok(Vec::new());
        }

        let mut files: Vec<_> = fs::read_dir(&self.autosave_dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "autosave"))
            .map(|entry| entry.path())
            .collect();

        // Sort by modification time (newest first)
        files.sort_by(|a, b| {
            let time_a = a.metadata().and_then(|m| m.modified()).unwrap_or(std::time::UNIX_EPOCH);
            let time_b = b.metadata().and_then(|m| m.modified()).unwrap_or(std::time::UNIX_EPOCH);
            time_b.cmp(&time_a)
        });

        Ok(files)
    }

    /// Deletes a recovery file.
    pub fn delete_recovery_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::remove_file(path)?;
        Ok(())
    }

    /// Clears all recovery files.
    pub fn clear_recovery_files(&self) -> Result<()> {
        if !self.autosave_dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&self.autosave_dir)? {
            let entry = entry?;
            if entry.path().extension().map_or(false, |ext| ext == "autosave") {
                fs::remove_file(entry.path())?;
            }
        }

        Ok(())
    }

    /// Returns the autosave directory.
    pub fn autosave_dir(&self) -> &Path {
        &self.autosave_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_autosave_config_default() {
        let config = AutosaveConfig::default();
        assert!(config.enabled);
        assert_eq!(config.interval_secs, 300);
        assert_eq!(config.max_backups, 5);
    }

    #[test]
    fn test_autosave_manager() {
        let config = AutosaveConfig::default();
        let mut manager = AutosaveManager::new(config);

        assert!(!manager.should_autosave()); // No document path

        manager.set_document_path("/tmp/test.vega");
        assert!(manager.should_autosave()); // Should autosave immediately
    }

    #[test]
    fn test_crash_recovery_manager() {
        let temp_dir = TempDir::new().unwrap();
        let manager = CrashRecoveryManager::new(temp_dir.path());

        assert!(!manager.has_recovery_files());

        let files = manager.list_recovery_files().unwrap();
        assert!(files.is_empty());
    }
}
