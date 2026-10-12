//! CPU rendering pipeline
//!
//! This module provides the reference CPU rendering implementation for VEGA.
//! It handles compositing layers with blend modes, adjustments, and transforms.

pub mod cache;
pub mod compositing;
pub mod viewport;

pub use cache::{CacheEntry, RenderCache};
pub use compositing::{blend_pixels, BlendOp, Compositor};
pub use viewport::ViewportRenderer;
