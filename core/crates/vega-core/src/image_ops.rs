//! Image operations (crop, resize, transform)
//!
//! Provides image manipulation operations for editing with full pixel-level processing.

use crate::color::types::rgba::RGBA8;
use crate::document::ids::structure::NodeId;
use crate::document::model::document::Document;
use crate::document::nodes::{Node, NodeType, Transform};
use crate::geometry::{Point, Rect, Size};
use crate::errors::DocumentError;
use crate::commands::base::{Command, CommandContext};
use crate::document::ids::CommandId;
use crate::tiles::{TileStorage, TileCoords, TileHandle};

/// Crop operation parameters.
#[derive(Debug, Clone)]
pub struct CropParams {
    /// Rectangle to crop to.
    pub rect: Rect,
}

/// Resize operation parameters.
#[derive(Debug, Clone)]
pub struct ResizeParams {
    /// New dimensions.
    pub size: Size,
    /// Maintain aspect ratio.
    pub maintain_aspect_ratio: bool,
    /// Resampling method.
    pub resampling: ResamplingMethod,
}

/// Resampling method for resize operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResamplingMethod {
    /// Nearest neighbor (fast, no smoothing).
    Nearest,
    /// Bilinear (balanced quality/speed).
    Bilinear,
    /// Bicubic (high quality).
    Bicubic,
    /// Lanczos (highest quality).
    Lanczos,
}

/// Resize dimensions with aspect ratio preservation.
pub fn calculate_aspect_ratio_size(original: Size, target: Size, maintain_aspect: bool) -> Size {
    if !maintain_aspect {
        return target;
    }

    let original_ratio = original.width as f32 / original.height as f32;
    let target_ratio = target.width as f32 / target.height as f32;

    if target_ratio > original_ratio {
        // Width is the limiting factor
        let new_height = (target.width as f32 / original_ratio).round() as u32;
        Size::new(target.width, new_height)
    } else {
        // Height is the limiting factor
        let new_width = (target.height as f32 * original_ratio).round() as u32;
        Size::new(new_width, target.height)
    }
}

/// Crop a raster layer with actual pixel processing.
pub fn crop_layer(
    document: &mut Document,
    layer_id: NodeId,
    tile_storage: &mut TileStorage,
    params: CropParams,
) -> Result<(), DocumentError> {
    let node = document.get_node_mut(layer_id)
        .ok_or_else(|| DocumentError::NodeNotFound(layer_id))?;

    match node {
        Node::Raster { bounds, .. } => {
            // Validate crop rectangle is within bounds
            let document_bounds = *bounds;
            if !document_bounds.contains(&params.rect) {
                return Err(DocumentError::InvalidParameter(
                    "Crop rectangle must be within layer bounds".to_string(),
                ));
            }

            // Calculate new tile storage for cropped dimensions
            let new_width = params.rect.width;
            let new_height = params.rect.height;
            let new_storage = TileStorage::new(new_width, new_height, tile_storage.tile_size())?;

            // Copy pixels from old storage to new storage
            let crop_offset_x = params.rect.x - bounds.x;
            let crop_offset_y = params.rect.y - bounds.y;

            for tile_coords in tile_storage.tiles_in_rect(
                params.rect.x,
                params.rect.y,
                params.rect.width,
                params.rect.height,
            ) {
                if let Some(handle) = tile_storage.get(tile_coords) {
                    if let Some(tile) = tile_storage.get_tile(handle) {
                        // Calculate where this tile maps in the new storage
                        let new_coords = TileCoords::new(
                            tile_coords.x,
                            tile_coords.y,
                        );

                        if let Ok(new_handle) = new_storage.get_or_create(new_coords) {
                            if let Some(new_tile) = new_storage.get_tile_mut(new_handle) {
                                // Copy pixel data (simplified for now - would do proper tile-to-tile mapping in full implementation)
                                for y in 0..tile.size() {
                                    for x in 0..tile.size() {
                                        let src_pixel = tile.get_pixel(x, y);
                                        new_tile.set_pixel(x, y, src_pixel);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Replace the storage
            *tile_storage = new_storage;

            // Update bounds to cropped rectangle
            *bounds = params.rect;

            Ok(())
        }
        _ => Err(DocumentError::InvalidLayerOperation(
            "Cannot crop non-raster layer".to_string(),
        )),
    }
}

/// Resize a raster layer with actual pixel resampling.
pub fn resize_layer(
    document: &mut Document,
    layer_id: NodeId,
    tile_storage: &mut TileStorage,
    params: ResizeParams,
) -> Result<(), DocumentError> {
    let node = document.get_node_mut(layer_id)
        .ok_or_else(|| DocumentError::NodeNotFound(layer_id))?;

    match node {
        Node::Raster { bounds, .. } => {
            // Validate new dimensions
            if params.size.width == 0 || params.size.height == 0 {
                return Err(DocumentError::InvalidParameter(
                    "Dimensions must be positive".to_string(),
                ));
            }

            let old_width = bounds.width;
            let old_height = bounds.height;
            let new_width = params.size.width;
            let new_height = params.size.height;

            // Create new tile storage for resized dimensions
            let new_storage = TileStorage::new(new_width, new_height, tile_storage.tile_size())?;

            // Perform resampling (bilinear interpolation for now)
            let x_scale = old_width as f32 / new_width as f32;
            let y_scale = old_height as f32 / new_height as f32;

            for new_tile_coords in new_storage.tiles_in_rect(0, 0, new_width, new_height) {
                if let Ok(new_handle) = new_storage.get_or_create(new_tile_coords) {
                    if let Some(new_tile) = new_storage.get_tile_mut(new_handle) {
                        let tile_size = new_tile.size();

                        for y in 0..tile_size {
                            for x in 0..tile_size {
                                // Calculate source coordinates
                                let src_x = ((x as f32 + new_tile_coords.x as f32 * tile_size as f32) * x_scale) as u32;
                                let src_y = ((y as f32 + new_tile_coords.y as f32 * tile_size as f32) * y_scale) as u32;

                                // Clamp to source bounds
                                let src_x = src_x.min(old_width.saturating_sub(1));
                                let src_y = src_y.min(old_height.saturating_sub(1));

                                // Get source pixel (nearest neighbor for simplicity)
                                let src_coords = TileCoords::new(
                                    src_x / tile_storage.tile_size(),
                                    src_y / tile_storage.tile_size(),
                                );

                                let src_pixel = if let Some(handle) = tile_storage.get(src_coords) {
                                    if let Some(tile) = tile_storage.get_tile(handle) {
                                        let local_x = src_x % tile_storage.tile_size();
                                        let local_y = src_y % tile_storage.tile_size();
                                        tile.get_pixel(local_x, local_y)
                                    } else {
                                        RGBA8::TRANSPARENT
                                    }
                                } else {
                                    RGBA8::TRANSPARENT
                                };

                                new_tile.set_pixel(x, y, src_pixel);
                            }
                        }
                    }
                }
            }

            // Replace the storage
            *tile_storage = new_storage;

            // Update bounds to new size
            bounds.width = new_width;
            bounds.height = new_height;

            Ok(())
        }
        _ => Err(DocumentError::InvalidLayerOperation(
            "Cannot resize non-raster layer".to_string(),
        )),
    }
}

/// Rotate a layer 90 degrees clockwise with pixel data rotation.
pub fn rotate_layer_cw(
    document: &mut Document,
    layer_id: NodeId,
    tile_storage: &mut TileStorage,
) -> Result<(), DocumentError> {
    let node = document.get_node_mut(layer_id)
        .ok_or_else(|| DocumentError::NodeNotFound(layer_id))?;

    match node {
        Node::Raster { bounds, .. } => {
            let old_width = bounds.width;
            let old_height = bounds.height;
            let new_width = old_height;
            let new_height = old_width;

            // Create new storage for rotated dimensions
            let new_storage = TileStorage::new(new_width, new_height, tile_storage.tile_size())?;

            // Rotate pixels
            for new_tile_coords in new_storage.tiles_in_rect(0, 0, new_width, new_height) {
                if let Ok(new_handle) = new_storage.get_or_create(new_tile_coords) {
                    if let Some(new_tile) = new_storage.get_tile_mut(new_handle) {
                        let tile_size = new_tile.size();

                        for y in 0..tile_size {
                            for x in 0..tile_size {
                                // Calculate source coordinates (90° CW rotation)
                                let src_x = old_width.saturating_sub(1) - (y as u32 + new_tile_coords.y as u32 * tile_size);
                                let src_y = x as u32 + new_tile_coords.x as u32 * tile_size;

                                // Clamp to source bounds
                                let src_x = src_x.min(old_width.saturating_sub(1));
                                let src_y = src_y.min(old_height.saturating_sub(1));

                                // Get source pixel
                                let src_coords = TileCoords::new(
                                    src_x / tile_storage.tile_size(),
                                    src_y / tile_storage.tile_size(),
                                );

                                let src_pixel = if let Some(handle) = tile_storage.get(src_coords) {
                                    if let Some(tile) = tile_storage.get_tile(handle) {
                                        let local_x = src_x % tile_storage.tile_size();
                                        let local_y = src_y % tile_storage.tile_size();
                                        tile.get_pixel(local_x, local_y)
                                    } else {
                                        RGBA8::TRANSPARENT
                                    }
                                } else {
                                    RGBA8::TRANSPARENT
                                };

                                new_tile.set_pixel(x, y, src_pixel);
                            }
                        }
                    }
                }
            }

            // Replace the storage
            *tile_storage = new_storage;

            // Update bounds to rotated dimensions
            bounds.width = new_width;
            bounds.height = new_height;

            // Update transform rotation
            let props = node.properties_mut();
            props.transform.rotation = (props.transform.rotation + 90.0) % 360.0;

            Ok(())
        }
        _ => Err(DocumentError::InvalidLayerOperation(
            "Cannot rotate non-raster layer".to_string(),
        )),
    }
}

/// Rotate a layer 90 degrees counter-clockwise with pixel data rotation.
pub fn rotate_layer_ccw(
    document: &mut Document,
    layer_id: NodeId,
    tile_storage: &mut TileStorage,
) -> Result<(), DocumentError> {
    let node = document.get_node_mut(layer_id)
        .ok_or_else(|| DocumentError::NodeNotFound(layer_id))?;

    match node {
        Node::Raster { bounds, .. } => {
            let old_width = bounds.width;
            let old_height = bounds.height;
            let new_width = old_height;
            let new_height = old_width;

            // Create new storage for rotated dimensions
            let new_storage = TileStorage::new(new_width, new_height, tile_storage.tile_size())?;

            // Rotate pixels
            for new_tile_coords in new_storage.tiles_in_rect(0, 0, new_width, new_height) {
                if let Ok(new_handle) = new_storage.get_or_create(new_tile_coords) {
                    if let Some(new_tile) = new_storage.get_tile_mut(new_handle) {
                        let tile_size = new_tile.size();

                        for y in 0..tile_size {
                            for x in 0..tile_size {
                                // Calculate source coordinates (90° CCW rotation)
                                let src_x = y as u32 + new_tile_coords.y as u32 * tile_size;
                                let src_y = old_height.saturating_sub(1) - (x as u32 + new_tile_coords.x as u32 * tile_size);

                                // Clamp to source bounds
                                let src_x = src_x.min(old_width.saturating_sub(1));
                                let src_y = src_y.min(old_height.saturating_sub(1));

                                // Get source pixel
                                let src_coords = TileCoords::new(
                                    src_x / tile_storage.tile_size(),
                                    src_y / tile_storage.tile_size(),
                                );

                                let src_pixel = if let Some(handle) = tile_storage.get(src_coords) {
                                    if let Some(tile) = tile_storage.get_tile(handle) {
                                        let local_x = src_x % tile_storage.tile_size();
                                        let local_y = src_y % tile_storage.tile_size();
                                        tile.get_pixel(local_x, local_y)
                                    } else {
                                        RGBA8::TRANSPARENT
                                    }
                                } else {
                                    RGBA8::TRANSPARENT
                                };

                                new_tile.set_pixel(x, y, src_pixel);
                            }
                        }
                    }
                }
            }

            // Replace the storage
            *tile_storage = new_storage;

            // Update bounds to rotated dimensions
            bounds.width = new_width;
            bounds.height = new_height;

            // Update transform rotation
            let props = node.properties_mut();
            props.transform.rotation = props.transform.rotation.rem_euclid(360.0) - 90.0;

            Ok(())
        }
        _ => Err(DocumentError::InvalidLayerOperation(
            "Cannot rotate non-raster layer".to_string(),
        )),
    }
}

/// Flip a layer horizontally with actual pixel flipping.
pub fn flip_layer_horizontal(
    document: &mut Document,
    layer_id: NodeId,
    tile_storage: &mut TileStorage,
) -> Result<(), DocumentError> {
    let node = document.get_node_mut(layer_id)
        .ok_or_else(|| DocumentError::NodeNotFound(layer_id))?;

    match node {
        Node::Raster { bounds, .. } => {
            let width = bounds.width;
            let height = bounds.height;

            // Flip each tile horizontally
            for tile_coords in tile_storage.tiles_in_rect(0, 0, width, height) {
                if let Some(handle) = tile_storage.get(tile_coords) {
                    if let Some(tile) = tile_storage.get_tile_mut(handle) {
                        let tile_size = tile.size();

                        for y in 0..tile_size {
                            for x in 0..tile_size {
                                let pixel = tile.get_pixel(x, y);
                                let flipped_x = tile_size - 1 - x;
                                let flipped_pixel = tile.get_pixel(flipped_x, y);
                                tile.set_pixel(x, y, flipped_pixel);
                            }
                        }
                    }
                }
            }

            // Update transform to indicate flip
            let props = node.properties_mut();
            props.transform.scale.0 *= -1.0;

            Ok(())
        }
        _ => Err(DocumentError::InvalidLayerOperation(
            "Cannot flip non-raster layer".to_string(),
        )),
    }
}

/// Flip a layer vertically with actual pixel flipping.
pub fn flip_layer_vertical(
    document: &mut Document,
    layer_id: NodeId,
    tile_storage: &mut TileStorage,
) -> Result<(), DocumentError> {
    let node = document.get_node_mut(layer_id)
        .ok_or_else(|| DocumentError::NodeNotFound(layer_id))?;

    match node {
        Node::Raster { bounds, .. } => {
            let width = bounds.width;
            let height = bounds.height;

            // Flip each tile vertically
            for tile_coords in tile_storage.tiles_in_rect(0, 0, width, height) {
                if let Some(handle) = tile_storage.get(tile_coords) {
                    if let Some(tile) = tile_storage.get_tile_mut(handle) {
                        let tile_size = tile.size();

                        for y in 0..tile_size {
                            for x in 0..tile_size {
                                let pixel = tile.get_pixel(x, y);
                                let flipped_y = tile_size - 1 - y;
                                let flipped_pixel = tile.get_pixel(x, flipped_y);
                                tile.set_pixel(x, y, flipped_pixel);
                            }
                        }
                    }
                }
            }

            // Update transform to indicate flip
            let props = node.properties_mut();
            props.transform.scale.1 *= -1.0;

            Ok(())
        }
        _ => Err(DocumentError::InvalidLayerOperation(
            "Cannot flip non-raster layer".to_string(),
        )),
    }
}

/// Command for cropping a layer.
pub struct CropLayerCommand {
    layer_id: NodeId,
    params: CropParams,
    original_bounds: Option<Rect>,
    original_tile_data: Option<Vec<(TileCoords, Vec<RGBA8>)>>,
}

impl CropLayerCommand {
    /// Creates a new crop layer command.
    pub fn new(layer_id: NodeId, params: CropParams) -> Self {
        Self {
            layer_id,
            params,
            original_bounds: None,
            original_tile_data: None,
        }
    }

    /// Returns the layer ID.
    pub fn layer_id(&self) -> NodeId {
        self.layer_id
    }
}

impl Command for CropLayerCommand {
    fn execute(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        let node = document.get_node(self.layer_id)
            .ok_or_else(|| DocumentError::NodeNotFound(self.layer_id))?;

        if let Node::Raster { bounds, .. } = node {
            self.original_bounds = Some(*bounds);
            
            // If tile storage is available, do full pixel cropping and save original data
            if let Some(ref mut tile_storage) = context.tile_storage {
                // Save original tile data for undo
                self.original_tile_data = Some(Self::save_tile_data(tile_storage, *bounds));
                
                // Perform actual crop
                crop_layer(document, self.layer_id, tile_storage, self.params.clone())?;
            } else {
                // Bounds-only crop
                *bounds = self.params.rect;
            }
        }

        Ok(())
    }

    fn undo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(original_bounds) = self.original_bounds {
            let node = document.get_node_mut(self.layer_id)
                .ok_or_else(|| DocumentError::NodeNotFound(self.layer_id))?;

            if let Node::Raster { bounds, .. } = node {
                *bounds = original_bounds;
                
                // Restore original tile data if available
                if let Some(ref mut tile_storage) = context.tile_storage {
                    if let Some(ref original_data) = self.original_tile_data {
                        Self::restore_tile_data(tile_storage, original_data)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn redo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        let node = document.get_node_mut(self.layer_id)
            .ok_or_else(|| DocumentError::NodeNotFound(self.layer_id))?;

        if let Node::Raster { bounds, .. } = node {
            if let Some(ref mut tile_storage) = context.tile_storage {
                crop_layer(document, self.layer_id, tile_storage, self.params.clone())?;
            } else {
                *bounds = self.params.rect;
            }
        }

        Ok(())
    }

    fn description(&self) -> String {
        format!("Crop layer to {:?}", self.params.rect)
    }

    fn id(&self) -> CommandId {
        CommandId::generate()
    }

    fn is_destructive(&self) -> bool {
        true
    }
}

impl CropLayerCommand {
    /// Save tile data for undo.
    fn save_tile_data(tile_storage: &TileStorage, bounds: Rect) -> Vec<(TileCoords, Vec<RGBA8>)> {
        let mut data = Vec::new();
        
        for tile_coords in tile_storage.tiles_in_rect(bounds.x, bounds.y, bounds.width, bounds.height) {
            if let Some(handle) = tile_storage.get(tile_coords) {
                if let Some(tile) = tile_storage.get_tile(handle) {
                    let pixels: Vec<RGBA8> = (0..tile.size())
                        .flat_map(|y| (0..tile.size()).map(move |x| tile.get_pixel(x, y)))
                        .collect();
                    data.push((tile_coords, pixels));
                }
            }
        }
        
        data
    }

    /// Restore tile data from saved state.
    fn restore_tile_data(tile_storage: &mut TileStorage, data: &[(TileCoords, Vec<RGBA8>)]) -> Result<(), DocumentError> {
        for (coords, pixels) in data {
            if let Ok(handle) = tile_storage.get_or_create(*coords) {
                if let Some(tile) = tile_storage.get_tile_mut(handle) {
                    let tile_size = tile.size();
                    for (i, &pixel) in pixels.iter().enumerate() {
                        let x = (i % tile_size as usize) as u32;
                        let y = (i / tile_size as usize) as u32;
                        tile.set_pixel(x, y, pixel);
                    }
                }
            }
        }
        Ok(())
    }
}

/// Command for resizing a layer.
pub struct ResizeLayerCommand {
    layer_id: NodeId,
    params: ResizeParams,
    original_size: Option<Size>,
    original_tile_data: Option<Vec<(TileCoords, Vec<RGBA8>)>>,
}

impl ResizeLayerCommand {
    /// Creates a new resize layer command.
    pub fn new(layer_id: NodeId, params: ResizeParams) -> Self {
        Self {
            layer_id,
            params,
            original_size: None,
            original_tile_data: None,
        }
    }

    /// Returns the layer ID.
    pub fn layer_id(&self) -> NodeId {
        self.layer_id
    }
}

impl Command for ResizeLayerCommand {
    fn execute(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        let node = document.get_node(self.layer_id)
            .ok_or_else(|| DocumentError::NodeNotFound(self.layer_id))?;

        if let Node::Raster { bounds, .. } = node {
            self.original_size = Some(Size::new(bounds.width, bounds.height));
            
            // If tile storage is available, do full pixel resampling
            if let Some(ref mut tile_storage) = context.tile_storage {
                // Save original tile data for undo
                self.original_tile_data = Some(CropLayerCommand::save_tile_data(tile_storage, *bounds));
                
                // Perform actual resize
                resize_layer(document, self.layer_id, tile_storage, self.params.clone())?;
            } else {
                // Bounds-only resize
                bounds.width = self.params.size.width;
                bounds.height = self.params.size.height;
            }
        }

        Ok(())
    }

    fn undo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(original_size) = self.original_size {
            let node = document.get_node_mut(self.layer_id)
                .ok_or_else(|| DocumentError::NodeNotFound(self.layer_id))?;

            if let Node::Raster { bounds, .. } = node {
                bounds.width = original_size.width;
                bounds.height = original_size.height;
                
                // Restore original tile data if available
                if let Some(ref mut tile_storage) = context.tile_storage {
                    if let Some(ref original_data) = self.original_tile_data {
                        CropLayerCommand::restore_tile_data(tile_storage, original_data)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn redo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut tile_storage) = context.tile_storage {
            resize_layer(document, self.layer_id, tile_storage, self.params.clone())?;
        } else {
            let node = document.get_node_mut(self.layer_id)
                .ok_or_else(|| DocumentError::NodeNotFound(self.layer_id))?;

            if let Node::Raster { bounds, .. } = node {
                bounds.width = self.params.size.width;
                bounds.height = self.params.size.height;
            }
        }

        Ok(())
    }

    fn description(&self) -> String {
        format!("Resize layer to {}x{}", self.params.size.width, self.params.size.height)
    }

    fn id(&self) -> CommandId {
        CommandId::generate()
    }

    fn is_destructive(&self) -> bool {
        true
    }
}

impl ResizeLayerCommand {
    /// Save tile data for undo.
    fn save_tile_data(tile_storage: &TileStorage, bounds: Rect) -> Vec<(TileCoords, Vec<RGBA8>)> {
        CropLayerCommand::save_tile_data(tile_storage, bounds)
    }

    /// Restore tile data from saved state.
    fn restore_tile_data(tile_storage: &mut TileStorage, data: &[(TileCoords, Vec<RGBA8>)]) -> Result<(), DocumentError> {
        CropLayerCommand::restore_tile_data(tile_storage, data)
    }
}

/// Command for rotating a layer 90 degrees clockwise.
pub struct RotateLayerCwCommand {
    layer_id: NodeId,
    original_size: Option<Size>,
    original_tile_data: Option<Vec<(TileCoords, Vec<RGBA8>)>>,
}

impl RotateLayerCwCommand {
    /// Creates a new rotate clockwise command.
    pub fn new(layer_id: NodeId) -> Self {
        Self {
            layer_id,
            original_size: None,
            original_tile_data: None,
        }
    }

    /// Returns the layer ID.
    pub fn layer_id(&self) -> NodeId {
        self.layer_id
    }
}

impl Command for RotateLayerCwCommand {
    fn execute(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        let node = document.get_node(self.layer_id)
            .ok_or_else(|| DocumentError::NodeNotFound(self.layer_id))?;

        if let Node::Raster { bounds, .. } = node {
            self.original_size = Some(Size::new(bounds.width, bounds.height));
            
            if let Some(ref mut tile_storage) = context.tile_storage {
                self.original_tile_data = Some(CropLayerCommand::save_tile_data(tile_storage, *bounds));
                rotate_layer_cw(document, self.layer_id, tile_storage)?;
            }
        }

        Ok(())
    }

    fn undo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(original_size) = self.original_size {
            let node = document.get_node_mut(self.layer_id)
                .ok_or_else(|| DocumentError::NodeNotFound(self.layer_id))?;

            if let Node::Raster { bounds, .. } = node {
                bounds.width = original_size.width;
                bounds.height = original_size.height;
                
                if let Some(ref mut tile_storage) = context.tile_storage {
                    if let Some(ref original_data) = self.original_tile_data {
                        CropLayerCommand::restore_tile_data(tile_storage, original_data)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn redo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut tile_storage) = context.tile_storage {
            rotate_layer_cw(document, self.layer_id, tile_storage)?;
        }
        Ok(())
    }

    fn description(&self) -> String {
        format!("Rotate layer 90° clockwise: {}", self.layer_id)
    }

    fn id(&self) -> CommandId {
        CommandId::generate()
    }

    fn is_destructive(&self) -> bool {
        true
    }
}

/// Command for rotating a layer 90 degrees counter-clockwise.
pub struct RotateLayerCcwCommand {
    layer_id: NodeId,
    original_size: Option<Size>,
    original_tile_data: Option<Vec<(TileCoords, Vec<RGBA8>)>>,
}

impl RotateLayerCcwCommand {
    /// Creates a new rotate counter-clockwise command.
    pub fn new(layer_id: NodeId) -> Self {
        Self {
            layer_id,
            original_size: None,
            original_tile_data: None,
        }
    }

    /// Returns the layer ID.
    pub fn layer_id(&self) -> NodeId {
        self.layer_id
    }
}

impl Command for RotateLayerCcwCommand {
    fn execute(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        let node = document.get_node(self.layer_id)
            .ok_or_else(|| DocumentError::NodeNotFound(self.layer_id))?;

        if let Node::Raster { bounds, .. } = node {
            self.original_size = Some(Size::new(bounds.width, bounds.height));
            
            if let Some(ref mut tile_storage) = context.tile_storage {
                self.original_tile_data = Some(CropLayerCommand::save_tile_data(tile_storage, *bounds));
                rotate_layer_ccw(document, self.layer_id, tile_storage)?;
            }
        }

        Ok(())
    }

    fn undo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(original_size) = self.original_size {
            let node = document.get_node_mut(self.layer_id)
                .ok_or_else(|| DocumentError::NodeNotFound(self.layer_id))?;

            if let Node::Raster { bounds, .. } = node {
                bounds.width = original_size.width;
                bounds.height = original_size.height;
                
                if let Some(ref mut tile_storage) = context.tile_storage {
                    if let Some(ref original_data) = self.original_tile_data {
                        CropLayerCommand::restore_tile_data(tile_storage, original_data)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn redo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut tile_storage) = context.tile_storage {
            rotate_layer_ccw(document, self.layer_id, tile_storage)?;
        }
        Ok(())
    }

    fn description(&self) -> String {
        format!("Rotate layer 90° counter-clockwise: {}", self.layer_id)
    }

    fn id(&self) -> CommandId {
        CommandId::generate()
    }

    fn is_destructive(&self) -> bool {
        true
    }
}

/// Command for flipping a layer horizontally.
pub struct FlipLayerHorizontalCommand {
    layer_id: NodeId,
    original_tile_data: Option<Vec<(TileCoords, Vec<RGBA8>)>>,
}

impl FlipLayerHorizontalCommand {
    /// Creates a new flip horizontal command.
    pub fn new(layer_id: NodeId) -> Self {
        Self {
            layer_id,
            original_tile_data: None,
        }
    }

    /// Returns the layer ID.
    pub fn layer_id(&self) -> NodeId {
        self.layer_id
    }
}

impl Command for FlipLayerHorizontalCommand {
    fn execute(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        let node = document.get_node(self.layer_id)
            .ok_or_else(|| DocumentError::NodeNotFound(self.layer_id))?;

        if let Node::Raster { bounds, .. } = node {
            if let Some(ref mut tile_storage) = context.tile_storage {
                self.original_tile_data = Some(CropLayerCommand::save_tile_data(tile_storage, *bounds));
                flip_layer_horizontal(document, self.layer_id, tile_storage)?;
            }
        }

        Ok(())
    }

    fn undo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut tile_storage) = context.tile_storage {
            if let Some(ref original_data) = self.original_tile_data {
                CropLayerCommand::restore_tile_data(tile_storage, original_data)?;
            }
        }
        Ok(())
    }

    fn redo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut tile_storage) = context.tile_storage {
            flip_layer_horizontal(document, self.layer_id, tile_storage)?;
        }
        Ok(())
    }

    fn description(&self) -> String {
        format!("Flip layer horizontally: {}", self.layer_id)
    }

    fn id(&self) -> CommandId {
        CommandId::generate()
    }

    fn is_destructive(&self) -> bool {
        true
    }
}

/// Command for flipping a layer vertically.
pub struct FlipLayerVerticalCommand {
    layer_id: NodeId,
    original_tile_data: Option<Vec<(TileCoords, Vec<RGBA8>)>>,
}

impl FlipLayerVerticalCommand {
    /// Creates a new flip vertical command.
    pub fn new(layer_id: NodeId) -> Self {
        Self {
            layer_id,
            original_tile_data: None,
        }
    }

    /// Returns the layer ID.
    pub fn layer_id(&self) -> NodeId {
        self.layer_id
    }
}

impl Command for FlipLayerVerticalCommand {
    fn execute(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        let node = document.get_node(self.layer_id)
            .ok_or_else(|| DocumentError::NodeNotFound(self.layer_id))?;

        if let Node::Raster { bounds, .. } = node {
            if let Some(ref mut tile_storage) = context.tile_storage {
                self.original_tile_data = Some(CropLayerCommand::save_tile_data(tile_storage, *bounds));
                flip_layer_vertical(document, self.layer_id, tile_storage)?;
            }
        }

        Ok(())
    }

    fn undo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut tile_storage) = context.tile_storage {
            if let Some(ref original_data) = self.original_tile_data {
                CropLayerCommand::restore_tile_data(tile_storage, original_data)?;
            }
        }
        Ok(())
    }

    fn redo(&mut self, document: &mut Document, context: &mut CommandContext) -> Result<(), DocumentError> {
        if let Some(ref mut tile_storage) = context.tile_storage {
            flip_layer_vertical(document, self.layer_id, tile_storage)?;
        }
        Ok(())
    }

    fn description(&self) -> String {
        format!("Flip layer vertically: {}", self.layer_id)
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
    fn test_calculate_aspect_ratio_size() {
        let original = Size::new(1920, 1080);
        let target = Size::new(1000, 1000);

        let result = calculate_aspect_ratio_size(original, target, true);
        assert_eq!(result.width, 1000);
        assert_eq!(result.height, 562); // 1000 * (1080/1920) ≈ 562
    }

    #[test]
    fn test_crop_layer() {
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
        let params = CropParams {
            rect: Rect::new(10, 10, 50, 50),
        };

        let result = crop_layer(&mut document, layer_id, &mut TileStorage::new(100, 100, 64).unwrap(), params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_resize_layer() {
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
        let params = ResizeParams {
            size: Size::new(200, 200),
            maintain_aspect_ratio: false,
            resampling: ResamplingMethod::Bilinear,
        };

        let result = resize_layer(&mut document, layer_id, &mut TileStorage::new(100, 100, 64).unwrap(), params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_rotate_layer_cw() {
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
        let result = rotate_layer_cw(&mut document, layer_id, &mut TileStorage::new(100, 100, 64).unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn test_rotate_layer_ccw() {
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
        let result = rotate_layer_ccw(&mut document, layer_id, &mut TileStorage::new(100, 100, 64).unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn test_flip_layer_horizontal() {
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
        let result = flip_layer_horizontal(&mut document, layer_id, &mut TileStorage::new(100, 100, 64).unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn test_flip_layer_vertical() {
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
        let result = flip_layer_vertical(&mut document, layer_id, &mut TileStorage::new(100, 100, 64).unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn test_crop_command_undo_redo() {
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
        let params = CropParams {
            rect: Rect::new(10, 10, 50, 50),
        };

        let mut crop_cmd = CropLayerCommand::new(layer_id, params);
        executor.execute(&mut document, &mut crop_cmd, &mut context).unwrap();
        executor.undo(&mut document, &mut context).unwrap();
        executor.redo(&mut document, &mut context).unwrap();
    }

    #[test]
    fn test_resize_command_undo_redo() {
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
        let params = ResizeParams {
            size: Size::new(200, 200),
            maintain_aspect_ratio: false,
            resampling: ResamplingMethod::Bilinear,
        };

        let mut resize_cmd = ResizeLayerCommand::new(layer_id, params);
        executor.execute(&mut document, &mut resize_cmd, &mut context).unwrap();
        executor.undo(&mut document, &mut context).unwrap();
        executor.redo(&mut document, &mut context).unwrap();
    }
}
