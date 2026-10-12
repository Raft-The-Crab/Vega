//! Configuration management for VEGA Core
//!
//! Provides runtime configuration with validation and sensible defaults.

use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Runtime configuration for VEGA Core.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VegaConfig {
    /// Rendering configuration.
    pub render: RenderConfig,
    /// Memory configuration.
    pub memory: MemoryConfig,
    /// Tile configuration.
    pub tiles: TileConfig,
    /// Cache configuration.
    pub cache: CacheConfig,
}

impl Default for VegaConfig {
    fn default() -> Self {
        Self {
            render: RenderConfig::default(),
            memory: MemoryConfig::default(),
            tiles: TileConfig::default(),
            cache: CacheConfig::default(),
        }
    }
}

/// Rendering configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderConfig {
    /// Enable parallel rendering when available.
    pub enable_parallel: bool,
    /// Maximum number of parallel rendering threads (0 = auto).
    pub max_parallel_threads: usize,
    /// Enable render caching.
    pub enable_caching: bool,
    /// Enable render metrics collection.
    pub enable_metrics: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            enable_parallel: true,
            max_parallel_threads: 0, // Auto-detect
            enable_caching: true,
            enable_metrics: true,
        }
    }
}

/// Memory configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Maximum memory for tile storage in bytes (0 = unlimited).
    pub max_tile_memory: usize,
    /// Maximum memory for render cache in bytes (0 = unlimited).
    pub max_cache_memory: usize,
    /// Memory pressure threshold percentage (0.0-1.0).
    pub memory_pressure_threshold: f32,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            max_tile_memory: 512 * 1024 * 1024, // 512MB
            max_cache_memory: 256 * 1024 * 1024, // 256MB
            memory_pressure_threshold: 0.8,
        }
    }
}

/// Tile configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileConfig {
    /// Default tile size in pixels.
    pub default_tile_size: u32,
    /// Minimum tile size in pixels.
    pub min_tile_size: u32,
    /// Maximum tile size in pixels.
    pub max_tile_size: u32,
    /// Maximum number of tiles per document.
    pub max_tiles_per_document: usize,
}

impl Default for TileConfig {
    fn default() -> Self {
        Self {
            default_tile_size: 256,
            min_tile_size: 64,
            max_tile_size: 4096,
            max_tiles_per_document: 10_000,
        }
    }
}

/// Cache configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Maximum number of cached tiles.
    pub max_cached_tiles: usize,
    /// Cache eviction policy ("lru" or "random").
    pub eviction_policy: String,
    /// Enable cache compression (future feature).
    pub enable_compression: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_cached_tiles: 1000,
            eviction_policy: "lru".to_string(),
            enable_compression: false,
        }
    }
}

impl VegaConfig {
    /// Creates a new configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads configuration from a JSON string.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serializes configuration to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Validates the configuration.
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.tiles.default_tile_size < self.tiles.min_tile_size
            || self.tiles.default_tile_size > self.tiles.max_tile_size
        {
            return Err(ConfigError::InvalidTileSize(
                self.tiles.default_tile_size,
            ));
        }

        if self.tiles.min_tile_size == 0 || self.tiles.max_tile_size == 0 {
            return Err(ConfigError::InvalidTileSize(0));
        }

        if self.memory.memory_pressure_threshold < 0.0
            || self.memory.memory_pressure_threshold > 1.0
        {
            return Err(ConfigError::InvalidMemoryPressureThreshold(
                self.memory.memory_pressure_threshold,
            ));
        }

        if self.cache.eviction_policy != "lru" && self.cache.eviction_policy != "random" {
            return Err(ConfigError::InvalidEvictionPolicy(
                self.cache.eviction_policy.clone(),
            ));
        }

        Ok(())
    }

    /// Creates an Arc-wrapped configuration for thread-safe sharing.
    pub fn into_arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}

/// Configuration errors.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Invalid tile size: {0}")]
    InvalidTileSize(u32),

    #[error("Invalid memory pressure threshold: {0}")]
    InvalidMemoryPressureThreshold(f32),

    #[error("Invalid eviction policy: {0}")]
    InvalidEvictionPolicy(String),

    #[error("JSON parse error: {0}")]
    JsonError(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = VegaConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_invalid_tile_size() {
        let mut config = VegaConfig::default();
        config.tiles.default_tile_size = 10;
        config.tiles.min_tile_size = 64;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_invalid_memory_threshold() {
        let mut config = VegaConfig::default();
        config.memory.memory_pressure_threshold = 1.5;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_json() {
        let config = VegaConfig::default();
        let json = config.to_json().unwrap();
        let restored = VegaConfig::from_json(&json).unwrap();
        assert_eq!(config.tiles.default_tile_size, restored.tiles.default_tile_size);
    }
}
