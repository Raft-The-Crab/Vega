//! Error types for the VEGA core.
//!
//! This module provides structured error types that distinguish between
//! expected user errors (unsupported features, invalid parameters, etc.)
//! and programmer defects.

pub mod document;
pub mod parsing;
pub mod runtime;
pub mod serialization;
pub mod validation;

pub use document::DocumentError;
pub use parsing::ParseError;
pub use runtime::RuntimeError;
pub use serialization::SerializationError;
pub use validation::ValidationError;

use thiserror::Error;

/// The primary error type for VEGA core operations.
#[derive(Error, Debug)]
pub enum VegaError {
    /// Errors related to document structure and operations.
    #[error("document error: {0}")]
    Document(#[from] DocumentError),

    /// Errors related to parsing file formats.
    #[error("parse error: {0}")]
    Parse(#[from] ParseError),

    /// Runtime errors (I/O, memory, etc.).
    #[error("runtime error: {0}")]
    Runtime(#[from] RuntimeError),

    /// Errors related to serialization/deserialization.
    #[error("serialization error: {0}")]
    Serialization(#[from] SerializationError),

    /// Validation errors for invalid inputs or states.
    #[error("validation error: {0}")]
    Validation(#[from] ValidationError),
}

/// Result type alias for VEGA operations.
pub type Result<T> = std::result::Result<T, VegaError>;
