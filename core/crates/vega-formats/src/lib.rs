//! VEGA file formats library
//!
//! Provides file format implementations for the VEGA image editor.

pub mod autosave;
pub mod integrity;
pub mod thumbnail;
pub mod vega;

pub use autosave::{AutosaveConfig, AutosaveError, AutosaveManager, CrashRecoveryManager};
pub use integrity::{compute_checksum, compute_checksum_bytes, verify_checksum, IntegrityError};
pub use thumbnail::{ThumbnailConfig, ThumbnailGenerator};
pub use vega::{FormatError, Manifest, VegaProject, FORMAT_VERSION, Result};
