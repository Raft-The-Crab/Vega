//! VEGA file formats library
//!
//! Provides file format implementations for the VEGA image editor.

pub mod autosave;
pub mod vega;

pub use autosave::{AutosaveConfig, AutosaveError, AutosaveManager, CrashRecoveryManager};
pub use vega::{FormatError, Manifest, VegaProject, FORMAT_VERSION, Result};
