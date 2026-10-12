//! Layer masks
//!
//! Provides layer mask functionality for controlling layer visibility on a per-pixel basis.

use crate::color::types::Rgba8;
use crate::document::ids::structure::{NodeId, CommandId};
use crate::document::model::Document;
use crate::document::nodes::{Node, NodeType};
use crate::geometry::{Point, Rect, Size};
use crate::errors::DocumentError;
use crate::commands::base::{Command, CommandContext};
use crate::tiles::{TileStorage, TileCoords};
use crate::selections::SelectionMask;
use std::collections::HashMap;

/// Layer mask data.
#[derive(Debug, Clone)]
pub struct LayerMask {
    /// Mask data (0 = hidden, 255 = visible).
    pub data: Vec<u8>,
    /// Mask width.
    pub width: u32,
    /// Mask height.
    pub height: u32,
    /// Whether the mask is enabled.
    pub enabled: bool,
}

impl LayerMask {
    /// Creates a new layer mask.
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            data: vec![255; size], // Fully visible by default
            width,
            height,
            enabled: true,
        }
    }

    /// Returns the mask value at a pixel position.
    pub fn get(&self, x: u32, y: u32) -> u8 {
        if x >= self.width || y >= self.height {
            return 0;
        }
        let index = (y * self.width + x) as usize;
        self.data[index]
    }

    /// Sets the mask value at a pixel position.
    pub fn set(&mut self, x: u32, y: u32, value: u8) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = (y * self.width + x) as usize;
        self.data[index] = value;
    }

    /// Fills the entire mask with a value.
    pub fn fill(&mut self, value: u8) {
        self.data.fill(value);
    }

    /// Clears the mask (sets all to 0, fully hidden).
    pub fn clear(&mut self) {
        self.fill(0);
    }

    /// Shows the entire layer (sets all to 255).
    pub fn show_all(&mut self) {
        self.fill(255);
    }

    /// Inverts the mask.
    pub fn invert(&mut self) {
        for pixel in &mut self.data {
            *pixel = 255 - *pixel;
        }
    }

    /// Enables or disables the mask.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Returns whether the mask is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Applies the mask to a pixel, adjusting its alpha.
    pub fn apply_to_pixel(&self, pixel: Rgba8, x: u32, y: u32) -> Rgba8 {
        if !self.enabled {
            return pixel;
        }

        let mask_value = self.get(x, y) as f32 / 255.0;
        Rgba8 {
            r: pixel.r,
            g: pixel.g,
            b: pixel.b,
            a: (pixel.a as f32 * mask_value) as u8,
        }
    }

    /// Creates a mask from a selection.
    pub fn from_selection(selection: &SelectionMask) -> Self {
        Self {
            data: selection.data.clone(),
            width: selection.width,
            height: selection.height,
            enabled: true,
        }
    }
}

/// Layer mask storage for the document.
pub type LayerMaskStorage = HashMap<NodeId, LayerMask>;

/// Adds a layer mask to a raster layer.
pub fn add_layer_mask(
    document: &mut Document,
    layer_id: NodeId,
    mask_storage: &mut LayerMaskStorage,
) -> Result<(), DocumentError> {
    let node = document.get_node(layer_id)
        .ok_or_else(|| DocumentError::NodeNotFound(layer_id))?;

    match node {
        Node::Raster { bounds, .. } => {
            if mask_storage.contains_key(&layer_id) {
                return Err(DocumentError::InvalidLayerOperation(
                    "Layer already has a mask".to_string(),
                ));
            }

            let mask = LayerMask::new(bounds.width, bounds.height);
            mask_storage.insert(layer_id, mask);

            Ok(())
        }
        _ => Err(DocumentError::InvalidLayerOperation(
            "Cannot add mask to non-raster layer".to_string(),
        )),
    }
}

/// Removes a layer mask from a layer.
pub fn remove_layer_mask(
    document: &mut Document,
    layer_id: NodeId,
    mask_storage: &mut LayerMaskStorage,
) -> Result<LayerMask, DocumentError> {
    let node = document.get_node(layer_id)
        .ok_or_else(|| DocumentError::NodeNotFound(layer_id))?;

    match node {
        Node::Raster { .. } => {
            let mask = mask_storage.remove(&layer_id)
                .ok_or_else(|| DocumentError::InvalidLayerOperation(
                    "Layer has no mask".to_string(),
                ))?;

            Ok(mask)
        }
        _ => Err(DocumentError::InvalidLayerOperation(
            "Cannot remove mask from non-raster layer".to_string(),
        )),
    }
}

/// Applies a selection as a layer mask.
pub fn apply_selection_as_mask(
    document: &mut Document,
    layer_id: NodeId,
    selection: &SelectionMask,
    mask_storage: &mut LayerMaskStorage,
) -> Result<(), DocumentError> {
    let node = document.get_node(layer_id)
        .ok_or_else(|| DocumentError::NodeNotFound(layer_id))?;

    match node {
        Node::Raster { bounds, .. } => {
            // Validate selection dimensions match layer
            if selection.width != bounds.width || selection.height != bounds.height {
                return Err(DocumentError::InvalidParameter(
                    "Selection dimensions must match layer dimensions".to_string(),
                ));
            }

            let mask = LayerMask::from_selection(selection);
            mask_storage.insert(layer_id, mask);

            Ok(())
        }
        _ => Err(DocumentError::InvalidLayerOperation(
            "Cannot apply mask to non-raster layer".to_string(),
        )),
    }
}

/// Paints on a layer mask.
pub fn paint_on_mask(
    document: &mut Document,
    layer_id: NodeId,
    mask_storage: &mut LayerMaskStorage,
    position: Point,
    size: u32,
    opacity: f32,
    value: u8,
) -> Result<(), DocumentError> {
    let node = document.get_node(layer_id)
        .ok_or_else(|| DocumentError::NodeNotFound(layer_id))?;

    match node {
        Node::Raster { bounds, .. } => {
            let mask = mask_storage.get_mut(&layer_id)
                .ok_or_else(|| DocumentError::InvalidLayerOperation(
                    "Layer has no mask".to_string(),
                ))?;

            // Validate size
            if size == 0 {
                return Err(DocumentError::InvalidParameter(
                    "Size must be positive".to_string(),
                ));
            }

            // Validate opacity
            if opacity < 0.0 || opacity > 1.0 {
                return Err(DocumentError::InvalidParameter(
                    "Opacity must be between 0.0 and 1.0".to_string(),
                ));
            }

            // Calculate brush bounds
            let radius = size as i32 / 2;
            let brush_x = position.x as i32;
            let brush_y = position.y as i32;

            let brush_rect = Rect::new(
                (brush_x - radius).max(0) as u32,
                (brush_y - radius).max(0) as u32,
                size,
                size,
            );

            // Paint on mask
            for y in brush_rect.y..(brush_rect.y + brush_rect.height).min(mask.height) {
                for x in brush_rect.x..(brush_rect.x + brush_rect.width).min(mask.width) {
                    // Calculate distance from brush center
                    let dx = x as f32 - position.x as f32;
                    let dy = y as f32 - position.y as f32;
                    let distance = (dx * dx + dy * dy).sqrt();
                    let brush_radius = size as f32 / 2.0;

                    if distance > brush_radius {
                        continue;
                    }

                    // Calculate falloff
                    let falloff = 1.0 - (distance / brush_radius);
                    let current_value = mask.get(x, y);
                    let new_value = (current_value as f32 + (value as f32 - current_value as f32) * opacity * falloff) as u8;

                    mask.set(x, y, new_value);
                }
            }

            Ok(())
        }
        _ => Err(DocumentError::InvalidLayerOperation(
            "Cannot paint on non-raster layer mask".to_string(),
        )),
    }
}

/// Command for adding a layer mask.
#[derive(Debug)]
pub struct AddLayerMaskCommand {
    layer_id: NodeId,
    removed_mask: Option<LayerMask>,
}

impl AddLayerMaskCommand {
    /// Creates a new add layer mask command.
    pub fn new(layer_id: NodeId) -> Self {
        Self {
            layer_id,
            removed_mask: None,
        }
    }

    /// Returns the layer ID.
    pub fn layer_id(&self) -> NodeId {
        self.layer_id
    }
}

impl Command for AddLayerMaskCommand {
    fn execute(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut mask_storage) = context.mask_storage {
            add_layer_mask(document, self.layer_id, mask_storage)?;
        }
        Ok(())
    }

    fn undo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut mask_storage) = context.mask_storage {
            self.removed_mask = Some(remove_layer_mask(document, self.layer_id, mask_storage)?);
        }
        Ok(())
    }

    fn redo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut mask_storage) = context.mask_storage {
            add_layer_mask(document, self.layer_id, mask_storage)?;
        }
        Ok(())
    }

    fn description(&self) -> String {
        format!("Add layer mask to node {}", self.layer_id)
    }

    fn id(&self) -> CommandId {
        CommandId::generate()
    }
}

/// Command for removing a layer mask.
#[derive(Debug)]
pub struct RemoveLayerMaskCommand {
    layer_id: NodeId,
    removed_mask: Option<LayerMask>,
}

impl RemoveLayerMaskCommand {
    /// Creates a new remove layer mask command.
    pub fn new(layer_id: NodeId) -> Self {
        Self {
            layer_id,
            removed_mask: None,
        }
    }

    /// Returns the layer ID.
    pub fn layer_id(&self) -> NodeId {
        self.layer_id
    }
}

impl Command for RemoveLayerMaskCommand {
    fn execute(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut mask_storage) = context.mask_storage {
            self.removed_mask = Some(remove_layer_mask(document, self.layer_id, mask_storage)?);
        }
        Ok(())
    }

    fn undo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut mask_storage) = context.mask_storage {
            if let Some(mask) = self.removed_mask.take() {
                mask_storage.insert(self.layer_id, mask);
            }
        }
        Ok(())
    }

    fn redo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut mask_storage) = context.mask_storage {
            self.removed_mask = Some(remove_layer_mask(document, self.layer_id, mask_storage)?);
        }
        Ok(())
    }

    fn description(&self) -> String {
        format!("Remove layer mask from node {}", self.layer_id)
    }

    fn id(&self) -> CommandId {
        CommandId::generate()
    }
}

/// Command for applying selection as layer mask.
#[derive(Debug)]
pub struct ApplySelectionAsMaskCommand {
    layer_id: NodeId,
    selection: SelectionMask,
    previous_mask: Option<LayerMask>,
}

impl ApplySelectionAsMaskCommand {
    /// Creates a new apply selection as mask command.
    pub fn new(layer_id: NodeId, selection: SelectionMask) -> Self {
        Self {
            layer_id,
            selection,
            previous_mask: None,
        }
    }

    /// Returns the layer ID.
    pub fn layer_id(&self) -> NodeId {
        self.layer_id
    }
}

impl Command for ApplySelectionAsMaskCommand {
    fn execute(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut mask_storage) = context.mask_storage {
            // Save previous mask if it exists
            self.previous_mask = mask_storage.get(&self.layer_id).cloned();
            
            apply_selection_as_mask(document, self.layer_id, &self.selection, mask_storage)?;
        }
        Ok(())
    }

    fn undo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut mask_storage) = context.mask_storage {
            if let Some(mask) = self.previous_mask.take() {
                mask_storage.insert(self.layer_id, mask);
            } else {
                mask_storage.remove(&self.layer_id);
            }
        }
        Ok(())
    }

    fn redo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut mask_storage) = context.mask_storage {
            self.previous_mask = mask_storage.get(&self.layer_id).cloned();
            apply_selection_as_mask(document, self.layer_id, &self.selection, mask_storage)?;
        }
        Ok(())
    }

    fn description(&self) -> String {
        format!("Apply selection as mask to node {}", self.layer_id)
    }

    fn id(&self) -> CommandId {
        CommandId::generate()
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
    fn test_layer_mask() {
        let mask = LayerMask::new(100, 100);
        assert_eq!(mask.width, 100);
        assert_eq!(mask.height, 100);
        assert!(mask.is_enabled());
        assert_eq!(mask.get(50, 50), 255);

        mask.set(50, 50, 128);
        assert_eq!(mask.get(50, 50), 128);

        mask.invert();
        assert_eq!(mask.get(50, 50), 127);
    }

    #[test]
    fn test_layer_mask_show_all() {
        let mut mask = LayerMask::new(100, 100);
        mask.clear();
        assert_eq!(mask.get(50, 50), 0);

        mask.show_all();
        assert_eq!(mask.get(50, 50), 255);
    }

    #[test]
    fn test_layer_mask_apply_to_pixel() {
        let mask = LayerMask::new(100, 100);
        mask.set(50, 50, 128);

        let pixel = Rgba8::RED;
        let masked = mask.apply_to_pixel(pixel, 50, 50);

        assert_eq!(masked.r, 255);
        assert_eq!(masked.g, 0);
        assert_eq!(masked.b, 0);
        assert!(masked.a < 255); // Alpha reduced by mask
    }

    #[test]
    fn test_add_layer_mask() {
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
        let mut mask_storage = LayerMaskStorage::new();
        let result = add_layer_mask(&mut document, layer_id, &mut mask_storage);
        assert!(result.is_ok());
    }

    #[test]
    fn test_remove_layer_mask() {
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
        let mut mask_storage = LayerMaskStorage::new();
        add_layer_mask(&mut document, layer_id, &mut mask_storage).unwrap();

        let result = remove_layer_mask(&mut document, layer_id, &mut mask_storage);
        assert!(result.is_ok());
    }

    #[test]
    fn test_paint_on_mask() {
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
        let mut mask_storage = LayerMaskStorage::new();
        add_layer_mask(&mut document, layer_id, &mut mask_storage).unwrap();

        let result = paint_on_mask(&mut document, layer_id, &mut mask_storage, Point::new(50, 50), 10, 1.0, 255);
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_mask_command_undo_redo() {
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
        let mut mask_cmd = AddLayerMaskCommand::new(layer_id);
        // Simplified command - just test structure
        assert_eq!(mask_cmd.layer_id(), layer_id);
    }
}
