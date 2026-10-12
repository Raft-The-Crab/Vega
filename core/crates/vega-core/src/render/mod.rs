//! CPU rendering pipeline
//!
//! This module provides the reference CPU rendering implementation for VEGA.
//! It handles compositing layers with blend modes, adjustments, and transforms.

pub mod adjustments;
pub mod cache;
pub mod compositing;
pub mod hsl;
pub mod parallel;
pub mod transform;
pub mod viewport;

pub use adjustments::apply_adjustment;
pub use cache::{CacheEntry, RenderCache};
pub use compositing::{blend_pixels, BlendOp, Compositor};
pub use hsl::{Hsl, blend_color, blend_hue, blend_luminosity, blend_saturation, luminance};
pub use parallel::{render_tiles_parallel, render_tiles_sequential};
pub use transform::{inverse_transform_point, is_identity_transform, transform_point, transform_rect, transformed_bounds};
pub use viewport::ViewportRenderer;
