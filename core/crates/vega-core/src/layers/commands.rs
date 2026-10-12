//! Commands for layer manipulation
//!
//! Provides undoable operations for creating, deleting, moving, and transforming layers.

use crate::commands::base::Command;
use crate::document::model::document::Document;
use crate::document::nodes::Node;
use crate::document::ids::structure::NodeId;
use crate::document::nodes::NodeType;
use crate::errors::DocumentError;
use crate::geometry::Rect;
use crate::document::nodes::Transform;

/// Command to create a new layer.
pub struct CreateLayerCommand {
    parent_id: Option<NodeId>,
    layer_id: Option<NodeId>,
    name: String,
    node_type: NodeType,
}

impl CreateLayerCommand {
    /// Creates a new command to add a layer.
    pub fn new(parent_id: Option<NodeId>, name: String, node_type: NodeType) -> Self {
        Self {
            parent_id,
            layer_id: None,
            name,
            node_type,
        }
    }

    /// Returns the ID of the created layer, if execution succeeded.
    pub fn layer_id(&self) -> Option<NodeId> {
        self.layer_id
    }
}

impl Command for CreateLayerCommand {
    fn execute(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        let node = Node::new(self.name.clone(), self.node_type.clone());
        let id = node.id();
        self.layer_id = Some(id);

        if let Some(parent_id) = self.parent_id {
            document.add_child_node(parent_id, node)?;
        } else {
            document.add_node(node)?;
        }

        Ok(())
    }

    fn undo(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        if let Some(layer_id) = self.layer_id {
            document.remove_node(layer_id)?;
        }
        Ok(())
    }

    fn redo(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        self.execute(document)
    }
}

/// Command to delete a layer.
pub struct DeleteLayerCommand {
    layer_id: NodeId,
    removed_node: Option<Node>,
    parent_id: Option<NodeId>,
}

impl DeleteLayerCommand {
    /// Creates a new command to delete a layer.
    pub fn new(layer_id: NodeId) -> Self {
        Self {
            layer_id,
            removed_node: None,
            parent_id: None,
        }
    }
}

impl Command for DeleteLayerCommand {
    fn execute(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        let node = document.get_node(self.layer_id).ok_or(DocumentError::NodeNotFound(
            self.layer_id,
        ))?;
        self.removed_node = Some(node.clone());
        self.parent_id = document.get_parent(self.layer_id);

        document.remove_node(self.layer_id)?;
        Ok(())
    }

    fn undo(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        if let Some(node) = self.removed_node.take() {
            if let Some(parent_id) = self.parent_id {
                document.add_child(parent_id, node)?;
            } else {
                document.add_node(node)?;
            }
        }
        Ok(())
    }

    fn redo(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        self.execute(document)
    }
}

/// Command to move a layer to a new parent.
pub struct MoveLayerCommand {
    layer_id: NodeId,
    old_parent_id: Option<NodeId>,
    new_parent_id: Option<NodeId>,
}

impl MoveLayerCommand {
    /// Creates a new command to move a layer.
    pub fn new(layer_id: NodeId, new_parent_id: Option<NodeId>) -> Self {
        Self {
            layer_id,
            old_parent_id: None,
            new_parent_id,
        }
    }
}

impl Command for MoveLayerCommand {
    fn execute(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        self.old_parent_id = document.get_parent(self.layer_id);

        let node = document.remove_node(self.layer_id)?;
        if let Some(parent_id) = self.new_parent_id {
            document.add_child_node(parent_id, node)?;
        } else {
            document.add_node(node)?;
        }

        Ok(())
    }

    fn undo(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        let node = document.remove_node(self.layer_id)?;
        if let Some(parent_id) = self.old_parent_id {
            document.add_child_node(parent_id, node)?;
        } else {
            document.add_node(node)?;
        }
        Ok(())
    }

    fn redo(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        self.execute(document)
    }
}

/// Command to reorder a layer within its parent.
pub struct ReorderLayerCommand {
    layer_id: NodeId,
    old_index: Option<usize>,
    new_index: usize,
}

impl ReorderLayerCommand {
    /// Creates a new command to reorder a layer.
    pub fn new(layer_id: NodeId, new_index: usize) -> Self {
        Self {
            layer_id,
            old_index: None,
            new_index,
        }
    }
}

impl Command for ReorderLayerCommand {
    fn execute(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        let parent_id = document
            .get_parent(self.layer_id)
            .ok_or(DocumentError::NodeNotFound(self.layer_id))?;

        let children = document.get_children(parent_id)?;
        let old_index = children
            .iter()
            .position(|&id| id == self.layer_id)
            .ok_or(DocumentError::NodeNotFound(self.layer_id))?;

        self.old_index = Some(old_index);

        // Remove and re-insert at new position
        document.reorder_child(parent_id, self.layer_id, self.new_index)?;
        Ok(())
    }

    fn undo(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        if let Some(old_index) = self.old_index {
            let parent_id = document
                .get_parent(self.layer_id)
                .ok_or(DocumentError::NodeNotFound(self.layer_id))?;

            document.reorder_child(parent_id, self.layer_id, old_index)?;
        }
        Ok(())
    }

    fn redo(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        self.execute(document)
    }
}

/// Command to transform a layer.
pub struct TransformLayerCommand {
    layer_id: NodeId,
    old_transform: Option<Transform>,
    new_transform: Transform,
}

impl TransformLayerCommand {
    /// Creates a new command to transform a layer.
    pub fn new(layer_id: NodeId, new_transform: Transform) -> Self {
        Self {
            layer_id,
            old_transform: None,
            new_transform,
        }
    }
}

impl Command for TransformLayerCommand {
    fn execute(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        let node = document.get_node_mut(self.layer_id).ok_or(DocumentError::NodeNotFound(
            self.layer_id,
        ))?;

        self.old_transform = Some(node.properties().transform.clone());
        node.properties_mut().transform = self.new_transform.clone();

        Ok(())
    }

    fn undo(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        if let Some(old_transform) = self.old_transform.take() {
            let node = document.get_node_mut(self.layer_id).ok_or(DocumentError::NodeNotFound(
                self.layer_id,
            ))?;
            node.properties_mut().transform = old_transform;
        }
        Ok(())
    }

    fn redo(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        self.execute(document)
    }
}

/// Command to duplicate a layer.
pub struct DuplicateLayerCommand {
    layer_id: NodeId,
    duplicate_id: Option<NodeId>,
    parent_id: Option<NodeId>,
}

impl DuplicateLayerCommand {
    /// Creates a new command to duplicate a layer.
    pub fn new(layer_id: NodeId) -> Self {
        Self {
            layer_id,
            duplicate_id: None,
            parent_id: None,
        }
    }

    /// Returns the ID of the duplicated layer, if execution succeeded.
    pub fn duplicate_id(&self) -> Option<NodeId> {
        self.duplicate_id
    }
}

impl Command for DuplicateLayerCommand {
    fn execute(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        let node = document.get_node(self.layer_id).ok_or(DocumentError::NodeNotFound(
            self.layer_id,
        ))?;

        self.parent_id = document.get_parent(self.layer_id);

        let mut duplicate = node.clone();
        duplicate.set_name(format!("{} Copy", node.name()));

        let id = duplicate.id();
        self.duplicate_id = Some(id);

        if let Some(parent_id) = self.parent_id {
            document.add_child(parent_id, duplicate)?;
        } else {
            document.add_node(duplicate)?;
        }

        Ok(())
    }

    fn undo(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        if let Some(duplicate_id) = self.duplicate_id {
            document.remove_node(duplicate_id)?;
        }
        Ok(())
    }

    fn redo(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        self.execute(document)
    }
}

/// Command to merge a layer down with the layer below it.
pub struct MergeDownCommand {
    layer_id: NodeId,
    below_id: Option<NodeId>,
    merged_node: Option<Node>,
    parent_id: Option<NodeId>,
}

impl MergeDownCommand {
    /// Creates a new command to merge a layer down.
    pub fn new(layer_id: NodeId) -> Self {
        Self {
            layer_id,
            below_id: None,
            merged_node: None,
            parent_id: None,
        }
    }
}

impl Command for MergeDownCommand {
    fn execute(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        let parent_id = document
            .get_parent(self.layer_id)
            .ok_or(DocumentError::NodeNotFound(self.layer_id))?;

        self.parent_id = Some(parent_id);

        let children = document.get_children(parent_id)?;
        let layer_index = children
            .iter()
            .position(|&id| id == self.layer_id)
            .ok_or(DocumentError::NodeNotFound(self.layer_id))?;

        if layer_index == 0 {
            return Err(DocumentError::InvalidOperation(
                "Cannot merge bottom layer down".to_string(),
            ));
        }

        let below_id = children[layer_index - 1];
        self.below_id = Some(below_id);

        // Remove both layers
        let layer = document.remove_node(self.layer_id)?;
        let below = document.remove_node(below_id)?;

        // In a real implementation, this would perform actual pixel merging
        // For now, we'll just keep the below layer and rename it
        let mut merged = below;
        merged.set_name(format!("Merged {}", merged.name()));

        self.merged_node = Some(merged.clone());
        document.add_child_node(parent_id, merged)?;

        Ok(())
    }

    fn undo(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        if let (Some(parent_id), Some(below_id), Some(merged_node)) =
            (self.parent_id, self.below_id, self.merged_node.take())
        {
            // Remove the merged layer
            document.remove_node(merged_node.id())?;

            // Restore the original layers
            let layer = document
                .get_node(self.layer_id)
                .ok_or(DocumentError::NodeNotFound(self.layer_id))?
                .clone();

            let below = document
                .get_node(below_id)
                .ok_or(DocumentError::NodeNotFound(below_id))?
                .clone();

            document.add_child_node(parent_id, layer)?;
            document.add_child_node(parent_id, below)?;
        }
        Ok(())
    }

    fn redo(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        self.execute(document)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::nodes::mod_rs::NodeType;

    #[test]
    fn test_create_layer_command() {
        let mut document = Document::new("Test".to_string(), crate::geometry::Size::new(1920, 1080), crate::color::ColorSpace::default());
        let command = CreateLayerCommand::new(None, "Test Layer".to_string(), NodeType::Raster);

        command.execute(&mut document).unwrap();
        assert!(command.layer_id().is_some());

        command.undo(&mut document).unwrap();
        assert!(document.get_node(command.layer_id().unwrap()).is_none());
    }

    #[test]
    fn test_delete_layer_command() {
        let mut document = Document::new("Test".to_string(), crate::geometry::Size::new(1920, 1080), crate::color::ColorSpace::default());
        let node = Node::new("Test Layer".to_string(), NodeType::Raster);
        let layer_id = node.id();
        document.add_node(node).unwrap();

        let command = DeleteLayerCommand::new(layer_id);
        command.execute(&mut document).unwrap();
        assert!(document.get_node(layer_id).is_none());

        command.undo(&mut document).unwrap();
        assert!(document.get_node(layer_id).is_some());
    }

    #[test]
    fn test_move_layer_command() {
        let mut document = Document::new("Test".to_string(), crate::geometry::Size::new(1920, 1080), crate::color::ColorSpace::default());
        let group = Node::new("Group".to_string(), NodeType::Group);
        let group_id = group.id();
        document.add_node(group).unwrap();

        let layer = Node::new("Layer".to_string(), NodeType::Raster);
        let layer_id = layer.id();
        document.add_node(layer).unwrap();

        let command = MoveLayerCommand::new(layer_id, Some(group_id));
        command.execute(&mut document).unwrap();
        assert_eq!(document.get_parent(layer_id), Some(group_id));

        command.undo(&mut document).unwrap();
        assert_eq!(document.get_parent(layer_id), None);
    }
}
