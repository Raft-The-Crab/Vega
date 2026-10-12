//! Thumbnail generation
//!
//! Provides thumbnail generation for .vega files.

use std::io::Cursor;
use image::{ImageBuffer, Rgba, RgbaImage};
use vega_core::{ColorSpace, Size, Document};
use crate::Result;

/// Thumbnail configuration.
#[derive(Debug, Clone)]
pub struct ThumbnailConfig {
    /// Maximum thumbnail width.
    pub max_width: u32,
    /// Maximum thumbnail height.
    pub max_height: u32,
    /// Thumbnail quality (for JPEG, if used).
    pub quality: u8,
}

impl Default for ThumbnailConfig {
    fn default() -> Self {
        Self {
            max_width: 256,
            max_height: 256,
            quality: 85,
        }
    }
}

/// Thumbnail generator.
pub struct ThumbnailGenerator {
    config: ThumbnailConfig,
}

impl ThumbnailGenerator {
    /// Creates a new thumbnail generator.
    pub fn new(config: ThumbnailConfig) -> Self {
        Self { config }
    }

    /// Creates a thumbnail generator with default config.
    pub fn with_defaults() -> Self {
        Self::new(ThumbnailConfig::default())
    }

    /// Generates a thumbnail from a document.
    ///
    /// For now, this generates a simple colored placeholder since we don't have
    /// full rendering implemented yet. In P2, this will use the rendering pipeline.
    pub fn generate_thumbnail(&self, document: &Document) -> Result<RgbaImage> {
        let (width, height) = self.calculate_thumbnail_size(document.size());

        // Generate a simple thumbnail based on document properties
        let mut image = ImageBuffer::new(width, height);

        // Fill with a gradient based on document name hash
        let hash = self.hash_string(document.name());
        let color = Rgba([
            (hash % 256) as u8,
            ((hash >> 8) % 256) as u8,
            ((hash >> 16) % 256) as u8,
            255,
        ]);

        for pixel in image.pixels_mut() {
            *pixel = color;
        }

        Ok(image)
    }

    /// Calculates the thumbnail size maintaining aspect ratio.
    fn calculate_thumbnail_size(&self, doc_size: Size) -> (u32, u32) {
        let (max_w, max_h) = (self.config.max_width, self.config.max_height);
        let (doc_w, doc_h) = (doc_size.width, doc_size.height);

        if doc_w <= max_w && doc_h <= max_h {
            return (doc_w, doc_h);
        }

        let ratio = (max_w as f32 / doc_w as f32).min(max_h as f32 / doc_h as f32);
        let new_w = (doc_w as f32 * ratio).round() as u32;
        let new_h = (doc_h as f32 * ratio).round() as u32;

        (new_w.max(1), new_h.max(1))
    }

    /// Simple hash function for generating consistent colors.
    fn hash_string(&self, s: &str) -> u32 {
        s.bytes().fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32))
    }

    /// Encodes a thumbnail to PNG bytes.
    pub fn encode_png(&self, image: &RgbaImage) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        image.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png)?;
        Ok(buffer)
    }

    /// Returns the configuration.
    pub fn config(&self) -> &ThumbnailConfig {
        &self.config
    }
}

impl Default for ThumbnailGenerator {
    fn default() -> Self {
        Self::with_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vega_core::Size;

    #[test]
    fn test_thumbnail_config_default() {
        let config = ThumbnailConfig::default();
        assert_eq!(config.max_width, 256);
        assert_eq!(config.max_height, 256);
    }

    #[test]
    fn test_calculate_thumbnail_size() {
        let generator = ThumbnailGenerator::with_defaults();
        let doc_size = Size::new(1920, 1080);
        let (w, h) = generator.calculate_thumbnail_size(doc_size);

        assert!(w <= 256);
        assert!(h <= 256);
    }

    #[test]
    fn test_generate_thumbnail() {
        let generator = ThumbnailGenerator::with_defaults();
        let document = Document::new(
            "Test".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        let thumbnail = generator.generate_thumbnail(&document).unwrap();
        assert_eq!(thumbnail.width(), 100);
        assert_eq!(thumbnail.height(), 100);
    }

    #[test]
    fn test_encode_png() {
        let generator = ThumbnailGenerator::with_defaults();
        let image = RgbaImage::new(100, 100);
        let png_bytes = generator.encode_png(&image).unwrap();

        assert!(!png_bytes.is_empty());
        // PNG magic bytes
        assert_eq!(&png_bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }
}
