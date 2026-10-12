//! Document-related errors.

use thiserror::Error;

/// Errors related to document structure and operations.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum DocumentError {
    /// No document with the given ID exists.
    #[error("document not found: {0}")]
    NotFound(String),

    /// The document is in an invalid state for the requested operation.
    #[error("invalid document state: {0}")]
    InvalidState(String),

    /// A layer with the given ID does not exist.
    #[error("layer not found: {0}")]
    LayerNotFound(String),

    /// Invalid layer operation (e.g., moving a layer into itself).
    #[error("invalid layer operation: {0}")]
    InvalidLayerOperation(String),

    /// The document has reached a resource limit (e.g., max layers).
    #[error("document limit exceeded: {0}")]
    LimitExceeded(String),

    /// A node in the document graph is invalid.
    #[error("invalid node: {0}")]
    InvalidNode(String),

    /// Circular reference detected in the document graph.
    #[error("circular reference detected")]
    CircularReference,

    /// Resource limit error.
    #[error("resource limit: {0}")]
    ResourceLimit(String),

    /// Invalid parameter value.
    #[error("invalid parameter: {0}")]
    InvalidParameter(String),

    /// Node not found.
    #[error("node not found: {0}")]
    NodeNotFound(crate::document::ids::structure::NodeId),

    /// Invalid operation for current state.
    #[error("invalid operation: {0}")]
    InvalidOperation(String),

    /// Tile operation error.
    #[error("tile error: {0}")]
    TileError(String),

    /// Rendering error.
    #[error("rendering error: {0}")]
    RenderError(String),
}

impl From<crate::errors::VegaError> for DocumentError {
    fn from(err: crate::errors::VegaError) -> Self {
        match err {
            crate::errors::VegaError::Document(e) => e,
            crate::errors::VegaError::ResourceLimit(s) => DocumentError::ResourceLimit(s),
            _ => DocumentError::InvalidState(err.to_string()),
        }
    }
}
