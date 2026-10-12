//! Tile structure for pixel data
//!
//! Tiles are fixed-size blocks of pixel data that form the basic unit
//! of image storage in VEGA.

use crate::color::types::Rgba8;
use crate::limits;
use std::fmt;

/// Coordinates of a tile in the tile grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileCoords {
    /// X index of the tile in the grid.
    pub x: u32,
    /// Y index of the tile in the grid.
    pub y: u32,
}

impl TileCoords {
    /// Creates new tile coordinates.
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    /// Converts pixel coordinates to tile coordinates.
    pub fn from_pixel_coords(px: u32, py: u32, tile_size: u32) -> Self {
        Self {
            x: px / tile_size,
            y: py / tile_size,
        }
    }
}

/// Unique identifier for a tile within a storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileIndex(u64);

impl TileIndex {
    /// Creates a new tile index from a raw value.
    pub fn new(index: u64) -> Self {
        Self(index)
    }

    /// Returns the raw index value.
    pub fn value(self) -> u64 {
        self.0
    }
}

/// A tile containing pixel data.
///
/// Tiles are fixed-size blocks of pixels that can be efficiently cached,
/// transmitted, and processed independently.
pub struct Tile {
    /// Tile coordinates in the grid.
    coords: TileCoords,
    /// Size of the tile in pixels (usually power of 2).
    size: u32,
    /// Pixel data in row-major order.
    data: Vec<Rgba8>,
    /// Whether this tile has been modified since load.
    dirty: bool,
}

impl Tile {
    /// Creates a new empty tile with the given coordinates and size.
    ///
    /// # Panics
    /// Panics if size is not within [MIN_TILE_SIZE, MAX_TILE_SIZE].
    pub fn new(coords: TileCoords, size: u32) -> Self {
        assert!(
            size >= limits::MIN_TILE_SIZE && size <= limits::MAX_TILE_SIZE,
            "Tile size {} out of range [{}, {}]",
            size,
            limits::MIN_TILE_SIZE,
            limits::MAX_TILE_SIZE
        );

        let pixel_count = (size * size) as usize;
        let data = vec![Rgba8::TRANSPARENT; pixel_count];

        Self {
            coords,
            size,
            data,
            dirty: true,
        }
    }

    /// Creates a new tile with specific pixel data.
    ///
    /// # Panics
    /// Panics if data length doesn't match size squared.
    pub fn with_data(coords: TileCoords, size: u32, data: Vec<Rgba8>) -> Self {
        let expected_len = (size * size) as usize;
        assert_eq!(
            data.len(),
            expected_len,
            "Data length {} doesn't match tile size {}x{}",
            data.len(),
            size,
            size
        );

        Self {
            coords,
            size,
            data,
            dirty: true,
        }
    }

    /// Returns the tile coordinates.
    pub fn coords(&self) -> TileCoords {
        self.coords
    }

    /// Returns the tile size in pixels.
    pub fn size(&self) -> u32 {
        self.size
    }

    /// Returns whether the tile has been modified.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Marks the tile as clean (not modified).
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    /// Marks the tile as dirty (modified).
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Returns a reference to the pixel data.
    pub fn data(&self) -> &[Rgba8] {
        &self.data
    }

    /// Returns a mutable reference to the pixel data.
    pub fn data_mut(&mut self) -> &mut [Rgba8] {
        self.dirty = true;
        &mut self.data
    }

    /// Gets a pixel at the given local coordinates.
    ///
    /// # Panics
    /// Panics if x or y are out of bounds.
    pub fn get_pixel(&self, x: u32, y: u32) -> Rgba8 {
        assert!(x < self.size && y < self.size, "Pixel coordinates out of bounds");
        let index = (y * self.size + x) as usize;
        self.data[index]
    }

    /// Sets a pixel at the given local coordinates.
    ///
    /// # Panics
    /// Panics if x or y are out of bounds.
    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Rgba8) {
        assert!(x < self.size && y < self.size, "Pixel coordinates out of bounds");
        let index = (y * self.size + x) as usize;
        self.data[index] = pixel;
        self.dirty = true;
    }

    /// Fills the entire tile with a single color.
    pub fn fill(&mut self, color: Rgba8) {
        self.data.fill(color);
        self.dirty = true;
    }

    /// Clears the tile to transparent.
    pub fn clear(&mut self) {
        self.fill(Rgba8::TRANSPARENT);
    }

    /// Creates a copy of this tile.
    pub fn clone(&self) -> Tile {
        Tile {
            coords: self.coords,
            size: self.size,
            data: self.data.clone(),
            dirty: self.dirty,
        }
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tile")
            .field("coords", &self.coords)
            .field("size", &self.size)
            .field("dirty", &self.dirty)
            .field("pixel_count", &self.data.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_coords() {
        let coords = TileCoords::new(1, 2);
        assert_eq!(coords.x, 1);
        assert_eq!(coords.y, 2);

        let from_pixels = TileCoords::from_pixel_coords(300, 500, 256);
        assert_eq!(from_pixels.x, 1);
        assert_eq!(from_pixels.y, 1);
    }

    #[test]
    fn test_tile_creation() {
        let coords = TileCoords::new(0, 0);
        let tile = Tile::new(coords, 256);
        assert_eq!(tile.size(), 256);
        assert_eq!(tile.data().len(), 256 * 256);
        assert!(tile.is_dirty());
    }

    #[test]
    fn test_tile_with_data() {
        let coords = TileCoords::new(0, 0);
        let data = vec![Rgba8::WHITE; 64 * 64];
        let tile = Tile::with_data(coords, 64, data);
        assert_eq!(tile.size(), 64);
        assert_eq!(tile.data().len(), 64 * 64);
    }

    #[test]
    #[should_panic(expected = "Data length")]
    fn test_tile_with_data_wrong_length() {
        let coords = TileCoords::new(0, 0);
        let data = vec![Rgba8::WHITE; 100];
        Tile::with_data(coords, 64, data);
    }

    #[test]
    fn test_pixel_access() {
        let mut tile = Tile::new(TileCoords::new(0, 0), 64);
        let color = Rgba8::new(255, 128, 64, 255);
        tile.set_pixel(10, 20, color);
        assert_eq!(tile.get_pixel(10, 20), color);
    }

    #[test]
    fn test_fill() {
        let mut tile = Tile::new(TileCoords::new(0, 0), 64);
        tile.fill(Rgba8::RED);
        assert_eq!(tile.get_pixel(0, 0), Rgba8::RED);
        assert_eq!(tile.get_pixel(63, 63), Rgba8::RED);
    }

    #[test]
    fn test_clear() {
        let mut tile = Tile::new(TileCoords::new(0, 0), 64);
        tile.fill(Rgba8::RED);
        tile.clear();
        assert_eq!(tile.get_pixel(0, 0), Rgba8::TRANSPARENT);
    }

    #[test]
    fn test_dirty_flags() {
        let mut tile = Tile::new(TileCoords::new(0, 0), 64);
        assert!(tile.is_dirty());

        tile.mark_clean();
        assert!(!tile.is_dirty());

        tile.set_pixel(0, 0, Rgba8::WHITE);
        assert!(tile.is_dirty());
    }

    #[test]
    fn test_tile_index() {
        let index = TileIndex::new(42);
        assert_eq!(index.value(), 42);
    }
}
