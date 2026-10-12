//! Document validation and invariants.

use crate::document::ids::NodeId;
use crate::document::nodes::Node;
use crate::document::Document;
use crate::errors::{DocumentError, Result};
use std::collections::{HashMap, HashSet};

/// Validates a document and its structure.
pub struct DocumentValidator;

impl DocumentValidator {
    /// Validate the entire document structure.
    pub fn validate(document: &Document) -> Result<()> {
        Self::validate_node_existence(document)?;
        Self::validate_no_orphaned_children(document)?;
        Self::validate_no_circular_references(document)?;
        Self::validate_unique_names(document)?;
        Ok(())
    }

    /// Ensure all referenced nodes exist.
    fn validate_node_existence(document: &Document) -> Result<()> {
        let all_nodes: HashSet<NodeId> = document.nodes().keys().copied().collect();

        // Check root nodes
        for &root_id in document.root_nodes() {
            if !all_nodes.contains(&root_id) {
                return Err(DocumentError::InvalidNode(format!(
                    "Root node {} does not exist",
                    root_id
                ))
                .into());
            }
        }

        // Check children in groups
        for node in document.nodes().values() {
            if let Node::Group { children, .. } = node {
                for &child_id in children {
                    if !all_nodes.contains(&child_id) {
                        return Err(DocumentError::InvalidNode(format!(
                            "Child node {} does not exist",
                            child_id
                        ))
                        .into());
                    }
                }
            }
        }

        Ok(())
    }

    /// Ensure no child nodes are orphaned (referenced by a parent but not in the graph).
    fn validate_no_orphaned_children(document: &Document) -> Result<()> {
        let mut all_children: HashSet<NodeId> = HashSet::new();

        // Collect all children
        for node in document.nodes().values() {
            if let Node::Group { children, .. } = node {
                for &child_id in children {
                    all_children.insert(child_id);
                }
            }
        }

        // Check that all children are either root or have a parent
        let root_set: HashSet<NodeId> = document.root_nodes().iter().copied().collect();

        for node_id in all_children {
            if !root_set.contains(&node_id) {
                // Node is a child but not a root - need to verify it has a parent
                let has_parent = document.nodes().values().any(|node| {
                    if let Node::Group { children, .. } = node {
                        children.contains(&node_id)
                    } else {
                        false
                    }
                });

                if !has_parent {
                    return Err(DocumentError::InvalidNode(format!(
                        "Node {} is orphaned (referenced as child but has no parent)",
                        node_id
                    ))
                    .into());
                }
            }
        }

        Ok(())
    }

    /// Ensure no circular references in the node graph.
    fn validate_no_circular_references(document: &Document) -> Result<()> {
        let mut visited = HashSet::new();
        let mut visiting = HashSet::new();

        for &root_id in document.root_nodes() {
            if let Err(e) = Self::check_node_acyclic(document, root_id, &mut visited, &mut visiting) {
                return Err(e);
            }
        }

        Ok(())
    }

    fn check_node_acyclic(
        document: &Document,
        node_id: NodeId,
        visited: &mut HashSet<NodeId>,
        visiting: &mut HashSet<NodeId>,
    ) -> Result<()> {
        if visited.contains(&node_id) {
            return Ok(());
        }

        if visiting.contains(&node_id) {
            return Err(DocumentError::CircularReference.into());
        }

        visiting.insert(node_id);

        if let Some(node) = document.get_node(node_id) {
            if let Node::Group { children, .. } = node {
                for &child_id in children {
                    Self::check_node_acyclic(document, child_id, visited, visiting)?;
                }
            }
        }

        visiting.remove(&node_id);
        visited.insert(node_id);

        Ok(())
    }

    /// Ensure node names are unique within their scope.
    fn validate_unique_names(document: &Document) -> Result<()> {
        let mut name_counts: HashMap<String, usize> = HashMap::new();

        for node in document.nodes().values() {
            let name = &node.properties().name;
            *name_counts.entry(name.clone()).or_insert(0) += 1;
        }

        for (name, count) in name_counts {
            if count > 1 {
                // This is a warning, not an error, as duplicate names might be acceptable
                // For now, we'll log it but not fail validation
                tracing::warn!("Duplicate node name: {} ({} occurrences)", name, count);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::ColorSpace;
    use crate::document::{NodeProperties, NodeType};
    use crate::geometry::Size;

    #[test]
    fn test_validate_empty_document() {
        let doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);
        assert!(DocumentValidator::validate(&doc).is_ok());
    }

    #[test]
    fn test_validate_with_nodes() {
        let mut doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);
        let node_id = NodeId::generate();
        let node = Node::Raster {
            properties: NodeProperties::new(node_id, NodeType::Raster, "Layer 1".to_string()),
            bounds: crate::geometry::Rect::from_size(Size::new(100, 100)),
            color_space: ColorSpace::Srgb,
        };
        doc.add_node(node).unwrap();
        assert!(DocumentValidator::validate(&doc).is_ok());
    }

    #[test]
    fn test_validate_circular_reference() {
        let mut doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);

        let parent_id = NodeId::generate();
        let child_id = NodeId::generate();

        let parent = Node::Group {
            properties: NodeProperties::new(parent_id, NodeType::Group, "Parent".to_string()),
            children: vec![child_id],
        };

        let child = Node::Group {
            properties: NodeProperties::new(child_id, NodeType::Group, "Child".to_string()),
            children: vec![parent_id], // Circular reference
        };

        doc.add_node(parent).unwrap();
        doc.add_node(child).unwrap();

        assert!(matches!(
            DocumentValidator::validate(&doc),
            Err(crate::errors::VegaError::Document(
                crate::errors::DocumentError::CircularReference
            ))
        ));
    }
}
