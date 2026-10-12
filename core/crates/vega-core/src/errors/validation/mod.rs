//! Validation errors for inputs and operations.

use thiserror::Error;

/// Errors related to validation of inputs and operations.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// A required parameter is missing.
    #[error("missing required parameter: {0}")]
    MissingParameter(String),

    /// A parameter has an invalid value.
    #[error("invalid parameter '{param}': {message}")]
    InvalidParameter { param: String, message: String },

    /// A value is out of the allowed range.
    #[error("value out of range: {value} not in {min}..={max}")]
    OutOfRange {
        value: String,
        min: String,
        max: String,
    },

    /// A string is empty when a non-empty value is required.
    #[error("string cannot be empty: {0}")]
    EmptyString(String),

    /// An ID or identifier is invalid.
    #[error("invalid identifier: {0}")]
    InvalidIdentifier(String),

    /// An operation is not permitted in the current context.
    #[error("operation not permitted: {0}")]
    NotPermitted(String),

    /// Incompatible types or formats.
    #[error("incompatible types: {0}")]
    IncompatibleTypes(String),
}
