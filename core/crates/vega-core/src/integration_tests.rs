//! Integration tests for P0-P1 functionality
//!
//! Tests end-to-end workflows combining document management, tiles, layers, and rendering.

use crate::color::ColorSpace;
use crate::commands::base::{CommandExecutor, CommandContext};
use crate::document::model::document::Document;
use crate::document::nodes::NodeType;
use crate::geometry::{Point, Rect, Size};
use crate::layers::CreateLayerCommand;
use crate::render::ViewportRenderer;
use crate::tiles::TileStorage;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_workflow() {
        // Create a document
        let mut document = Document::new(
            "Test Document".to_string(),
            Size::new(1920, 1080),
            ColorSpace::default(),
        );

        // Create tile storage
        let tile_storage = TileStorage::with_default_tile_size(1920, 1080).unwrap();
        assert_eq!(tile_storage.width(), 1920);
        assert_eq!(tile_storage.height(), 1080);

        // Create command executor
        let mut executor = CommandExecutor::new(100);
        let mut context = CommandContext::new();

        // Add a layer using command
        let mut create_cmd = CreateLayerCommand::new(None, "Background".to_string(), NodeType::Raster);
        executor.execute(&mut document, &mut create_cmd, &mut context).unwrap();
        assert!(create_cmd.layer_id().is_some());

        // Verify layer was added
        let layer_id = create_cmd.layer_id().unwrap();
        assert!(document.get_node(layer_id).is_some());

        // Undo the command
        executor.undo(&mut document, &mut context).unwrap();
        assert!(document.get_node(layer_id).is_none());

        // Redo the command
        executor.redo(&mut document, &mut context).unwrap();
        assert!(document.get_node(layer_id).is_some());

        // Create viewport renderer
        let renderer = ViewportRenderer::new(tile_storage);

        // Render viewport
        let viewport = Rect::new(0, 0, 100, 100);
        let mut buffer = vec![crate::color::RGBA8::TRANSPARENT; 100 * 100];
        renderer.render_viewport(&document, viewport, &mut buffer).unwrap();

        // Verify buffer was filled (should be transparent since layer has no pixel data)
        assert!(buffer.iter().all(|&p| p == crate::color::RGBA8::TRANSPARENT));
    }

    #[test]
    fn test_multiple_layers_workflow() {
        let mut document = Document::new(
            "Multi-Layer".to_string(),
            Size::new(800, 600),
            ColorSpace::default(),
        );

        let mut executor = CommandExecutor::new(100);
        let mut context = CommandContext::new();

        // Create multiple layers
        let mut cmd1 = CreateLayerCommand::new(None, "Layer 1".to_string(), NodeType::Raster);
        executor.execute(&mut document, &mut cmd1, &mut context).unwrap();

        let mut cmd2 = CreateLayerCommand::new(None, "Layer 2".to_string(), NodeType::Raster);
        executor.execute(&mut document, &mut cmd2, &mut context).unwrap();

        let mut cmd3 = CreateLayerCommand::new(None, "Layer 3".to_string(), NodeType::Raster);
        executor.execute(&mut document, &mut cmd3, &mut context).unwrap();

        // Verify all layers exist
        assert_eq!(document.node_count(), 3);

        // Undo one layer
        executor.undo(&mut document, &mut context).unwrap();
        assert_eq!(document.node_count(), 2);

        // Undo another
        executor.undo(&mut document, &mut context).unwrap();
        assert_eq!(document.node_count(), 1);

        // Redo both
        executor.redo(&mut document, &mut context).unwrap();
        executor.redo(&mut document, &mut context).unwrap();
        assert_eq!(document.node_count(), 3);
    }

    #[test]
    fn test_tile_operations_with_document() {
        let document = Document::new(
            "Tile Test".to_string(),
            Size::new(512, 512),
            ColorSpace::default(),
        );

        let mut tile_storage = TileStorage::new(512, 512, 256).unwrap();
        assert_eq!(tile_storage.tiles_x(), 2);
        assert_eq!(tile_storage.tiles_y(), 2);

        // Create tiles
        let coords1 = crate::tiles::TileCoords::new(0, 0);
        let handle1 = tile_storage.get_or_create(coords1).unwrap();
        assert!(tile_storage.get(coords1).is_some());

        let coords2 = crate::tiles::TileCoords::new(1, 1);
        let handle2 = tile_storage.get_or_create(coords2).unwrap();
        assert!(tile_storage.get(coords2).is_some());

        // Verify tile count
        assert_eq!(tile_storage.allocated_count(), 2);

        // Modify tile data
        if let Some(tile) = tile_storage.get_tile_mut(handle1) {
            tile.fill(crate::color::RGBA8::RED);
            assert_eq!(tile.get_pixel(0, 0), crate::color::RGBA8::RED);
        }

        // Get tiles in rectangle
        let tiles = tile_storage.tiles_in_rect(100, 100, 200, 200);
        assert!(!tiles.is_empty());
    }

    #[test]
    fn test_document_revision_tracking() {
        let mut document = Document::new(
            "Revision Test".to_string(),
            Size::new(800, 600),
            ColorSpace::default(),
        );

        let initial_revision = document.revision();

        // Modify document
        let mut executor = CommandExecutor::new(100);
        let mut context = CommandContext::new();
        let mut cmd = CreateLayerCommand::new(None, "Test".to_string(), NodeType::Raster);
        executor.execute(&mut document, &mut cmd, &mut context).unwrap();

        // Revision should have incremented
        assert!(document.revision() > initial_revision);

        // Create revision guard
        let guard = document.create_revision_guard();
        assert!(guard.is_valid());

        // Modify again
        let mut cmd2 = CreateLayerCommand::new(None, "Test 2".to_string(), NodeType::Raster);
        executor.execute(&mut document, &mut cmd2, &mut context).unwrap();

        // Guard should now be invalid
        assert!(!guard.is_valid());
    }

    #[test]
    fn test_resource_limits_enforcement() {
        // Test document dimension limits
        let result = Document::new(
            "Too Large".to_string(),
            Size::new(200_000, 100),
            ColorSpace::default(),
        );
        assert!(result.is_err());

        // Test tile size limits
        let result = TileStorage::new(100, 100, 32);
        assert!(result.is_err()); // Below MIN_TILE_SIZE

        let result = TileStorage::new(100, 100, 5000);
        assert!(result.is_err()); // Above MAX_TILE_SIZE
    }

    #[test]
    fn test_empty_document_rendering() {
        let document = Document::new(
            "Empty".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        let tile_storage = TileStorage::new(100, 100, 64).unwrap();
        let renderer = ViewportRenderer::new(tile_storage);

        let viewport = Rect::new(0, 0, 100, 100);
        let mut buffer = vec![crate::color::RGBA8::TRANSPARENT; 100 * 100];
        renderer.render_viewport(&document, viewport, &mut buffer).unwrap();

        // Should be all transparent
        assert!(buffer.iter().all(|&p| p == crate::color::RGBA8::TRANSPARENT));
    }

    #[test]
    fn test_zero_size_document() {
        let result = Document::new(
            "Zero Size".to_string(),
            Size::new(0, 100),
            ColorSpace::default(),
        );
        assert!(result.is_err());

        let result = Document::new(
            "Zero Size".to_string(),
            Size::new(100, 0),
            ColorSpace::default(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_maximum_document_size() {
        let result = Document::new(
            "Max Size".to_string(),
            Size::new(100_000, 100_000),
            ColorSpace::default(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_command_executor_zero_history() {
        let mut document = Document::new(
            "Zero History".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        let mut executor = CommandExecutor::new(0);
        let mut context = CommandContext::new(); // No history

        let mut cmd = CreateLayerCommand::new(None, "Test".to_string(), NodeType::Raster);
        executor.execute(&mut document, &mut cmd, &mut context).unwrap();

        // Should not be able to undo with zero history
        let result = executor.undo(&mut document, &mut context);
        assert!(result.is_err());
    }

    #[test]
    fn test_duplicate_layer_names() {
        let mut document = Document::new(
            "Duplicate Names".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        let mut executor = CommandExecutor::new(100);
        let mut context = CommandContext::new();

        let mut cmd1 = CreateLayerCommand::new(None, "Layer".to_string(), NodeType::Raster);
        executor.execute(&mut document, &mut cmd1, &mut context).unwrap();

        let mut cmd2 = CreateLayerCommand::new(None, "Layer".to_string(), NodeType::Raster);
        executor.execute(&mut document, &mut cmd2, &mut context).unwrap();

        // Both should exist despite same name
        assert_eq!(document.node_count(), 2);
    }

    #[test]
    fn test_tile_bounds_edge_cases() {
        let mut tile_storage = TileStorage::new(300, 300, 256).unwrap();

        // Test tile at (0, 0)
        let coords = crate::tiles::TileCoords::new(0, 0);
        let handle = tile_storage.get_or_create(coords).unwrap();
        assert!(tile_storage.get(coords).is_some());

        // Test tile at boundary
        let coords = crate::tiles::TileCoords::new(1, 1);
        let handle = tile_storage.get_or_create(coords).unwrap();
        assert!(tile_storage.get(coords).is_some());

        // Test out of bounds
        let coords = crate::tiles::TileCoords::new(10, 10);
        let result = tile_storage.get_or_create(coords);
        assert!(result.is_err());
    }

    #[test]
    fn test_viewport_clip_to_document() {
        let document = Document::new(
            "Clip Test".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        let tile_storage = TileStorage::new(100, 100, 64).unwrap();
        let renderer = ViewportRenderer::new(tile_storage);

        // Viewport larger than document
        let viewport = Rect::new(0, 0, 200, 200);
        let mut buffer = vec![crate::color::RGBA8::TRANSPARENT; 200 * 200];
        let result = renderer.render_viewport(&document, viewport, &mut buffer);
        assert!(result.is_err()); // Should fail due to bounds check
    }

    #[test]
    fn test_opacity_edge_cases() {
        let mut document = Document::new(
            "Opacity Test".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        let mut executor = CommandExecutor::new(100);
        let mut context = CommandContext::new();
        let mut cmd = CreateLayerCommand::new(None, "Test".to_string(), NodeType::Raster);
        executor.execute(&mut document, &mut cmd, &mut context).unwrap();

        let layer_id = cmd.layer_id().unwrap();
        if let Some(node) = document.get_node_mut(layer_id) {
            let props = node.properties_mut();
            // Test boundary values
            props.opacity = 0.0;
            assert_eq!(props.opacity, 0.0);

            props.opacity = 1.0;
            assert_eq!(props.opacity, 1.0);
        }
    }

    #[test]
    fn test_max_nodes_limit() {
        let mut document = Document::new(
            "Max Nodes".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        let mut executor = CommandExecutor::new(100);
        let mut context = CommandContext::new();

        // Try to create more nodes than allowed
        for i in 0..10001 {
            let mut cmd = CreateLayerCommand::new(
                None,
                format!("Layer {}", i),
                NodeType::Raster,
            );
            if i < 10000 {
                executor.execute(&mut document, &mut cmd, &mut context).unwrap();
            } else {
                // Should fail at 10001
                let result = executor.execute(&mut document, &mut cmd, &mut context);
                assert!(result.is_err());
            }
        }
    }

    #[test]
    fn test_name_length_limit() {
        let mut document = Document::new(
            "Name Length".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        let mut executor = CommandExecutor::new(100);
        let mut context = CommandContext::new();

        // Name at limit
        let long_name = "A".repeat(256);
        let mut cmd = CreateLayerCommand::new(None, long_name, NodeType::Raster);
        let result = executor.execute(&mut document, &mut cmd, &mut context);
        assert!(result.is_ok());

        // Name over limit
        let too_long_name = "A".repeat(257);
        let mut cmd = CreateLayerCommand::new(None, too_long_name, NodeType::Raster);
        let result = executor.execute(&mut document, &mut cmd, &mut context);
        assert!(result.is_err());
    }
}
