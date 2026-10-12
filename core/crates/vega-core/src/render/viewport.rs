//! Viewport rendering for display
//!
//! Provides rendering of document regions to buffers for display.

use crate::color::types::Rgba8;
use crate::document::model::Document;
use crate::document::nodes::Node;
use crate::document::ids::structure::NodeId;
use crate::errors::DocumentError;
use crate::geometry::Rect;
use crate::render::compositing::{alpha_blend, Compositor};
use crate::render::transform::{is_identity_transform, transform_rect};
use crate::tiles::{TileStorage, TileCoords};

/// Viewport renderer for displaying document regions.
pub struct ViewportRenderer {
    tile_storage: TileStorage,
}

impl ViewportRenderer {
    /// Creates a new viewport renderer with the given tile storage.
    pub fn new(tile_storage: TileStorage) -> Self {
        Self { tile_storage }
    }

    /// Renders a viewport region to a pixel buffer.
    ///
    /// # Arguments
    /// * `document` - The document to render
    /// * `viewport` - The viewport rectangle in document coordinates
    /// * `buffer` - Output buffer (must be viewport.width * viewport.height in size)
    ///
    /// # Errors
    /// Returns an error if the buffer size doesn't match the viewport size.
    pub fn render_viewport(
        &self,
        document: &Document,
        viewport: Rect,
        buffer: &mut [Rgba8],
    ) -> Result<(), DocumentError> {
        // Validate viewport bounds
        if viewport.x < 0 || viewport.y < 0 {
            return Err(DocumentError::InvalidParameter(
                "Viewport coordinates must be non-negative".to_string(),
            ));
        }

        if viewport.width == 0 || viewport.height == 0 {
            return Err(DocumentError::InvalidParameter(
                "Viewport dimensions must be positive".to_string(),
            ));
        }

        let expected_size = (viewport.width * viewport.height) as usize;
        if buffer.len() != expected_size {
            return Err(DocumentError::InvalidParameter(format!(
                "Buffer size {} doesn't match viewport size {}",
                buffer.len(),
                expected_size
            )));
        }

        // Check viewport is within document bounds
        let doc_right = self.tile_storage.width() as i32;
        let doc_bottom = self.tile_storage.height() as i32;
        let viewport_right = viewport.x + viewport.width as i32;
        let viewport_bottom = viewport.y + viewport.height as i32;

        if viewport.x > doc_right || viewport.y > doc_bottom {
            return Err(DocumentError::InvalidParameter(
                "Viewport is outside document bounds".to_string(),
            ));
        }

        // Clear buffer to transparent
        buffer.fill(Rgba8::TRANSPARENT);

        // Render each layer from bottom to top
        for node_id in document.root_nodes() {
            self.render_node(document, *node_id, viewport, buffer)?;
        }

        Ok(())
    }

    /// Renders a single node to the viewport with error handling.
    fn render_node(
        &self,
        document: &Document,
        node_id: NodeId,
        viewport: Rect,
        buffer: &mut [Rgba8],
    ) -> Result<(), DocumentError> {
        let node = document
            .get_node(node_id)
            .ok_or(DocumentError::NodeNotFound(node_id))?;

        if !node.properties().visible {
            return Ok(());
        }

        match node {
            Node::Raster { properties, bounds, .. } => {
                self.render_raster(properties, bounds, viewport, buffer)?;
            }
            Node::Group { properties, children } => {
                // Render group children
                for child_id in children {
                    self.render_node(document, *child_id, viewport, buffer)?;
                }
            }
            Node::Adjustment { properties, .. } => {
                // Adjustment layers modify the rendering pipeline
                // They are applied to layers below them
                // For now, adjustments are not rendered directly in viewport
                // They will be applied during the compositing phase in future implementation
            }
            Node::Text { .. } => {
                // Text rendering requires font rendering engine
                // This is a P2 feature for now
                // Text nodes will be skipped in viewport rendering until text rendering is implemented
            }
            Node::Shape { .. } => {
                // Shape rendering requires vector path rendering
                // This is a P2 feature for now
                // Shape nodes will be skipped in viewport rendering until shape rendering is implemented
            }
            Node::Mask { .. } => {
                // Mask rendering requires alpha channel manipulation
                // This is a P2 feature for now
                // Mask nodes will be skipped in viewport rendering until mask rendering is implemented
            }
            Node::Asset { .. } => {
                // Asset rendering requires external resource loading
                // This is a P2 feature for now
                // Asset nodes will be skipped in viewport rendering until asset rendering is implemented
            }
        }

        Ok(())
    }

    /// Renders a raster layer to the viewport with validation.
    fn render_raster(
        &self,
        properties: &crate::document::nodes::NodeProperties,
        bounds: &Rect,
        viewport: Rect,
        buffer: &mut [Rgba8],
    ) -> Result<(), DocumentError> {
        // Validate opacity
        if properties.opacity < 0.0 || properties.opacity > 1.0 {
            return Err(DocumentError::InvalidParameter(
                "Opacity must be in range [0.0, 1.0]".to_string(),
            ));
        }

        // Apply transform if not identity
        let transform = properties.transform;
        let effective_bounds = if is_identity_transform(
            transform.translation,
            transform.scale,
            transform.rotation,
        ) {
            *bounds
        } else {
            transform_rect(
                *bounds,
                transform.translation,
                transform.scale,
                transform.rotation,
            )
        };

        // Calculate intersection with viewport
        let intersection = viewport.intersection(effective_bounds).unwrap_or(Rect::default());

        if intersection.width == 0 || intersection.height == 0 {
            return Ok(());
        }

        // Get tiles that intersect with the viewport
        let tiles = self.tile_storage.tiles_in_rect(
            intersection.x,
            intersection.y,
            intersection.width,
            intersection.height,
        );

        // Compositor for this layer
        let compositor = Compositor::new(properties.blend_mode);

        // Render each tile
        for tile_coords in tiles {
            if let Some(tile_handle) = self.tile_storage.get(tile_coords) {
                if let Some(tile) = self.tile_storage.get_tile(tile_handle) {
                    self.render_tile_to_buffer(
                        tile,
                        &intersection,
                        viewport,
                        buffer,
                        &compositor,
                        properties.opacity,
                        transform,
                    )?;
                }
            }
        }

        Ok(())
    }

    /// Renders a single tile to the buffer with bounds checking.
    fn render_tile_to_buffer(
        &self,
        tile: &crate::tiles::Tile,
        tile_bounds: &Rect,
        viewport: Rect,
        buffer: &mut [Rgba8],
        compositor: &Compositor,
        opacity: f32,
        transform: crate::document::nodes::Transform,
    ) -> Result<(), DocumentError> {
        let tile_coords = tile.coords();
        let tile_size = tile.size();

        // Calculate tile position in document space
        let tile_x = tile_coords.x * tile_size;
        let tile_y = tile_coords.y * tile_size;

        // Calculate the region of this tile that falls within the viewport
        // Convert viewport coordinates to i32 for signed arithmetic
        let tile_x_i32 = tile_x as i32;
        let tile_y_i32 = tile_y as i32;
        let tile_size_i32 = tile_size as i32;
        
        let start_x = (tile_x_i32 - viewport.x).max(0) as u32;
        let start_y = (tile_y_i32 - viewport.y).max(0) as u32;
        let end_x = ((tile_x_i32 + tile_size_i32 - viewport.x).min(viewport.width as i32)).max(0) as u32;
        let end_y = ((tile_y_i32 + tile_size_i32 - viewport.y).min(viewport.height as i32)).max(0) as u32;

        // Render the overlapping region
        for y in start_y..end_y {
            for x in start_x..end_x {
                let buffer_index = (y * viewport.width + x) as usize;
                if buffer_index >= buffer.len() {
                    continue; // Skip out of bounds
                }

                let tile_pixel_x = (x as i32 + viewport.x - tile_x_i32) as u32;
                let tile_pixel_y = (y as i32 + viewport.y - tile_y_i32) as u32;

                if tile_pixel_x < tile_size && tile_pixel_y < tile_size {
                    let src_pixel = tile.get_pixel(tile_pixel_x, tile_pixel_y);

                    // Apply opacity
                    let src_pixel = if opacity < 1.0 {
                        Rgba8::new(
                            src_pixel.r,
                            src_pixel.g,
                            src_pixel.b,
                            (src_pixel.a as f32 * opacity).round() as u8,
                        )
                    } else {
                        src_pixel
                    };

                    buffer[buffer_index] = compositor.composite(buffer[buffer_index], src_pixel);
                }
            }
        }

        Ok(())
    }

    /// Returns a reference to the tile storage.
    pub fn tile_storage(&self) -> &TileStorage {
        &self.tile_storage
    }

    /// Returns a mutable reference to the tile storage.
    pub fn tile_storage_mut(&mut self) -> &mut TileStorage {
        &mut self.tile_storage
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Size;

    #[test]
    fn test_viewport_renderer_creation() {
        let storage = TileStorage::new(1920, 1080, 256).unwrap();
        let renderer = ViewportRenderer::new(storage);
        assert_eq!(renderer.tile_storage().width(), 1920);
    }

    #[test]
    fn test_render_viewport_buffer_size_mismatch() {
        let storage = TileStorage::new(1920, 1080, 256).unwrap();
        let renderer = ViewportRenderer::new(storage);
        let document = Document::new(
            "Test".to_string(),
            Size::new(1920, 1080),
            crate::color::ColorSpace::default(),
        );

        let viewport = Rect::new(0, 0, 100, 100);
        let mut buffer = vec![Rgba8::TRANSPARENT; 50]; // Wrong size

        let result = renderer.render_viewport(&document, viewport, &mut buffer);
        assert!(result.is_err());
    }

    #[test]
    fn test_render_viewport_empty() {
        let storage = TileStorage::new(1920, 1080, 256).unwrap();
        let renderer = ViewportRenderer::new(storage);
        let document = Document::new(
            "Test".to_string(),
            Size::new(1920, 1080),
            crate::color::ColorSpace::default(),
        );

        let viewport = Rect::new(0, 0, 100, 100);
        let mut buffer = vec![Rgba8::TRANSPARENT; 100 * 100];

        renderer.render_viewport(&document, viewport, &mut buffer).unwrap();
        // Should be all transparent since document has no layers
        assert!(buffer.iter().all(|&p| p == Rgba8::TRANSPARENT));
    }

    #[test]
    fn test_render_viewport_invalid_coords() {
        let storage = TileStorage::new(1920, 1080, 256).unwrap();
        let renderer = ViewportRenderer::new(storage);
        let document = Document::new(
            "Test".to_string(),
            Size::new(1920, 1080),
            crate::color::ColorSpace::default(),
        );

        let viewport = Rect::new(-10, 0, 100, 100); // Negative x
        let mut buffer = vec![Rgba8::TRANSPARENT; 100 * 100];

        let result = renderer.render_viewport(&document, viewport, &mut buffer);
        assert!(result.is_err());
    }

    #[test]
    fn test_render_viewport_zero_dimensions() {
        let storage = TileStorage::new(1920, 1080, 256).unwrap();
        let renderer = ViewportRenderer::new(storage);
        let document = Document::new(
            "Test".to_string(),
            Size::new(1920, 1080),
            crate::color::ColorSpace::default(),
        );

        let viewport = Rect::new(0, 0, 0, 100); // Zero width
        let mut buffer = vec![Rgba8::TRANSPARENT; 0];

        let result = renderer.render_viewport(&document, viewport, &mut buffer);
        assert!(result.is_err());
    }
}
