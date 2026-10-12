//! Color types and pixel formats.
//!
//! This module provides color representations with explicit bit depth,
//! alpha conventions, and color space information.

pub mod spaces;
pub mod types;

pub use spaces::ColorSpace;
pub use types::{Rgba, Rgba16, Rgba32, Rgba8};
