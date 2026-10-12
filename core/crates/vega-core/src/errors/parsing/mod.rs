//! Parsing errors for file formats.

use thiserror::Error;

/// Errors related to parsing file formats.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The file format is not recognized or supported.
    #[error("unsupported file format: {0}")]
    UnsupportedFormat(String),

    /// The file is corrupted or malformed.
    #[error("malformed file: {0}")]
    Malformed(String),

    /// A required field or structure is missing.
    #[error("missing required field: {0}")]
    MissingField(String),

    /// A field has an invalid value.
    #[error("invalid value for field '{field}': {message}")]
    InvalidValue { field: String, message: String },

    /// The file version is newer than what this version can handle.
    #[error("unsupported file version: {0}")]
    UnsupportedVersion(String),

    /// An I/O error occurred during parsing.
    #[error("I/O error during parsing: {0}")]
    Io(String),

    /// The file exceeds size limits.
    #[error("file size exceeds limit: {0} bytes")]
    SizeLimitExceeded(usize),
}
