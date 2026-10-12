//! # VEGA Core
//!
//! The authoritative document model, editing operations, and history for the VEGA image editor.

pub mod color;
pub mod commands;
pub mod document;
pub mod errors;
pub mod geometry;
pub mod limits;

pub use color::{ColorSpace, Rgba, Rgba8, Rgba16, Rgba32};
pub use commands::{Command, CommandExecutor, Transaction};
pub use document::{
    AdjustmentType, BlendMode, CommandId, Document, DocumentId, DocumentMetadata, LayerId, MaskData,
    Node, NodeId, NodeProperties, NodeType, Revision, RevisionCounter, RevisionGuard,
    RevisionMismatchError, ShapeType, Transform,
};
pub use errors::{VegaError, Result};
pub use geometry::{Point, Rect, Size};
pub use limits::{LimitError, MAX_DOCUMENT_HEIGHT, MAX_DOCUMENT_WIDTH, MAX_NODES};
