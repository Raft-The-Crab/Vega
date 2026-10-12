//! Serialization/deserialization errors.

use thiserror::Error;

/// Errors related to serialization and deserialization.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum SerializationError {
    /// Failed to serialize data.
    #[error("serialization failed: {0}")]
    SerializationFailed(String),

    /// Failed to deserialize data.
    #[error("deserialization failed: {0}")]
    DeserializationFailed(String),

    /// Incompatible data format version.
    #[error("incompatible format version: expected {expected}, found {found}")]
    IncompatibleVersion { expected: String, found: String },

    /// Missing required data during deserialization.
    #[error("missing required data: {0}")]
    MissingData(String),

    /// Invalid data structure encountered.
    #[error("invalid data structure: {0}")]
    InvalidStructure(String),

    /// Custom error from underlying serialization library.
    #[error("serialization library error: {0}")]
    LibraryError(String),
}
