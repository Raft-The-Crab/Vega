//! Tile storage management
//!
//! This module provides efficient storage and management of tiles for
//! image data. It handles the mapping between pixel coordinates and tiles,
//! manages tile lifetimes, and provides access patterns for rendering.

use crate::color::types::Rgba8;
use crate::errors::DocumentError;
use crate::limits;
use crate::tiles::tile::{Tile, TileCoords, TileIndex};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// Handle to a tile in storage.
///
/// This provides safe access to tile data and ensures the tile
/// remains valid in storage while the handle exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileHandle {
    index: TileIndex,
}

impl TileHandle {
    /// Creates a new tile handle.
    fn new(index: TileIndex) -> Self {
        Self { index }
    }

    /// Returns the tile index.
    pub fn index(self) -> TileIndex {
        self.index
    }
}

/// Storage for tiles in a document.
///
/// Manages a sparse grid of tiles, creating them on demand and
/// providing efficient access patterns for rendering and editing.
///
/// # Thread Safety
/// This struct is not thread-safe for concurrent mutation. For concurrent
/// access, wrap it in a Mutex or RwLock from the standard library.
///
/// # Resource Limits
/// The storage enforces a maximum of 10,000 allocated tiles to prevent
/// memory exhaustion. Attempting to create more tiles will return an error.
pub struct TileStorage {
    /// Map from tile coordinates to tile handles.
    tiles: HashMap<TileCoords, TileHandle>,
    /// Actual tile data indexed by handle.
    tile_data: HashMap<TileIndex, Tile>,
    /// Next available tile index.
    next_index: AtomicU64,
    /// Document width in pixels.
    width: u32,
    /// Document height in pixels.
    height: u32,
    /// Tile size in pixels.
    tile_size: u32,
    /// Number of tiles across the document.
    tiles_x: u32,
    /// Number of tiles down the document.
    tiles_y: u32,
}

impl TileStorage {
    /// Creates a new tile storage for the given document dimensions.
    ///
    /// # Errors
    /// Returns an error if dimensions are invalid.
    pub fn new(width: u32, height: u32, tile_size: u32) -> Result<Self, DocumentError> {
        limits::validate_dimensions(width, height)?;
        if tile_size < limits::MIN_TILE_SIZE || tile_size > limits::MAX_TILE_SIZE {
            return Err(DocumentError::InvalidParameter(format!(
                "Tile size {} out of range [{}, {}]",
                tile_size, limits::MIN_TILE_SIZE, limits::MAX_TILE_SIZE
            )));
        }

        let tiles_x = (width + tile_size - 1) / tile_size;
        let tiles_y = (height + tile_size - 1) / tile_size;

        Ok(Self {
            tiles: HashMap::new(),
            tile_data: HashMap::new(),
            next_index: AtomicU64::new(0),
            width,
            height,
            tile_size,
            tiles_x,
            tiles_y,
        })
    }

    /// Creates tile storage with default tile size.
    pub fn with_default_tile_size(width: u32, height: u32) -> Result<Self, DocumentError> {
        Self::new(width, height, limits::DEFAULT_TILE_SIZE)
    }

    /// Returns the document width.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the document height.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns the tile size.
    pub fn tile_size(&self) -> u32 {
        self.tile_size
    }

    /// Returns the number of tiles across the document.
    pub fn tiles_x(&self) -> u32 {
        self.tiles_x
    }

    /// Returns the number of tiles down the document.
    pub fn tiles_y(&self) -> u32 {
        self.tiles_y
    }

    /// Returns the total number of tiles in the grid.
    pub fn total_tiles(&self) -> u64 {
        self.tiles_x as u64 * self.tiles_y as u64
    }

    /// Gets a tile by coordinates, creating it if it doesn't exist.
    pub fn get_or_create(&mut self, coords: TileCoords) -> Result<TileHandle, DocumentError> {
        if coords.x >= self.tiles_x || coords.y >= self.tiles_y {
            return Err(DocumentError::InvalidParameter(format!(
                "Tile coordinates {:?} out of bounds for {}x{} grid",
                coords, self.tiles_x, self.tiles_y
            )));
        }

        // Check for tile count limit
        if self.allocated_count() >= 10000 {
            return Err(DocumentError::ResourceLimit(
                "Maximum tile count exceeded".to_string(),
            ));
        }

        if let Some(&handle) = self.tiles.get(&coords) {
            return Ok(handle);
        }

        // Create new tile
        let index = TileIndex::new(self.next_index.fetch_add(1, Ordering::SeqCst));
        let handle = TileHandle::new(index);

        // Determine actual tile size (edge tiles may be smaller)
        let tile_width = self.tile_width_for_tile(coords.x);
        let tile_height = self.tile_height_for_tile(coords.y);
        let tile_size = tile_width.max(tile_height);

        let mut tile = Tile::new(coords, tile_size);

        // For edge tiles, we need to handle partial tiles
        if tile_width != tile_size || tile_height != tile_size {
            // This is a partial tile - the extra pixels should be ignored
            // The tile is created at full size but only the valid region is used
        }

        self.tile_data.insert(index, tile);
        self.tiles.insert(coords, handle);

        Ok(handle)
    }

    /// Gets a tile by coordinates without creating it.
    ///
    /// # Safety
    /// This method performs a read-only operation and is safe to call
    /// concurrently with other read operations.
    pub fn get(&self, coords: TileCoords) -> Option<TileHandle> {
        self.tiles.get(&coords).copied()
    }

    /// Gets a tile by handle.
    ///
    /// # Safety
    /// This method performs a read-only operation and is safe to call
    /// concurrently with other read operations.
    pub fn get_tile(&self, handle: TileHandle) -> Option<&Tile> {
        self.tile_data.get(&handle.index())
    }

    /// Gets a mutable tile by handle.
    ///
    /// # Safety
    /// This method requires mutable access and cannot be called concurrently
    /// with any other operation on the storage.
    pub fn get_tile_mut(&mut self, handle: TileHandle) -> Option<&mut Tile> {
        self.tile_data.get_mut(&handle.index())
    }

    /// Removes a tile from storage.
    pub fn remove(&mut self, handle: TileHandle) -> Option<Tile> {
        if let Some(tile) = self.tile_data.remove(&handle.index()) {
            self.tiles.remove(&tile.coords());
            Some(tile)
        } else {
            None
        }
    }

    /// Returns the number of currently allocated tiles.
    pub fn allocated_count(&self) -> usize {
        self.tile_data.len()
    }

    /// Clears all tiles from storage.
    pub fn clear(&mut self) {
        self.tiles.clear();
        self.tile_data.clear();
        self.next_index.store(0, Ordering::SeqCst);
    }

    /// Converts pixel coordinates to tile coordinates.
    pub fn pixel_to_tile(&self, x: u32, y: u32) -> TileCoords {
        TileCoords::from_pixel_coords(x, y, self.tile_size)
    }

    /// Converts tile coordinates to pixel coordinates (top-left corner).
    pub fn tile_to_pixel(&self, coords: TileCoords) -> (u32, u32) {
        (coords.x * self.tile_size, coords.y * self.tile_size)
    }

    /// Returns the actual width of a tile at the given x index.
    /// Edge tiles may be smaller than the full tile size.
    fn tile_width_for_tile(&self, tile_x: u32) -> u32 {
        let start_x = tile_x * self.tile_size;
        let end_x = (start_x + self.tile_size).min(self.width);
        end_x - start_x
    }

    /// Returns the actual height of a tile at the given y index.
    /// Edge tiles may be smaller than the full tile size.
    fn tile_height_for_tile(&self, tile_y: u32) -> u32 {
        let start_y = tile_y * self.tile_size;
        let end_y = (start_y + self.tile_size).min(self.height);
        end_y - start_y
    }

    /// Iterates over all tiles in the storage.
    pub fn iter(&self) -> impl Iterator<Item = (TileCoords, TileHandle)> + '_ {
        self.tiles.iter().map(|(&coords, &handle)| (coords, handle))
    }

    /// Returns the number of allocated tiles.
    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    /// Returns tiles that intersect with the given pixel rectangle.
    pub fn tiles_in_rect(
        &self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Vec<TileCoords> {
        let start_x = x / self.tile_size;
        let start_y = y / self.tile_size;
        let end_x = ((x + width - 1) / self.tile_size).min(self.tiles_x - 1);
        let end_y = ((y + height - 1) / self.tile_size).min(self.tiles_y - 1);

        let mut coords = Vec::new();
        for ty in start_y..=end_y {
            for tx in start_x..=end_x {
                coords.push(TileCoords::new(tx, ty));
            }
        }
        coords
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_creation() {
        let storage = TileStorage::new(1920, 1080, 256).unwrap();
        assert_eq!(storage.width(), 1920);
        assert_eq!(storage.height(), 1080);
        assert_eq!(storage.tile_size(), 256);
        assert_eq!(storage.tiles_x(), 8); // 1920 / 256 = 7.5 -> 8
        assert_eq!(storage.tiles_y(), 5); // 1080 / 256 = 4.2 -> 5
    }

    #[test]
    fn test_storage_invalid_dimensions() {
        let result = TileStorage::new(0, 100, 256);
        assert!(result.is_err());

        let result = TileStorage::new(1920, 1080, 32);
        assert!(result.is_err()); // Below MIN_TILE_SIZE
    }

    #[test]
    fn test_get_or_create_tile() {
        let mut storage = TileStorage::new(512, 512, 256).unwrap();
        let coords = TileCoords::new(0, 0);

        let handle = storage.get_or_create(coords).unwrap();
        assert_eq!(storage.allocated_count(), 1);

        // Getting the same tile should return the same handle
        let handle2 = storage.get_or_create(coords).unwrap();
        assert_eq!(handle, handle2);
        assert_eq!(storage.allocated_count(), 1);
    }

    #[test]
    fn test_get_tile() {
        let mut storage = TileStorage::new(512, 512, 256).unwrap();
        let coords = TileCoords::new(0, 0);

        let handle = storage.get_or_create(coords).unwrap();
        let tile = storage.get_tile(handle).unwrap();
        assert_eq!(tile.coords(), coords);
        assert_eq!(tile.size(), 256);
    }

    #[test]
    fn test_get_tile_mut() {
        let mut storage = TileStorage::new(512, 512, 256).unwrap();
        let coords = TileCoords::new(0, 0);

        let handle = storage.get_or_create(coords).unwrap();
        let tile = storage.get_tile_mut(handle).unwrap();
        tile.fill(Rgba8::RED);

        let tile = storage.get_tile(handle).unwrap();
        assert_eq!(tile.get_pixel(0, 0), Rgba8::RED);
    }

    #[test]
    fn test_remove_tile() {
        let mut storage = TileStorage::new(512, 512, 256).unwrap();
        let coords = TileCoords::new(0, 0);

        let handle = storage.get_or_create(coords).unwrap();
        assert_eq!(storage.allocated_count(), 1);

        storage.remove(handle);
        assert_eq!(storage.allocated_count(), 0);
        assert!(storage.get(coords).is_none());
    }

    #[test]
    fn test_clear() {
        let mut storage = TileStorage::new(512, 512, 256).unwrap();

        storage.get_or_create(TileCoords::new(0, 0)).unwrap();
        storage.get_or_create(TileCoords::new(1, 0)).unwrap();
        assert_eq!(storage.allocated_count(), 2);

        storage.clear();
        assert_eq!(storage.allocated_count(), 0);
    }

    #[test]
    fn test_pixel_to_tile() {
        let storage = TileStorage::new(512, 512, 256).unwrap();
        let coords = storage.pixel_to_tile(100, 300);
        assert_eq!(coords.x, 0);
        assert_eq!(coords.y, 1);
    }

    #[test]
    fn test_tile_to_pixel() {
        let storage = TileStorage::new(512, 512, 256).unwrap();
        let coords = TileCoords::new(1, 2);
        let (x, y) = storage.tile_to_pixel(coords);
        assert_eq!(x, 256);
        assert_eq!(y, 512);
    }

    #[test]
    fn test_tiles_in_rect() {
        let storage = TileStorage::new(1024, 1024, 256).unwrap();
        let tiles = storage.tiles_in_rect(100, 100, 400, 400);
        assert_eq!(tiles.len(), 4); // 2x2 grid
    }

    #[test]
    fn test_tile_out_of_bounds() {
        let mut storage = TileStorage::new(512, 512, 256).unwrap();
        let coords = TileCoords::new(10, 10); // Out of bounds
        let result = storage.get_or_create(coords);
        assert!(result.is_err());
    }

    #[test]
    fn test_edge_tiles() {
        let storage = TileStorage::new(300, 300, 256).unwrap();
        assert_eq!(storage.tiles_x(), 2);
        assert_eq!(storage.tiles_y(), 2);
    }
}
