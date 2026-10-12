//! VEGA file formats library
//!
//! Provides file format implementations for the VEGA image editor.

pub mod autosave;
pub mod integrity;
pub mod layers;
pub mod recovery_ui;
pub mod thumbnail;
pub mod tiles;
pub mod vega;

pub use autosave::{AutosaveConfig, AutosaveError, AutosaveManager, CrashRecoveryManager};
pub use integrity::{compute_checksum, compute_checksum_bytes, verify_checksum, IntegrityError};
pub use layers::{LayerMetadata, LayerSerializer};
pub use recovery_ui::{RecoveryFileInfo, RecoverySession};
pub use thumbnail::{ThumbnailConfig, ThumbnailGenerator};
pub use tiles::TileSerializer;
pub use vega::{FormatError, Manifest, VegaProject, FORMAT_VERSION, Result};
