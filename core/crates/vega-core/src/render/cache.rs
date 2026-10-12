//! Rendering cache for optimization
//!
//! Provides caching mechanisms to improve rendering performance.

use crate::color::types::rgba::RGBA8;
use crate::document::ids::structure::NodeId;
use crate::document::revision::Revision;
use std::collections::HashMap;

/// Cache entry for rendered tile data.
#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// The rendered pixel data.
    pub data: Vec<RGBA8>,
    /// The document revision when this cache entry was created.
    pub revision: Revision,
    /// Whether the entry is valid.
    pub valid: bool,
}

impl CacheEntry {
    /// Creates a new cache entry.
    pub fn new(data: Vec<RGBA8>, revision: Revision) -> Self {
        Self {
            data,
            revision,
            valid: true,
        }
    }

    /// Checks if the cache entry is still valid for the given revision.
    pub fn is_valid_for(&self, revision: Revision) -> bool {
        self.valid && self.revision == revision
    }

    /// Invalidates the cache entry.
    pub fn invalidate(&mut self) {
        self.valid = false;
    }
}

/// Simple LRU cache for rendered tiles.
pub struct RenderCache {
    /// Cache entries keyed by node ID and tile coordinates.
    entries: HashMap<(NodeId, u32, u32), CacheEntry>,
    /// Maximum number of entries in the cache.
    max_entries: usize,
    /// Access order for LRU eviction.
    access_order: Vec<(NodeId, u32, u32)>,
}

impl RenderCache {
    /// Creates a new render cache with the given capacity.
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_entries,
            access_order: Vec::with_capacity(max_entries),
        }
    }

    /// Gets a cached entry if it exists and is valid for the current revision.
    pub fn get(&mut self, node_id: NodeId, tile_x: u32, tile_y: u32, revision: Revision) -> Option<&CacheEntry> {
        let key = (node_id, tile_x, tile_y);
        if let Some(entry) = self.entries.get(&key) {
            if entry.is_valid_for(revision) {
                // Update access order
                if let Some(pos) = self.access_order.iter().position(|&k| k == key) {
                    self.access_order.remove(pos);
                }
                self.access_order.push(key);
                return Some(entry);
            }
        }
        None
    }

    /// Inserts a new entry into the cache.
    pub fn insert(&mut self, node_id: NodeId, tile_x: u32, tile_y: u32, data: Vec<RGBA8>, revision: Revision) {
        let key = (node_id, tile_x, tile_y);

        // Evict if at capacity
        if self.entries.len() >= self.max_entries {
            if let Some(old_key) = self.access_order.first() {
                self.entries.remove(old_key);
                self.access_order.remove(0);
            }
        }

        let entry = CacheEntry::new(data, revision);
        self.entries.insert(key, entry);
        self.access_order.push(key);
    }

    /// Invalidates all cache entries for a given node.
    pub fn invalidate_node(&mut self, node_id: NodeId) {
        let keys_to_remove: Vec<_> = self.entries
            .keys()
            .filter(|(id, _, _)| *id == node_id)
            .cloned()
            .collect();

        for key in keys_to_remove {
            self.entries.remove(&key);
            if let Some(pos) = self.access_order.iter().position(|&k| k == key) {
                self.access_order.remove(pos);
            }
        }
    }

    /// Invalidates all cache entries.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.access_order.clear();
    }

    /// Returns the number of entries in the cache.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::ids::structure::NodeId;

    #[test]
    fn test_cache_entry_validity() {
        let data = vec![RGBA8::WHITE; 100];
        let entry = CacheEntry::new(data.clone(), Revision::new(1));

        assert!(entry.is_valid_for(Revision::new(1)));
        assert!(!entry.is_valid_for(Revision::new(2)));

        entry.invalidate();
        assert!(!entry.is_valid_for(Revision::new(1)));
    }

    #[test]
    fn test_render_cache() {
        let mut cache = RenderCache::new(10);
        let node_id = NodeId::generate();
        let data = vec![RGBA8::WHITE; 100];
        let revision = Revision::new(1);

        // Insert entry
        cache.insert(node_id, 0, 0, data.clone(), revision);
        assert_eq!(cache.len(), 1);

        // Get entry
        let cached = cache.get(node_id, 0, 0, revision);
        assert!(cached.is_some());

        // Get with wrong revision
        let cached = cache.get(node_id, 0, 0, Revision::new(2));
        assert!(cached.is_none());

        // Invalidate node
        cache.invalidate_node(node_id);
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_render_cache_eviction() {
        let mut cache = RenderCache::new(2);
        let node_id = NodeId::generate();
        let data = vec![RGBA8::WHITE; 100];
        let revision = Revision::new(1);

        // Insert 3 entries (should evict one)
        cache.insert(node_id, 0, 0, data.clone(), revision);
        cache.insert(node_id, 0, 1, data.clone(), revision);
        cache.insert(node_id, 0, 2, data.clone(), revision);

        assert_eq!(cache.len(), 2);
    }
}
