//! Runtime errors (I/O, memory, etc.).

use thiserror::Error;

/// Runtime errors that occur during execution.
#[derive(Error, Debug)]
pub enum RuntimeError {
    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Insufficient memory for the requested operation.
    #[error("out of memory: {0}")]
    OutOfMemory(String),

    /// An operation was canceled by the user.
    #[error("operation canceled")]
    Canceled,

    /// A timeout occurred.
    #[error("operation timed out")]
    Timeout,

    /// A concurrent access or locking error.
    #[error("concurrency error: {0}")]
    Concurrency(String),

    /// Image dimensions exceed supported limits.
    #[error("image dimensions exceed limits: {width}x{height}")]
    DimensionsExceeded { width: u32, height: u32 },

    /// Invalid parameter value provided.
    #[error("invalid parameter: {0}")]
    InvalidParameter(String),
}
