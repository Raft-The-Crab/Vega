//! Brush and eraser tools
//!
//! Provides painting tools for raster editing with full pixel-level processing.

use crate::color::types::Rgba8;
use crate::document::ids::structure::{NodeId, CommandId};
use crate::document::model::document::Document;
use crate::document::nodes::{Node, NodeType};
use crate::geometry::{Point, Rect, Size};
use crate::errors::DocumentError;
use crate::commands::base::{Command, CommandContext};
use crate::tiles::{TileStorage, TileCoords, TileHandle};

/// Brush tool parameters.
#[derive(Debug, Clone)]
pub struct BrushParams {
    /// Brush position.
    pub position: Point,
    /// Brush size in pixels.
    pub size: u32,
    /// Brush opacity (0.0 to 1.0).
    pub opacity: f32,
    /// Brush hardness (0.0 to 1.0).
    pub hardness: f32,
    /// Brush color.
    pub color: Rgba8,
}

/// Eraser tool parameters.
#[derive(Debug, Clone)]
pub struct EraserParams {
    /// Eraser position.
    pub position: Point,
    /// Eraser size in pixels.
    pub size: u32,
    /// Eraser opacity (0.0 to 1.0).
    pub opacity: f32,
    /// Eraser hardness (0.0 to 1.0).
    pub hardness: f32,
}

/// Paints with a brush on a raster layer.
pub fn paint_brush(
    document: &mut Document,
    layer_id: NodeId,
    tile_storage: &mut TileStorage,
    params: BrushParams,
) -> Result<(), DocumentError> {
    let node = document.get_node_mut(layer_id)
        .ok_or_else(|| DocumentError::NodeNotFound(layer_id))?;

    match node {
        Node::Raster { bounds, .. } => {
            // Validate brush size
            if params.size == 0 {
                return Err(DocumentError::InvalidParameter(
                    "Brush size must be positive".to_string(),
                ));
            }

            // Validate opacity
            if params.opacity < 0.0 || params.opacity > 1.0 {
                return Err(DocumentError::InvalidParameter(
                    "Opacity must be between 0.0 and 1.0".to_string(),
                ));
            }

            // Validate hardness
            if params.hardness < 0.0 || params.hardness > 1.0 {
                return Err(DocumentError::InvalidParameter(
                    "Hardness must be between 0.0 and 1.0".to_string(),
                ));
            }

            // Calculate brush bounds
            let brush_radius = params.size as i32 / 2;
            let brush_x = params.position.x as i32;
            let brush_y = params.position.y as i32;

            let brush_rect = Rect::new(
                (brush_x - brush_radius).max(0) as u32,
                (brush_y - brush_radius).max(0) as u32,
                params.size,
                params.size,
            );

            // Get tiles that intersect with brush area
            let intersecting_tiles = tile_storage.tiles_in_rect(
                brush_rect.x,
                brush_rect.y,
                brush_rect.width,
                brush_rect.height,
            );

            // Paint on each tile
            for tile_coords in intersecting_tiles {
                if let Some(handle) = tile_storage.get(tile_coords) {
                    if let Some(tile) = tile_storage.get_tile_mut(handle) {
                        paint_brush_on_tile(
                            tile,
                            tile_coords,
                            tile_storage.tile_size(),
                            bounds,
                            &params,
                        );
                    }
                }
            }

            Ok(())
        }
        _ => Err(DocumentError::InvalidLayerOperation(
            "Cannot paint on non-raster layer".to_string(),
        )),
    }
}

/// Paints with an eraser on a raster layer.
pub fn paint_eraser(
    document: &mut Document,
    layer_id: NodeId,
    tile_storage: &mut TileStorage,
    params: EraserParams,
) -> Result<(), DocumentError> {
    let node = document.get_node_mut(layer_id)
        .ok_or_else(|| DocumentError::NodeNotFound(layer_id))?;

    match node {
        Node::Raster { bounds, .. } => {
            // Validate eraser size
            if params.size == 0 {
                return Err(DocumentError::InvalidParameter(
                    "Eraser size must be positive".to_string(),
                ));
            }

            // Validate opacity
            if params.opacity < 0.0 || params.opacity > 1.0 {
                return Err(DocumentError::InvalidParameter(
                    "Opacity must be between 0.0 and 1.0".to_string(),
                ));
            }

            // Validate hardness
            if params.hardness < 0.0 || params.hardness > 1.0 {
                return Err(DocumentError::InvalidParameter(
                    "Hardness must be between 0.0 and 1.0".to_string(),
                ));
            }

            // Calculate eraser bounds
            let eraser_radius = params.size as i32 / 2;
            let eraser_x = params.position.x as i32;
            let eraser_y = params.position.y as i32;

            let eraser_rect = Rect::new(
                (eraser_x - eraser_radius).max(0) as u32,
                (eraser_y - eraser_radius).max(0) as u32,
                params.size,
                params.size,
            );

            // Get tiles that intersect with eraser area
            let intersecting_tiles = tile_storage.tiles_in_rect(
                eraser_rect.x,
                eraser_rect.y,
                eraser_rect.width,
                eraser_rect.height,
            );

            // Erase on each tile
            for tile_coords in intersecting_tiles {
                if let Some(handle) = tile_storage.get(tile_coords) {
                    if let Some(tile) = tile_storage.get_tile_mut(handle) {
                        paint_eraser_on_tile(
                            tile,
                            tile_coords,
                            tile_storage.tile_size(),
                            bounds,
                            &params,
                        );
                    }
                }
            }

            Ok(())
        }
        _ => Err(DocumentError::InvalidLayerOperation(
            "Cannot erase on non-raster layer".to_string(),
        )),
    }
}

/// Paints brush on a single tile.
fn paint_brush_on_tile(
    tile: &mut crate::tiles::Tile,
    tile_coords: TileCoords,
    tile_size: u32,
    layer_bounds: &Rect,
    params: &BrushParams,
) {
    let brush_radius = params.size as f32 / 2.0;
    let brush_x = params.position.x as f32;
    let brush_y = params.position.y as f32;

    let tile_offset_x = tile_coords.x * tile_size;
    let tile_offset_y = tile_coords.y * tile_size;

    for y in 0..tile_size {
        for x in 0..tile_size {
            // Calculate absolute pixel position
            let abs_x = tile_offset_x + x;
            let abs_y = tile_offset_y + y;

            // Check if pixel is within layer bounds
            if abs_x < layer_bounds.x || abs_x >= layer_bounds.x + layer_bounds.width {
                continue;
            }
            if abs_y < layer_bounds.y || abs_y >= layer_bounds.y + layer_bounds.height {
                continue;
            }

            // Calculate distance from brush center
            let dx = abs_x as f32 - brush_x;
            let dy = abs_y as f32 - brush_y;
            let distance = (dx * dx + dy * dy).sqrt();

            // Check if pixel is within brush radius
            if distance > brush_radius {
                continue;
            }

            // Calculate brush falloff based on hardness
            let falloff = if params.hardness >= 1.0 {
                1.0
            } else {
                let normalized_distance = distance / brush_radius;
                let edge = 1.0 - params.hardness;
                if normalized_distance < 1.0 - edge {
                    1.0
                } else {
                    1.0 - ((normalized_distance - (1.0 - edge)) / edge)
                }
            };

            // Get current pixel
            let current_pixel = tile.get_pixel(x, y);

            // Blend brush color with current pixel
            let blended = blend_color_with_opacity(
                current_pixel,
                params.color,
                params.opacity * falloff,
            );

            tile.set_pixel(x, y, blended);
        }
    }
}

/// Paints eraser on a single tile.
fn paint_eraser_on_tile(
    tile: &mut crate::tiles::Tile,
    tile_coords: TileCoords,
    tile_size: u32,
    layer_bounds: &Rect,
    params: &EraserParams,
) {
    let eraser_radius = params.size as f32 / 2.0;
    let eraser_x = params.position.x as f32;
    let eraser_y = params.position.y as f32;

    let tile_offset_x = tile_coords.x * tile_size;
    let tile_offset_y = tile_coords.y * tile_size;

    for y in 0..tile_size {
        for x in 0..tile_size {
            // Calculate absolute pixel position
            let abs_x = tile_offset_x + x;
            let abs_y = tile_offset_y + y;

            // Check if pixel is within layer bounds
            if abs_x < layer_bounds.x || abs_x >= layer_bounds.x + layer_bounds.width {
                continue;
            }
            if abs_y < layer_bounds.y || abs_y >= layer_bounds.y + layer_bounds.height {
                continue;
            }

            // Calculate distance from eraser center
            let dx = abs_x as f32 - eraser_x;
            let dy = abs_y as f32 - eraser_y;
            let distance = (dx * dx + dy * dy).sqrt();

            // Check if pixel is within eraser radius
            if distance > eraser_radius {
                continue;
            }

            // Calculate eraser falloff based on hardness
            let falloff = if params.hardness >= 1.0 {
                1.0
            } else {
                let normalized_distance = distance / eraser_radius;
                let edge = 1.0 - params.hardness;
                if normalized_distance < 1.0 - edge {
                    1.0
                } else {
                    1.0 - ((normalized_distance - (1.0 - edge)) / edge)
                }
            };

            // Get current pixel
            let current_pixel = tile.get_pixel(x, y);

            // Reduce alpha based on eraser opacity and falloff
            let eraser_strength = params.opacity * falloff;
            let new_alpha = (current_pixel.a as f32 * (1.0 - eraser_strength)) as u8;

            let erased = Rgba8 {
                r: current_pixel.r,
                g: current_pixel.g,
                b: current_pixel.b,
                a: new_alpha,
            };

            tile.set_pixel(x, y, erased);
        }
    }
}

/// Blends a color with current pixel based on opacity.
fn blend_color_with_opacity(
    current: Rgba8,
    new: Rgba8,
    opacity: f32,
) -> Rgba8 {
    if opacity >= 1.0 {
        return new;
    }
    if opacity <= 0.0 {
        return current;
    }

    let alpha = new.a as f32 * opacity;
    let inv_alpha = 1.0 - alpha / 255.0;

    let r = (current.r as f32 * inv_alpha + new.r as f32 * alpha / 255.0) as u8;
    let g = (current.g as f32 * inv_alpha + new.g as f32 * alpha / 255.0) as u8;
    let b = (current.b as f32 * inv_alpha + new.b as f32 * alpha / 255.0) as u8;
    let a = (current.a as f32 * inv_alpha + alpha) as u8;

    Rgba8 { r, g, b, a }
}

/// Command for brush painting.
pub struct BrushCommand {
    layer_id: NodeId,
    params: BrushParams,
    modified_tiles: Vec<(TileCoords, Vec<Rgba8>)>,
}

impl BrushCommand {
    /// Creates a new brush command.
    pub fn new(layer_id: NodeId, params: BrushParams) -> Self {
        Self {
            layer_id,
            params,
            modified_tiles: Vec::new(),
        }
    }

    /// Returns the layer ID.
    pub fn layer_id(&self) -> NodeId {
        self.layer_id
    }
}

impl Command for BrushCommand {
    fn execute(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut tile_storage) = context.tile_storage {
            // Save original tile state before painting
            let brush_radius = self.params.size as i32 / 2;
            let brush_x = self.params.position.x as i32;
            let brush_y = self.params.position.y as i32;

            let brush_rect = Rect::new(
                (brush_x - brush_radius).max(0) as u32,
                (brush_y - brush_radius).max(0) as u32,
                self.params.size,
                self.params.size,
            );

            for tile_coords in tile_storage.tiles_in_rect(
                brush_rect.x,
                brush_rect.y,
                brush_rect.width,
                brush_rect.height,
            ) {
                if let Some(handle) = tile_storage.get(tile_coords) {
                    if let Some(tile) = tile_storage.get_tile(handle) {
                        let tile_data = tile.data().to_vec();
                        self.modified_tiles.push((tile_coords, tile_data));
                    }
                }
            }

            // Paint the brush
            paint_brush(document, self.layer_id, tile_storage, self.params.clone())?;
        }

        Ok(())
    }

    fn undo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut tile_storage) = context.tile_storage {
            // Restore saved tile state
            for (tile_coords, tile_data) in &self.modified_tiles {
                if let Ok(handle) = tile_storage.get_or_create(*tile_coords) {
                    if let Some(tile) = tile_storage.get_tile_mut(handle) {
                        let tile_pixels = tile.data_mut();
                        tile_pixels.copy_from_slice(tile_data);
                    }
                }
            }
        }

        Ok(())
    }

    fn redo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut tile_storage) = context.tile_storage {
            paint_brush(document, self.layer_id, tile_storage, self.params.clone())?;
        }
        Ok(())
    }

    fn description(&self) -> String {
        format!("Brush at ({}, {})", self.params.position.x, self.params.position.y)
    }

    fn id(&self) -> CommandId {
        CommandId::generate()
    }

    fn is_destructive(&self) -> bool {
        true
    }
}

/// Command for eraser painting.
pub struct EraserCommand {
    layer_id: NodeId,
    params: EraserParams,
    modified_tiles: Vec<(TileCoords, Vec<Rgba8>)>,
}

impl EraserCommand {
    /// Creates a new eraser command.
    pub fn new(layer_id: NodeId, params: EraserParams) -> Self {
        Self {
            layer_id,
            params,
            modified_tiles: Vec::new(),
        }
    }

    /// Returns the layer ID.
    pub fn layer_id(&self) -> NodeId {
        self.layer_id
    }
}

impl Command for EraserCommand {
    fn execute(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut tile_storage) = context.tile_storage {
            // Save original tile state before erasing
            let eraser_radius = self.params.size as i32 / 2;
            let eraser_x = self.params.position.x as i32;
            let eraser_y = self.params.position.y as i32;

            let eraser_rect = Rect::new(
                (eraser_x - eraser_radius).max(0) as u32,
                (eraser_y - eraser_radius).max(0) as u32,
                self.params.size,
                self.params.size,
            );

            for tile_coords in tile_storage.tiles_in_rect(
                eraser_rect.x,
                eraser_rect.y,
                eraser_rect.width,
                eraser_rect.height,
            ) {
                if let Some(handle) = tile_storage.get(tile_coords) {
                    if let Some(tile) = tile_storage.get_tile(handle) {
                        let tile_data = tile.data().to_vec();
                        self.modified_tiles.push((tile_coords, tile_data));
                    }
                }
            }

            // Paint the eraser
            paint_eraser(document, self.layer_id, tile_storage, self.params.clone())?;
        }

        Ok(())
    }

    fn undo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut tile_storage) = context.tile_storage {
            // Restore saved tile state
            for (tile_coords, tile_data) in &self.modified_tiles {
                if let Ok(handle) = tile_storage.get_or_create(*tile_coords) {
                    if let Some(tile) = tile_storage.get_tile_mut(handle) {
                        let tile_pixels = tile.data_mut();
                        tile_pixels.copy_from_slice(tile_data);
                    }
                }
            }
        }

        Ok(())
    }

    fn redo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut tile_storage) = context.tile_storage {
            paint_eraser(document, self.layer_id, tile_storage, self.params.clone())?;
        }
        Ok(())
    }

    fn description(&self) -> String {
        format!("Eraser at ({}, {})", self.params.position.x, self.params.position.y)
    }

    fn id(&self) -> CommandId {
        CommandId::generate()
    }

    fn is_destructive(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::base::{CommandExecutor, CommandContext};
    use crate::layers::CreateLayerCommand;
    use crate::document::nodes::NodeType;
    use crate::color::ColorSpace;

    #[test]
    fn test_paint_brush() {
        let mut document = Document::new(
            "Test".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        let mut executor = CommandExecutor::new(100);
        let mut context = CommandContext::new();
        let mut cmd = CreateLayerCommand::new(None, "Layer".to_string(), NodeType::Raster);
        executor.execute(&mut document, &mut cmd, &mut context).unwrap();

        let layer_id = cmd.layer_id().unwrap();
        let params = BrushParams {
            position: Point::new(50, 50),
            size: 10,
            opacity: 1.0,
            hardness: 1.0,
            color: Rgba8::RED,
        };

        let result = paint_brush(&mut document, layer_id, &mut TileStorage::new(100, 100, 64).unwrap(), params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_paint_eraser() {
        let mut document = Document::new(
            "Test".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        let mut executor = CommandExecutor::new(100);
        let mut context = CommandContext::new();
        let mut cmd = CreateLayerCommand::new(None, "Layer".to_string(), NodeType::Raster);
        executor.execute(&mut document, &mut cmd, &mut context).unwrap();

        let layer_id = cmd.layer_id().unwrap();
        let params = EraserParams {
            position: Point::new(50, 50),
            size: 10,
            opacity: 1.0,
            hardness: 1.0,
        };

        let result = paint_eraser(&mut document, layer_id, &mut TileStorage::new(100, 100, 64).unwrap(), params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_brush_size_validation() {
        let mut document = Document::new(
            "Test".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        let mut executor = CommandExecutor::new(100);
        let mut context = CommandContext::new();
        let mut cmd = CreateLayerCommand::new(None, "Layer".to_string(), NodeType::Raster);
        executor.execute(&mut document, &mut cmd, &mut context).unwrap();

        let layer_id = cmd.layer_id().unwrap();
        let params = BrushParams {
            position: Point::new(50, 50),
            size: 0, // Invalid
            opacity: 1.0,
            hardness: 1.0,
            color: Rgba8::RED,
        };

        let result = paint_brush(&mut document, layer_id, &mut TileStorage::new(100, 100, 64).unwrap(), params);
        assert!(result.is_err());
    }

    #[test]
    fn test_brush_opacity_validation() {
        let mut document = Document::new(
            "Test".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        let mut executor = CommandExecutor::new(100);
        let mut context = CommandContext::new();
        let mut cmd = CreateLayerCommand::new(None, "Layer".to_string(), NodeType::Raster);
        executor.execute(&mut document, &mut cmd, &mut context).unwrap();

        let layer_id = cmd.layer_id().unwrap();
        let params = BrushParams {
            position: Point::new(50, 50),
            size: 10,
            opacity: 1.5, // Invalid
            hardness: 1.0,
            color: Rgba8::RED,
        };

        let result = paint_brush(&mut document, layer_id, &mut TileStorage::new(100, 100, 64).unwrap(), params);
        assert!(result.is_err());
    }

    #[test]
    fn test_blend_color_with_opacity() {
        let current = Rgba8::BLUE;
        let new = Rgba8::RED;
        let blended = blend_color_with_opacity(current, new, 0.5);

        // Result should be a blend of blue and red
        assert!(blended.r > current.r);
        assert!(blended.b < current.b);
    }
}
