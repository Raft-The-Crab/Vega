#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::ColorSpace;
    use crate::document::ids::structure::NodeId;
    use crate::document::nodes::{Node, NodeProperties, NodeType};
    use crate::geometry::{Point, Rect, Size};

    #[test]
    fn test_document_new() {
        let doc = Document::new("test".to_string(), Size::new(100, 200), ColorSpace::Srgb);
        assert_eq!(doc.name, "test");
        assert_eq!(doc.size.width, 100);
        assert_eq!(doc.size.height, 200);
        assert_eq!(doc.color_space, ColorSpace::Srgb);
    }

    #[test]
    fn test_document_default() {
        let doc = Document::default();
        assert_eq!(doc.name, "Untitled");
        assert_eq!(doc.size.width, 1920);
        assert_eq!(doc.size.height, 1080);
    }

    #[test]
    fn test_document_add_node() {
        let mut doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);
        let node_id = NodeId::generate();
        let node = Node::Raster {
            properties: NodeProperties::new(node_id, NodeType::Raster, "Layer 1".to_string()),
            bounds: Rect::from_size(Size::new(100, 100)),
            color_space: ColorSpace::Srgb,
        };

        let result = doc.add_node(node);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), node_id);
        assert_eq!(doc.node_count(), 1);
    }

    #[test]
    fn test_document_add_duplicate_node() {
        let mut doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);
        let node_id = NodeId::generate();
        let node = Node::Raster {
            properties: NodeProperties::new(node_id, NodeType::Raster, "Layer 1".to_string()),
            bounds: Rect::from_size(Size::new(100, 100)),
            color_space: ColorSpace::Srgb,
        };

        doc.add_node(node.clone()).unwrap();
        let result = doc.add_node(node);
        assert!(result.is_err());
    }

    #[test]
    fn test_document_remove_node() {
        let mut doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);
        let node_id = NodeId::generate();
        let node = Node::Raster {
            properties: NodeProperties::new(node_id, NodeType::Raster, "Layer 1".to_string()),
            bounds: Rect::from_size(Size::new(100, 100)),
            color_space: ColorSpace::Srgb,
        };

        doc.add_node(node).unwrap();
        assert_eq!(doc.node_count(), 1);

        doc.remove_node(node_id).unwrap();
        assert_eq!(doc.node_count(), 0);
    }

    #[test]
    fn test_document_remove_nonexistent_node() {
        let mut doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);
        let result = doc.remove_node(NodeId::generate());
        assert!(result.is_err());
    }

    #[test]
    fn test_document_get_node() {
        let mut doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);
        let node_id = NodeId::generate();
        let node = Node::Raster {
            properties: NodeProperties::new(node_id, NodeType::Raster, "Layer 1".to_string()),
            bounds: Rect::from_size(Size::new(100, 100)),
            color_space: ColorSpace::Srgb,
        };

        doc.add_node(node).unwrap();
        let retrieved = doc.get_node(node_id);
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_document_get_node_mut() {
        let mut doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);
        let node_id = NodeId::generate();
        let node = Node::Raster {
            properties: NodeProperties::new(node_id, NodeType::Raster, "Layer 1".to_string()),
            bounds: Rect::from_size(Size::new(100, 100)),
            color_space: ColorSpace::Srgb,
        };

        doc.add_node(node).unwrap();
        let retrieved = doc.get_node_mut(node_id);
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_document_set_name() {
        let mut doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);
        doc.set_name("new name".to_string());
        assert_eq!(doc.name, "new name");
    }

    #[test]
    fn test_document_metadata_touch() {
        let mut doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);
        let old_modified = doc.metadata.modified_at;
        doc.set_name("new".to_string());
        assert!(doc.metadata.modified_at > old_modified);
    }

    #[test]
    fn test_document_revision() {
        let doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);
        assert_eq!(doc.revision().value(), 0);
    }

    #[test]
    fn test_document_revision_increment() {
        let mut doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);
        let old_rev = doc.revision();
        doc.set_name("new".to_string());
        assert!(doc.revision().is_after(old_rev));
    }

    #[test]
    fn test_document_revision_guard() {
        let doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);
        let guard = doc.create_revision_guard();
        assert!(guard.is_valid());
    }

    #[test]
    fn test_document_revision_guard_invalid() {
        let mut doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);
        let guard = doc.create_revision_guard();
        doc.set_name("new".to_string());
        assert!(!guard.is_valid());
    }

    #[test]
    fn test_document_add_child() {
        let mut doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);

        let parent_id = NodeId::generate();
        let child_id = NodeId::generate();

        let parent = Node::Group {
            properties: NodeProperties::new(parent_id, NodeType::Group, "Parent".to_string()),
            children: vec![],
        };

        let child = Node::Raster {
            properties: NodeProperties::new(child_id, NodeType::Raster, "Child".to_string()),
            bounds: Rect::from_size(Size::new(50, 50)),
            color_space: ColorSpace::Srgb,
        };

        doc.add_node(parent).unwrap();
        doc.add_node(child).unwrap();

        // Remove child from root first
        doc.root_nodes.retain(|&id| id != child_id);

        let result = doc.add_child(parent_id, child_id);
        assert!(result.is_ok());

        let parent_node = doc.get_node(parent_id).unwrap();
        if let Node::Group { children, .. } = parent_node {
            assert!(children.contains(&child_id));
        } else {
            panic!("Parent should be a group");
        }
    }

    #[test]
    fn test_document_add_child_nonexistent_parent() {
        let mut doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);
        let child_id = NodeId::generate();
        let child = Node::Raster {
            properties: NodeProperties::new(child_id, NodeType::Raster, "Child".to_string()),
            bounds: Rect::from_size(Size::new(50, 50)),
            color_space: ColorSpace::Srgb,
        };

        doc.add_node(child).unwrap();

        let result = doc.add_child(NodeId::generate(), child_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_document_add_child_to_non_group() {
        let mut doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);

        let parent_id = NodeId::generate();
        let child_id = NodeId::generate();

        let parent = Node::Raster {
            properties: NodeProperties::new(parent_id, NodeType::Raster, "Parent".to_string()),
            bounds: Rect::from_size(Size::new(100, 100)),
            color_space: ColorSpace::Srgb,
        };

        let child = Node::Raster {
            properties: NodeProperties::new(child_id, NodeType::Raster, "Child".to_string()),
            bounds: Rect::from_size(Size::new(50, 50)),
            color_space: ColorSpace::Srgb,
        };

        doc.add_node(parent).unwrap();
        doc.add_node(child).unwrap();

        let result = doc.add_child(parent_id, child_id);
        assert!(result.is_err());
    }
}
