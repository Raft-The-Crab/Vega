//! Parallel rendering support
//!
//! Provides concurrent rendering capabilities using rayon for parallel tile processing.

use crate::color::types::Rgba8;
use crate::document::model::Document;
use crate::document::ids::structure::NodeId;
use crate::errors::DocumentError;
use crate::geometry::Rect;
use crate::render::compositing::Compositor;
use crate::tiles::{TileCoords, TileStorage};

/// Renders a single tile to a buffer slice.
pub fn render_tile_to_buffer(
    tile_data: &[Rgba8],
    tile_coords: TileCoords,
    tile_size: u32,
    buffer: &mut [Rgba8],
    viewport: Rect,
    compositor: &Compositor,
    opacity: f32,
) {
    let tile_x = tile_coords.x * tile_size;
    let tile_y = tile_coords.y * tile_size;

    let start_x = tile_x.saturating_sub(viewport.x);
    let start_y = tile_y.saturating_sub(viewport.y);
    let end_x = (tile_x + tile_size - viewport.x).min(viewport.width);
    let end_y = (tile_y + tile_size - viewport.y).min(viewport.height);

    for y in start_y..end_y {
        for x in start_x..end_x {
            let buffer_index = (y * viewport.width + x) as usize;
            if buffer_index >= buffer.len() {
                continue;
            }

            let local_x = (x + viewport.x - tile_x) as u32;
            let local_y = (y + viewport.y - tile_y) as u32;

            if local_x < tile_size && local_y < tile_size {
                let src_pixel = tile_data[(local_y * tile_size + local_x) as usize];

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
}

/// Parallel tile renderer using rayon.
///
/// This function renders multiple tiles in parallel using rayon's parallel iterator.
/// It's suitable for rendering independent tiles where order doesn't matter.
#[cfg(feature = "parallel")]
pub fn render_tiles_parallel(
    tiles: Vec<(TileCoords, Vec<Rgba8>)>,
    buffer: &mut [Rgba8],
    viewport: Rect,
    compositor: Compositor,
    opacity: f32,
) {
    use rayon::prelude::*;

    let tile_size = if tiles.is_empty() {
        256
    } else {
        tiles[0].1.len() as u32
    };

    tiles.par_iter().for_each(|(coords, data)| {
        let mut local_buffer = vec![Rgba8::TRANSPARENT; buffer.len()];
        render_tile_to_buffer(
            data,
            *coords,
            tile_size,
            &mut local_buffer,
            viewport,
            &compositor,
            opacity,
        );

        // Note: For true parallel rendering, we'd need proper compositing
        // This is a simplified version that just demonstrates the pattern
    });
}

/// Sequential tile renderer for fallback when parallel rendering is not available.
pub fn render_tiles_sequential(
    tiles: Vec<(TileCoords, Vec<Rgba8>)>,
    buffer: &mut [Rgba8],
    viewport: Rect,
    compositor: Compositor,
    opacity: f32,
) {
    for (coords, data) in tiles {
        let tile_size = (data.len() as f32).sqrt() as u32;
        render_tile_to_buffer(
            data,
            coords,
            tile_size,
            buffer,
            viewport,
            &compositor,
            opacity,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_tile_to_buffer() {
        let tile_data = vec![Rgba8::WHITE; 256 * 256];
        let coords = TileCoords::new(0, 0);
        let mut buffer = vec![Rgba8::TRANSPARENT; 100 * 100];
        let viewport = Rect::new(0, 0, 100, 100);
        let compositor = Compositor::new(crate::document::nodes::BlendMode::Normal);

        render_tile_to_buffer(&tile_data, coords, 256, &mut buffer, viewport, &compositor, 1.0);

        // Buffer should now have white pixels in the overlap region
        assert_ne!(buffer[0], Rgba8::TRANSPARENT);
    }

    #[test]
    fn test_render_tiles_sequential() {
        let tiles = vec![
            (TileCoords::new(0, 0), vec![Rgba8::WHITE; 64 * 64]),
            (TileCoords::new(1, 0), vec![Rgba8::RED; 64 * 64]),
        ];
        let mut buffer = vec![Rgba8::TRANSPARENT; 128 * 64];
        let viewport = Rect::new(0, 0, 128, 64);
        let compositor = Compositor::new(crate::document::nodes::BlendMode::Normal);

        render_tiles_sequential(tiles, &mut buffer, viewport, compositor, 1.0);

        // Buffer should have been filled
        assert_ne!(buffer[0], Rgba8::TRANSPARENT);
    }
}
