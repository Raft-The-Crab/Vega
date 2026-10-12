//! Tile memory management
//!
//! Provides memory tracking and eviction policies for tile storage.

use crate::color::types::Rgba8;
use crate::limits;
use std::collections::HashMap;

/// Memory usage statistics for tile storage.
#[derive(Debug, Clone, Copy)]
pub struct MemoryStats {
    /// Total number of tiles allocated.
    pub tile_count: usize,
    /// Total memory usage in bytes.
    pub total_bytes: usize,
    /// Memory usage percentage (of max limit).
    pub usage_percent: f32,
}

/// Memory policy for tile eviction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvictionPolicy {
    /// Least Recently Used (LRU) eviction.
    LRU,
    /// Random eviction.
    Random,
    /// No eviction (fail when full).
    None,
}

/// Memory manager for tile storage.
pub struct TileMemoryManager {
    /// Maximum memory limit in bytes.
    max_memory: usize,
    /// Current memory usage in bytes.
    current_memory: usize,
    /// Tile access tracking for LRU.
    access_count: HashMap<u64, u64>,
    /// Next access counter value.
    next_access: u64,
    /// Eviction policy.
    policy: EvictionPolicy,
}

impl TileMemoryManager {
    /// Creates a new memory manager with the given limit.
    ///
    /// # Arguments
    /// * `max_memory` - Maximum memory in bytes (use 0 for unlimited)
    /// * `policy` - Eviction policy to use
    pub fn new(max_memory: usize, policy: EvictionPolicy) -> Self {
        Self {
            max_memory,
            current_memory: 0,
            access_count: HashMap::new(),
            next_access: 0,
            policy,
        }
    }

    /// Creates a memory manager with a default limit (512MB).
    pub fn with_default_limit() -> Self {
        Self::new(512 * 1024 * 1024, EvictionPolicy::LRU)
    }

    /// Calculates memory usage for a tile of given size.
    pub fn tile_memory_size(tile_size: u32) -> usize {
        let pixel_count = (tile_size * tile_size) as usize;
        pixel_count * std::mem::size_of::<RGBA8>()
    }

    /// Checks if a tile can be allocated.
    pub fn can_allocate(&self, tile_size: u32) -> bool {
        if self.max_memory == 0 {
            return true;
        }

        let required = Self::tile_memory_size(tile_size);
        self.current_memory + required <= self.max_memory
    }

    /// Records tile allocation.
    pub fn allocate(&mut self, tile_index: u64, tile_size: u32) -> Result<(), &'static str> {
        let required = Self::tile_memory_size(tile_size);

        if self.max_memory > 0 && self.current_memory + required > self.max_memory {
            return Err("Memory limit exceeded");
        }

        self.current_memory += required;
        self.access_count.insert(tile_index, self.next_access);
        self.next_access += 1;

        Ok(())
    }

    /// Records tile deallocation.
    pub fn deallocate(&mut self, tile_index: u64, tile_size: u32) {
        let freed = Self::tile_memory_size(tile_size);
        self.current_memory = self.current_memory.saturating_sub(freed);
        self.access_count.remove(&tile_index);
    }

    /// Selects a tile for eviction based on the policy.
    ///
    /// Returns None if no eviction should occur or no tiles are available.
    pub fn select_for_eviction(&self, tile_indices: &[u64]) -> Option<u64> {
        if tile_indices.is_empty() {
            return None;
        }

        match self.policy {
            EvictionPolicy::LRU => {
                // Find tile with lowest access count
                tile_indices
                    .iter()
                    .min_by_key(|&id| self.access_count.get(&id).unwrap_or(&0))
                    .copied()
            }
            EvictionPolicy::Random => {
                // Select random tile
                let index = fastrand::usize(..tile_indices.len());
                Some(tile_indices[index])
            }
            EvictionPolicy::None => None,
        }
    }

    /// Updates access tracking for a tile.
    pub fn record_access(&mut self, tile_index: u64) {
        self.access_count.insert(tile_index, self.next_access);
        self.next_access += 1;
    }

    /// Returns current memory statistics.
    pub fn stats(&self) -> MemoryStats {
        let tile_count = self.access_count.len();
        let usage_percent = if self.max_memory > 0 {
            (self.current_memory as f32 / self.max_memory as f32) * 100.0
        } else {
            0.0
        };

        MemoryStats {
            tile_count,
            total_bytes: self.current_memory,
            usage_percent,
        }
    }

    /// Returns the current memory usage in bytes.
    pub fn current_memory(&self) -> usize {
        self.current_memory
    }

    /// Returns the maximum memory limit in bytes.
    pub fn max_memory(&self) -> usize {
        self.max_memory
    }

    /// Sets a new memory limit.
    pub fn set_max_memory(&mut self, max_memory: usize) {
        self.max_memory = max_memory;
    }

    /// Clears all tracking data.
    pub fn clear(&mut self) {
        self.current_memory = 0;
        self.access_count.clear();
        self.next_access = 0;
    }
}

impl Default for TileMemoryManager {
    fn default() -> Self {
        Self::with_default_limit()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_memory_size() {
        let size = TileMemoryManager::tile_memory_size(256);
        assert_eq!(size, 256 * 256 * 4); // RGBA8 = 4 bytes per pixel
    }

    #[test]
    fn test_memory_allocation() {
        let mut manager = TileMemoryManager::new(1024 * 1024, EvictionPolicy::LRU);
        assert!(manager.can_allocate(256));

        manager.allocate(1, 256).unwrap();
        assert_eq!(manager.current_memory(), TileMemoryManager::tile_memory_size(256));
    }

    #[test]
    fn test_memory_limit() {
        let mut manager = TileMemoryManager::new(1000, EvictionPolicy::LRU);
        let tile_size = 256;
        let tile_memory = TileMemoryManager::tile_memory_size(tile_size);

        assert!(tile_memory > 1000);
        assert!(!manager.can_allocate(tile_size));
    }

    #[test]
    fn test_deallocation() {
        let mut manager = TileMemoryManager::new(1024 * 1024, EvictionPolicy::LRU);
        manager.allocate(1, 256).unwrap();

        manager.deallocate(1, 256);
        assert_eq!(manager.current_memory(), 0);
    }

    #[test]
    fn test_lru_eviction() {
        let mut manager = TileMemoryManager::new(1024 * 1024, EvictionPolicy::LRU);
        manager.allocate(1, 64).unwrap();
        manager.allocate(2, 64).unwrap();
        manager.allocate(3, 64).unwrap();

        // Access tile 1 most recently
        manager.record_access(1);

        // LRU should select tile 2 or 3 (least recently accessed)
        let indices = vec![1, 2, 3];
        let evicted = manager.select_for_eviction(&indices);
        assert!(evicted.is_some());
        assert_ne!(evicted.unwrap(), 1);
    }

    #[test]
    fn test_memory_stats() {
        let mut manager = TileMemoryManager::new(10 * 1024 * 1024, EvictionPolicy::LRU);
        manager.allocate(1, 256).unwrap();

        let stats = manager.stats();
        assert_eq!(stats.tile_count, 1);
        assert!(stats.total_bytes > 0);
        assert!(stats.usage_percent < 100.0);
    }
}
