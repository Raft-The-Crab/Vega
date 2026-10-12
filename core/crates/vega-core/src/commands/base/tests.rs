#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::ColorSpace;
    use crate::document::ids::structure::NodeId;
    use crate::document::nodes::{Node, NodeProperties, NodeType};
    use crate::document::Document;
    use crate::geometry::{Rect, Size};

    /// A simple test command that adds a node.
    struct AddNodeCommand {
        id: CommandId,
        node: Option<Node>,
    }

    impl AddNodeCommand {
        fn new(node: Node) -> Self {
            Self {
                id: CommandId::generate(),
                node: Some(node),
            }
        }
    }

    impl std::fmt::Debug for AddNodeCommand {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("AddNodeCommand")
                .field("id", &self.id)
                .field("has_node", &self.node.is_some())
                .finish()
        }
    }

    impl Command for AddNodeCommand {
        fn execute(&mut self, document: &mut Document, _context: &mut CommandContext) -> Result<(), DocumentError> {
            if let Some(node) = self.node.take() {
                document.add_node(node)?;
            }
            Ok(())
        }

        fn undo(&mut self, _document: &mut Document, _context: &mut CommandContext) -> Result<(), DocumentError> {
            // This is a simplified undo - in reality we'd need to store the node ID
            Ok(())
        }

        fn redo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
            // Re-execute the add
            self.execute(document, context)
        }

        fn description(&self) -> String {
            "Add node".to_string()
        }

        fn id(&self) -> CommandId {
            self.id
        }
    }

    #[test]
    fn test_command_executor_new() {
        let executor = CommandExecutor::new(50);
        assert_eq!(executor.history_len(), 0);
        assert!(!executor.can_undo());
        assert!(!executor.can_redo());
    }

    #[test]
    fn test_command_executor_default() {
        let executor = CommandExecutor::default();
        assert_eq!(executor.history_len(), 0);
    }

    #[test]
    fn test_command_executor_execute() {
        let mut executor = CommandExecutor::new(10);
        let mut doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);
        let mut context = CommandContext::default();

        let node_id = NodeId::generate();
        let node = Node::Raster {
            properties: NodeProperties::new(node_id, NodeType::Raster, "Layer 1".to_string()),
            bounds: Rect::from_size(Size::new(100, 100)),
            color_space: ColorSpace::Srgb,
        };

        let command = Box::new(AddNodeCommand::new(node));
        let result = executor.execute(&mut doc, command, &mut context);

        assert!(result.is_ok());
        assert_eq!(executor.history_len(), 1);
        assert!(executor.can_undo());
        assert!(!executor.can_redo());
    }

    #[test]
    fn test_command_executor_undo() {
        let mut executor = CommandExecutor::new(10);
        let mut doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);
        let mut context = CommandContext::default();

        let node_id = NodeId::generate();
        let node = Node::Raster {
            properties: NodeProperties::new(node_id, NodeType::Raster, "Layer 1".to_string()),
            bounds: Rect::from_size(Size::new(100, 100)),
            color_space: ColorSpace::Srgb,
        };

        let command = Box::new(AddNodeCommand::new(node));
        executor.execute(&mut doc, command, &mut context).unwrap();

        let result = executor.undo(&mut doc, &mut context);
        assert!(result.is_ok());
        assert!(!executor.can_undo());
        assert!(executor.can_redo());
    }

    #[test]
    fn test_command_executor_redo() {
        let mut executor = CommandExecutor::new(10);
        let mut doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);
        let mut context = CommandContext::default();

        let node_id = NodeId::generate();
        let node = Node::Raster {
            properties: NodeProperties::new(node_id, NodeType::Raster, "Layer 1".to_string()),
            bounds: Rect::from_size(Size::new(100, 100)),
            color_space: ColorSpace::Srgb,
        };

        let command = Box::new(AddNodeCommand::new(node));
        executor.execute(&mut doc, command, &mut context).unwrap();
        executor.undo(&mut doc, &mut context).unwrap();

        let result = executor.redo(&mut doc, &mut context);
        assert!(result.is_ok());
        assert!(executor.can_undo());
        assert!(!executor.can_redo());
    }

    #[test]
    fn test_command_executor_clear_history() {
        let mut executor = CommandExecutor::new(10);
        let mut doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);
        let mut context = CommandContext::default();

        let node_id = NodeId::generate();
        let node = Node::Raster {
            properties: NodeProperties::new(node_id, NodeType::Raster, "Layer 1".to_string()),
            bounds: Rect::from_size(Size::new(100, 100)),
            color_space: ColorSpace::Srgb,
        };

        let command = Box::new(AddNodeCommand::new(node));
        executor.execute(&mut doc, command, &mut context).unwrap();

        executor.clear_history();
        assert_eq!(executor.history_len(), 0);
        assert!(!executor.can_undo());
        assert!(!executor.can_redo());
    }

    #[test]
    fn test_command_executor_max_history() {
        let mut executor = CommandExecutor::new(3);
        let mut doc = Document::new("test".to_string(), Size::new(100, 100), ColorSpace::Srgb);
        let mut context = CommandContext::default();

        // Add more commands than max_history
        for i in 0..5 {
            let node_id = NodeId::generate();
            let node = Node::Raster {
                properties: NodeProperties::new(
                    node_id,
                    NodeType::Raster,
                    format!("Layer {}", i),
                ),
                bounds: Rect::from_size(Size::new(100, 100)),
                color_space: ColorSpace::Srgb,
            };

            let command = Box::new(AddNodeCommand::new(node));
            executor.execute(&mut doc, command, &mut context).unwrap();
        }

        // History should be limited to max_history
        assert_eq!(executor.history_len(), 3);
    }

    #[test]
    fn test_transaction_new() {
        let transaction = Transaction::new("Test Transaction".to_string());
        assert_eq!(transaction.len(), 0);
        assert!(transaction.is_empty());
    }

    #[test]
    fn test_transaction_add() {
        let mut transaction = Transaction::new("Test".to_string());
        let node_id = NodeId::generate();
        let node = Node::Raster {
            properties: NodeProperties::new(node_id, NodeType::Raster, "Layer".to_string()),
            bounds: Rect::from_size(Size::new(100, 100)),
            color_space: ColorSpace::Srgb,
        };

        transaction.add(Box::new(AddNodeCommand::new(node)));
        assert_eq!(transaction.len(), 1);
        assert!(!transaction.is_empty());
    }

    #[test]
    fn test_transaction_can_undo() {
        let mut transaction = Transaction::new("Test".to_string());
        assert!(transaction.can_undo());
    }
}
