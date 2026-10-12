//! The document model.

use crate::color::ColorSpace;
use crate::document::ids::{DocumentId, NodeId};
use crate::document::nodes::Node;
use crate::document::revision::{Revision, RevisionCounter};
use crate::errors::{DocumentError, Result};
use crate::geometry::Size;
use crate::limits::{validate_dimensions, validate_name, validate_node_count, LimitError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

impl From<LimitError> for DocumentError {
    fn from(err: LimitError) -> Self {
        DocumentError::ResourceLimit(err.to_string())
    }
}

/// A VEGA document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Unique document ID.
    pub id: DocumentId,

    /// Document name.
    pub name: String,

    /// Canvas dimensions.
    pub size: Size,

    /// Color space of the document.
    pub color_space: ColorSpace,

    /// All nodes in the document, indexed by ID.
    nodes: HashMap<NodeId, Node>,

    /// Root nodes (top-level layers/groups).
    root_nodes: Vec<NodeId>,

    /// Document metadata.
    pub metadata: DocumentMetadata,

    /// Revision counter for tracking changes.
    #[serde(skip)]
    revision_counter: Arc<RevisionCounter>,
}

/// Document metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    /// Creation timestamp.
    pub created_at: u64,

    /// Last modification timestamp.
    pub modified_at: u64,

    /// Author/application that created the document.
    pub creator: String,

    /// Custom user metadata.
    pub custom: HashMap<String, String>,
}

impl Document {
    /// Create a new empty document.
    pub fn new(name: String, size: Size, color_space: ColorSpace) -> Self {
        // Validate dimensions
        validate_dimensions(size.width, size.height)
            .expect("Document dimensions should be validated at creation");

        // Validate name
        validate_name(&name).expect("Document name should be validated at creation");

        let id = DocumentId::generate();
        let now = Self::current_timestamp();

        Self {
            id,
            name,
            size,
            color_space,
            nodes: HashMap::new(),
            root_nodes: Vec::new(),
            metadata: DocumentMetadata {
                created_at: now,
                modified_at: now,
                creator: "VEGA".to_string(),
                custom: HashMap::new(),
            },
            revision_counter: Arc::new(RevisionCounter::new()),
        }
    }

    /// Get the current timestamp in seconds since epoch.
    fn current_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    /// Get the current document revision.
    pub fn revision(&self) -> Revision {
        self.revision_counter.current()
    }

    /// Get a reference to the revision counter.
    pub fn revision_counter(&self) -> Arc<RevisionCounter> {
        self.revision_counter.clone()
    }

    /// Create a revision guard for the current document state.
    pub fn create_revision_guard(&self) -> crate::document::RevisionGuard {
        crate::document::RevisionGuard::new(self.revision_counter.clone())
    }

    /// Increment the document revision.
    fn increment_revision(&self) {
        self.revision_counter.increment();
    }

    /// Add a node to the document.
    pub fn add_node(&mut self, node: Node) -> Result<NodeId> {
        let id = node.id();

        // Validate node count before adding
        validate_node_count(self.nodes.len() + 1)?;

        // Check for duplicate ID
        if self.nodes.contains_key(&id) {
            return Err(DocumentError::InvalidNode(format!(
                "Node with ID {} already exists",
                id
            ))
            .into());
        }

        self.nodes.insert(id, node);
        self.root_nodes.push(id);
        self.touch_metadata();
        self.increment_revision();

        Ok(id)
    }

    /// Add a node as a child of a group.
    pub fn add_child(&mut self, parent_id: NodeId, child_id: NodeId) -> Result<()> {
        // Verify both nodes exist
        if !self.nodes.contains_key(&parent_id) {
            return Err(DocumentError::LayerNotFound(format!("{}", parent_id)).into());
        }
        if !self.nodes.contains_key(&child_id) {
            return Err(DocumentError::LayerNotFound(format!("{}", child_id)).into());
        }

        // Verify parent is a group
        let parent = self.nodes.get(&parent_id).unwrap();
        match parent {
            Node::Group { children, .. } => {
                // This is a simplified check - in reality we'd need to handle the parent's children list
                // For now, we'll just verify the node exists
            }
            _ => {
                return Err(DocumentError::InvalidLayerOperation(
                    "Parent must be a group".to_string(),
                )
                .into());
            }
        }

        // Remove from root if it's there
        self.root_nodes.retain(|&id| id != child_id);

        // Add to parent's children
        if let Some(Node::Group { children, .. }) = self.nodes.get_mut(&parent_id) {
            children.push(child_id);
        }

        self.touch_metadata();
        self.increment_revision();
        Ok(())
    }

    /// Remove a node from the document.
    pub fn remove_node(&mut self, id: NodeId) -> Result<()> {
        if !self.nodes.contains_key(&id) {
            return Err(DocumentError::LayerNotFound(format!("{}", id)).into());
        }

        self.nodes.remove(&id);
        self.root_nodes.retain(|&node_id| node_id != id);

        // Also remove from any parent's children list
        for node in self.nodes.values_mut() {
            if let Node::Group { children, .. } = node {
                children.retain(|&child_id| child_id != id);
            }
        }

        self.touch_metadata();
        self.increment_revision();
        Ok(())
    }

    /// Get a node by ID.
    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    /// Get a mutable reference to a node by ID.
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }

    /// Get all root nodes.
    pub fn root_nodes(&self) -> &[NodeId] {
        &self.root_nodes
    }

    /// Get all nodes.
    pub fn nodes(&self) -> &HashMap<NodeId, Node> {
        &self.nodes
    }

    /// Update the modification timestamp.
    fn touch_metadata(&mut self) {
        self.metadata.modified_at = Self::current_timestamp();
    }

    /// Set document name.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
        self.touch_metadata();
        self.increment_revision();
    }

    /// Get the number of nodes in the document.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new(
            "Untitled".to_string(),
            Size::new(1920, 1080),
            ColorSpace::default(),
        )
    }
}

pub mod tests;
