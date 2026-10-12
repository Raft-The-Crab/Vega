#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_id_generation() {
        let id1 = DocumentId::generate();
        let id2 = DocumentId::generate();
        assert_ne!(id1, id2, "Generated IDs should be unique");
    }

    #[test]
    fn test_document_id_value() {
        let id = DocumentId::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn test_document_id_display() {
        let id = DocumentId::new(123);
        assert_eq!(format!("{}", id), "doc-123");
    }

    #[test]
    fn test_layer_id_generation() {
        let id1 = LayerId::generate();
        let id2 = LayerId::generate();
        assert_ne!(id1, id2, "Generated IDs should be unique");
    }

    #[test]
    fn test_layer_id_value() {
        let id = LayerId::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn test_layer_id_display() {
        let id = LayerId::new(456);
        assert_eq!(format!("{}", id), "layer-456");
    }

    #[test]
    fn test_node_id_generation() {
        let id1 = NodeId::generate();
        let id2 = NodeId::generate();
        assert_ne!(id1, id2, "Generated IDs should be unique");
    }

    #[test]
    fn test_node_id_value() {
        let id = NodeId::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn test_node_id_display() {
        let id = NodeId::new(789);
        assert_eq!(format!("{}", id), "node-789");
    }

    #[test]
    fn test_command_id_generation() {
        let id1 = CommandId::generate();
        let id2 = CommandId::generate();
        assert_ne!(id1, id2, "Generated IDs should be unique");
    }

    #[test]
    fn test_command_id_value() {
        let id = CommandId::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn test_command_id_display() {
        let id = CommandId::new(999);
        assert_eq!(format!("{}", id), "cmd-999");
    }

    #[test]
    fn test_id_hashing() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for _ in 0..100 {
            set.insert(DocumentId::generate());
        }
        assert_eq!(set.len(), 100, "All generated IDs should be unique");
    }
}
