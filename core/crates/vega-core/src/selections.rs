//! Selection tools
//!
//! Provides selection tools for defining regions of interest in images.

use crate::color::types::rgba::RGBA8;
use crate::document::ids::structure::NodeId;
use crate::document::model::document::Document;
use crate::document::nodes::{Node, NodeType};
use crate::geometry::{Point, Rect, Size};
use crate::errors::DocumentError;
use crate::commands::base::Command;
use crate::tiles::{TileStorage, TileCoords};

/// Selection operation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    /// New selection (replaces existing).
    New,
    /// Add to existing selection.
    Add,
    /// Subtract from existing selection.
    Subtract,
    /// Intersect with existing selection.
    Intersect,
}

/// Selection shape type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionShape {
    /// Rectangular selection.
    Rectangle,
    /// Elliptical selection.
    Ellipse,
    /// Lasso (freehand) selection.
    Lasso,
    /// Magic wand (color-based) selection.
    MagicWand,
}

/// Rectangular selection parameters.
#[derive(Debug, Clone)]
pub struct RectangularSelectionParams {
    /// Selection rectangle.
    pub rect: Rect,
    /// Selection mode.
    pub mode: SelectionMode,
    /// Feather radius in pixels.
    pub feather: u32,
}

/// Elliptical selection parameters.
#[derive(Debug, Clone)]
pub struct EllipticalSelectionParams {
    /// Selection bounds (ellipse fits within).
    pub bounds: Rect,
    /// Selection mode.
    pub mode: SelectionMode,
    /// Feather radius in pixels.
    pub feather: u32,
}

/// Lasso selection parameters.
#[derive(Debug, Clone)]
pub struct LassoSelectionParams {
    /// Lasso points (closed polygon).
    pub points: Vec<Point>,
    /// Selection mode.
    pub mode: SelectionMode,
    /// Feather radius in pixels.
    pub feather: u32,
}

/// Magic wand selection parameters.
#[derive(Debug, Clone)]
pub struct MagicWandParams {
    /// Starting point.
    pub point: Point,
    /// Tolerance (0 to 255).
    pub tolerance: u8,
    /// Selection mode.
    pub mode: SelectionMode,
    /// Feather radius in pixels.
    pub feather: u32,
}

/// Selection mask data.
#[derive(Debug, Clone)]
pub struct SelectionMask {
    /// Mask data (0 = not selected, 255 = fully selected).
    pub data: Vec<u8>,
    /// Mask width.
    pub width: u32,
    /// Mask height.
    pub height: u32,
}

impl SelectionMask {
    /// Creates a new selection mask.
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            data: vec![0; size],
            width,
            height,
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

    /// Clears the mask (sets all to 0).
    pub fn clear(&mut self) {
        self.fill(0);
    }

    /// Inverts the mask.
    pub fn invert(&mut self) {
        for pixel in &mut self.data {
            *pixel = 255 - *pixel;
        }
    }

    /// Combines this mask with another using the specified mode.
    pub fn combine(&mut self, other: &SelectionMask, mode: SelectionMode) {
        let min_width = self.width.min(other.width);
        let min_height = self.height.min(other.height);

        for y in 0..min_height {
            for x in 0..min_width {
                let self_val = self.get(x, y);
                let other_val = other.get(x, y);

                let new_val = match mode {
                    SelectionMode::New => other_val,
                    SelectionMode::Add => self_val.saturating_add(other_val).min(255),
                    SelectionMode::Subtract => self_val.saturating_sub(other_val),
                    SelectionMode::Intersect => self_val.min(other_val),
                };

                self.set(x, y, new_val);
            }
        }
    }
}

/// Creates a rectangular selection mask.
pub fn create_rectangular_selection(
    width: u32,
    height: u32,
    params: RectangularSelectionParams,
) -> SelectionMask {
    let mut mask = SelectionMask::new(width, height);

    let rect = params.rect;
    let feather = params.feather as f32;

    for y in 0..height {
        for x in 0..width {
            // Check if point is within rectangle
            let within = x >= rect.x && x < rect.x + rect.width &&
                        y >= rect.y && y < rect.y + rect.height;

            if !within {
                continue;
            }

            // Apply feather if needed
            let value = if feather > 0.0 {
                let dx = if x < rect.x + feather {
                    (x as f32 - rect.x as f32) / feather
                } else if x >= rect.x + rect.width - feather {
                    (rect.x as f32 + rect.width as f32 - x as f32) / feather
                } else {
                    1.0
                };

                let dy = if y < rect.y + feather {
                    (y as f32 - rect.y as f32) / feather
                } else if y >= rect.y + rect.height - feather {
                    (rect.y as f32 + rect.height as f32 - y as f32) / feather
                } else {
                    1.0
                };

                let feather_factor = dx.min(dy).min(1.0);
                (feather_factor * 255.0) as u8
            } else {
                255
            };

            mask.set(x, y, value);
        }
    }

    mask
}

/// Creates an elliptical selection mask.
pub fn create_elliptical_selection(
    width: u32,
    height: u32,
    params: EllipticalSelectionParams,
) -> SelectionMask {
    let mut mask = SelectionMask::new(width, height);

    let bounds = params.bounds;
    let center_x = bounds.x as f32 + bounds.width as f32 / 2.0;
    let center_y = bounds.y as f32 + bounds.height as f32 / 2.0;
    let radius_x = bounds.width as f32 / 2.0;
    let radius_y = bounds.height as f32 / 2.0;
    let feather = params.feather as f32;

    for y in 0..height {
        for x in 0..width {
            // Calculate normalized distance from center
            let dx = (x as f32 - center_x) / radius_x;
            let dy = (y as f32 - center_y) / radius_y;
            let distance = (dx * dx + dy * dy).sqrt();

            // Check if point is within ellipse
            if distance > 1.0 {
                continue;
            }

            // Apply feather if needed
            let value = if feather > 0.0 {
                let feather_factor = (1.0 - distance).min(1.0);
                (feather_factor * 255.0) as u8
            } else {
                255
            };

            mask.set(x, y, value);
        }
    }

    mask
}

/// Creates a lasso (freehand) selection mask.
pub fn create_lasso_selection(
    width: u32,
    height: u32,
    params: LassoSelectionParams,
) -> SelectionMask {
    let mut mask = SelectionMask::new(width, height);

    if params.points.len() < 3 {
        return mask;
    }

    // Use scanline fill algorithm for polygon
    for y in 0..height {
        let mut intersections = Vec::new();

        // Find intersections with polygon edges
        for i in 0..params.points.len() {
            let p1 = params.points[i];
            let p2 = params.points[(i + 1) % params.points.len()];

            // Check if edge crosses this scanline
            if (p1.y <= y as i32 && p2.y > y as i32) ||
               (p2.y <= y as i32 && p1.y > y as i32) {
                // Calculate intersection x coordinate
                let t = (y as i32 - p1.y) as f32 / (p2.y - p1.y) as f32;
                let x = p1.x as f32 + t * (p2.x - p1.x) as f32;
                intersections.push(x);
            }
        }

        // Sort intersections
        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Fill between pairs of intersections
        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let start = intersections[i].max(0.0) as u32;
                let end = intersections[i + 1].min(width as f32) as u32;

                for x in start..end {
                    mask.set(x, y, 255);
                }
            }
        }
    }

    mask
}

/// Creates a magic wand selection mask (flood fill based on color similarity).
pub fn create_magic_wand_selection(
    document: &Document,
    layer_id: NodeId,
    tile_storage: &TileStorage,
    params: MagicWandParams,
) -> Result<SelectionMask, DocumentError> {
    let node = document.get_node(layer_id)
        .ok_or_else(|| DocumentError::NodeNotFound(layer_id))?;

    let bounds = match node {
        Node::Raster { bounds, .. } => *bounds,
        _ => return Err(DocumentError::InvalidLayerOperation(
            "Cannot select on non-raster layer".to_string(),
        )),
    };

    let mut mask = SelectionMask::new(bounds.width, bounds.height);

    // Get the color at the starting point
    let start_x = params.point.x as u32 - bounds.x;
    let start_y = params.point.y as u32 - bounds.y;

    if start_x >= bounds.width || start_y >= bounds.height {
        return Ok(mask);
    }

    let start_coords = TileCoords::from_pixel_coords(
        start_x + bounds.x,
        start_y + bounds.y,
        tile_storage.tile_size(),
    );

    let start_color = if let Some(handle) = tile_storage.get(start_coords) {
        if let Some(tile) = tile_storage.get_tile(handle) {
            let local_x = start_x % tile_storage.tile_size();
            let local_y = start_y % tile_storage.tile_size();
            tile.get_pixel(local_x, local_y)
        } else {
            RGBA8::TRANSPARENT
        }
    } else {
        RGBA8::TRANSPARENT
    };

    // Flood fill algorithm
    let mut stack = vec![(start_x, start_y)];
    let mut visited = std::collections::HashSet::new();

    while let Some((x, y)) = stack.pop() {
        if x >= bounds.width || y >= bounds.height {
            continue;
        }

        let key = (x, y);
        if visited.contains(&key) {
            continue;
        }
        visited.insert(key);

        // Get current pixel color
        let coords = TileCoords::from_pixel_coords(
            x + bounds.x,
            y + bounds.y,
            tile_storage.tile_size(),
        );

        let current_color = if let Some(handle) = tile_storage.get(coords) {
            if let Some(tile) = tile_storage.get_tile(handle) {
                let local_x = x % tile_storage.tile_size();
                let local_y = y % tile_storage.tile_size();
                tile.get_pixel(local_x, local_y)
            } else {
                RGBA8::TRANSPARENT
            }
        } else {
            RGBA8::TRANSPARENT
        };

        // Check color similarity
        if color_distance(start_color, current_color) <= params.tolerance as f32 {
            mask.set(x, y, 255);

            // Add neighbors to stack
            if x > 0 {
                stack.push((x - 1, y));
            }
            if x < bounds.width - 1 {
                stack.push((x + 1, y));
            }
            if y > 0 {
                stack.push((x, y - 1));
            }
            if y < bounds.height - 1 {
                stack.push((x, y + 1));
            }
        }
    }

    Ok(mask)
}

/// Calculates color distance for magic wand.
fn color_distance(c1: RGBA8, c2: RGBA8) -> f32 {
    let dr = (c1.r as f32 - c2.r as f32).abs();
    let dg = (c1.g as f32 - c2.g as f32).abs();
    let db = (c1.b as f32 - c2.b as f32).abs();
    let da = (c1.a as f32 - c2.a as f32).abs();

    (dr + dg + db + da) / 4.0
}

/// Command for rectangular selection.
pub struct RectangularSelectionCommand {
    layer_id: NodeId,
    params: RectangularSelectionParams,
    previous_mask: Option<SelectionMask>,
}

impl RectangularSelectionCommand {
    /// Creates a new rectangular selection command.
    pub fn new(layer_id: NodeId, params: RectangularSelectionParams) -> Self {
        Self {
            layer_id,
            params,
            previous_mask: None,
        }
    }

    /// Returns the layer ID.
    pub fn layer_id(&self) -> NodeId {
        self.layer_id
    }
}

impl Command for RectangularSelectionCommand {
    fn execute(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        // This is a simplified version - full implementation would manage selection state
        // For now, we'll return success
        Ok(())
    }

    fn undo(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        // Restore previous selection
        Ok(())
    }

    fn redo(&mut self, document: &mut Document) -> Result<(), DocumentError> {
        // Re-apply selection
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_mask() {
        let mut mask = SelectionMask::new(100, 100);
        assert_eq!(mask.width, 100);
        assert_eq!(mask.height, 100);
        assert_eq!(mask.get(50, 50), 0);

        mask.set(50, 50, 255);
        assert_eq!(mask.get(50, 50), 255);

        mask.clear();
        assert_eq!(mask.get(50, 50), 0);
    }

    #[test]
    fn test_selection_mask_invert() {
        let mut mask = SelectionMask::new(100, 100);
        mask.set(50, 50, 100);
        mask.invert();
        assert_eq!(mask.get(50, 50), 155);
    }

    #[test]
    fn test_rectangular_selection() {
        let params = RectangularSelectionParams {
            rect: Rect::new(10, 10, 50, 50),
            mode: SelectionMode::New,
            feather: 0,
        };

        let mask = create_rectangular_selection(100, 100, params);
        assert_eq!(mask.get(20, 20), 255);
        assert_eq!(mask.get(5, 5), 0);
    }

    #[test]
    fn test_elliptical_selection() {
        let params = EllipticalSelectionParams {
            bounds: Rect::new(10, 10, 50, 50),
            mode: SelectionMode::New,
            feather: 0,
        };

        let mask = create_elliptical_selection(100, 100, params);
        // Center should be selected
        assert!(mask.get(35, 35) > 0);
    }

    #[test]
    fn test_lasso_selection() {
        let points = vec![
            Point::new(10, 10),
            Point::new(60, 10),
            Point::new(60, 60),
            Point::new(10, 60),
        ];

        let params = LassoSelectionParams {
            points,
            mode: SelectionMode::New,
            feather: 0,
        };

        let mask = create_lasso_selection(100, 100, params);
        // Interior should be selected
        assert!(mask.get(30, 30) > 0);
    }

    #[test]
    fn test_color_distance() {
        let c1 = RGBA8::RED;
        let c2 = RGBA8::RED;
        assert_eq!(color_distance(c1, c2), 0.0);

        let c3 = RGBA8::BLUE;
        assert!(color_distance(c1, c3) > 0.0);
    }
}
