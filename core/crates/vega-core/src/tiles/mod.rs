//! Tile-based pixel storage system
//!
//! This module provides efficient memory management for image data using
//! a tile-based approach. Images are divided into fixed-size tiles to enable:
//! - Efficient caching (only load needed tiles)
//! - Partial updates (modify only changed tiles)
//! - Memory efficiency (sparse representation)
//! - Parallel processing (work on tiles independently)

pub mod memory;
pub mod storage;
pub mod tile;

pub use memory::{EvictionPolicy, MemoryStats, TileMemoryManager};
pub use storage::{TileStorage, TileHandle};
pub use tile::{Tile, TileCoords, TileIndex};
