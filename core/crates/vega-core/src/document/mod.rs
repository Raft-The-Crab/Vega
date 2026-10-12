//! Document model and structure.

pub mod ids;
pub mod model;
pub mod nodes;
pub mod revision;
pub mod validation;

pub use ids::structure::{CommandId, DocumentId, LayerId, NodeId};
pub use model::document::{Document, DocumentMetadata};
pub use nodes::{
    AdjustmentType, BlendMode, MaskData, Node, NodeProperties, NodeType, ShapeType, Transform,
};
pub use revision::{Revision, RevisionCounter, RevisionGuard, RevisionMismatchError};
pub use validation::DocumentValidator;
