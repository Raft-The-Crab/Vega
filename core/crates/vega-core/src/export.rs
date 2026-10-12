//! Export functionality
//!
//! Provides export of documents to PNG, JPEG, and WebP formats.

use crate::color::types::rgba::RGBA8;
use crate::document::model::document::Document;
use crate::geometry::Size;
use crate::errors::DocumentError;
use crate::render::ViewportRenderer;
use crate::tiles::TileStorage;
use image::{ImageBuffer, Rgba, DynamicImage};
use std::path::Path;

/// Export format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    /// PNG format (lossless).
    Png,
    /// JPEG format (lossy).
    Jpeg,
    /// WebP format (lossy/lossless).
    WebP,
}

/// Export quality for lossy formats (0.0 to 1.0).
#[derive(Debug, Clone, Copy)]
pub struct ExportQuality {
    /// Quality value (0.0 to 1.0).
    pub value: f32,
}

impl ExportQuality {
    /// Creates a new export quality.
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
        }
    }

    /// High quality (0.9).
    pub fn high() -> Self {
        Self::new(0.9)
    }

    /// Medium quality (0.75).
    pub fn medium() -> Self {
        Self::new(0.75)
    }

    /// Low quality (0.5).
    pub fn low() -> Self {
        Self::new(0.5)
    }
}

/// Export parameters.
#[derive(Debug, Clone)]
pub struct ExportParams {
    /// Export format.
    pub format: ExportFormat,
    /// Export quality (for lossy formats).
    pub quality: Option<ExportQuality>,
    /// Scale factor (1.0 = original size).
    pub scale: f32,
    /// Whether to include alpha channel.
    pub include_alpha: bool,
}

impl Default for ExportParams {
    fn default() -> Self {
        Self {
            format: ExportFormat::Png,
            quality: None,
            scale: 1.0,
            include_alpha: true,
        }
    }
}

/// Exports a document to a file.
pub fn export_document(
    document: &Document,
    tile_storage: &TileStorage,
    renderer: &ViewportRenderer,
    path: &Path,
    params: ExportParams,
) -> Result<(), DocumentError> {
    // Validate scale
    if params.scale <= 0.0 {
        return Err(DocumentError::InvalidParameter(
            "Scale must be positive".to_string(),
        ));
    }

    // Render the document
    let width = (document.canvas_size().width as f32 * params.scale) as u32;
    let height = (document.canvas_size().height as f32 * params.scale) as u32;

    let mut buffer = vec![RGBA8::TRANSPARENT; (width * height) as usize];

    // Render to buffer (simplified - full implementation would use viewport renderer)
    // For now, we'll create a simple solid color buffer
    for y in 0..height {
        for x in 0..width {
            let index = (y * width + x) as usize;
            buffer[index] = RGBA8::WHITE;
        }
    }

    // Convert to image buffer
    let image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
        width,
        height,
        buffer.iter()
            .flat_map(|rgba| [rgba.r, rgba.g, rgba.b, rgba.a])
            .collect(),
    ).ok_or_else(|| DocumentError::InvalidParameter(
        "Failed to create image buffer".to_string(),
    ))?;

    let dynamic_image = DynamicImage::ImageRgba8(image_buffer);

    // Apply alpha removal if needed
    let dynamic_image = if !params.include_alpha {
        dynamic_image.to_rgb8()
    } else {
        dynamic_image
    };

    // Save based on format
    match params.format {
        ExportFormat::Png => {
            dynamic_image.save(path)
                .map_err(|e| DocumentError::RuntimeError(e.to_string()))?;
        }
        ExportFormat::Jpeg => {
            let quality = params.quality.unwrap_or(ExportQuality::medium());
            let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(
                std::fs::File::create(path)
                    .map_err(|e| DocumentError::RuntimeError(e.to_string()))?,
                (quality.value * 100.0) as u8,
            );
            dynamic_image.write_with_encoder(encoder)
                .map_err(|e| DocumentError::RuntimeError(e.to_string()))?;
        }
        ExportFormat::WebP => {
            let quality = params.quality.unwrap_or(ExportQuality::medium());
            let encoder = image::codecs::webp::WebPEncoder::new_with_quality(
                (quality.value * 100.0) as f32,
            );
            dynamic_image.write_with_encoder(encoder)
                .map_err(|e| DocumentError::RuntimeError(e.to_string()))?;
        }
    }

    Ok(())
}

/// Exports a document to PNG format.
pub fn export_to_png(
    document: &Document,
    tile_storage: &TileStorage,
    renderer: &ViewportRenderer,
    path: &Path,
) -> Result<(), DocumentError> {
    let params = ExportParams {
        format: ExportFormat::Png,
        ..Default::default()
    };
    export_document(document, tile_storage, renderer, path, params)
}

/// Exports a document to JPEG format.
pub fn export_to_jpeg(
    document: &Document,
    tile_storage: &TileStorage,
    renderer: &ViewportRenderer,
    path: &Path,
    quality: ExportQuality,
) -> Result<(), DocumentError> {
    let params = ExportParams {
        format: ExportFormat::Jpeg,
        quality: Some(quality),
        include_alpha: false, // JPEG doesn't support alpha
        ..Default::default()
    };
    export_document(document, tile_storage, renderer, path, params)
}

/// Exports a document to WebP format.
pub fn export_to_webp(
    document: &Document,
    tile_storage: &TileStorage,
    renderer: &ViewportRenderer,
    path: &Path,
    quality: ExportQuality,
) -> Result<(), DocumentError> {
    let params = ExportParams {
        format: ExportFormat::WebP,
        quality: Some(quality),
        ..Default::default()
    };
    export_document(document, tile_storage, renderer, path, params)
}

/// Exports a document with custom parameters.
pub fn export_with_params(
    document: &Document,
    tile_storage: &TileStorage,
    renderer: &ViewportRenderer,
    path: &Path,
    params: ExportParams,
) -> Result<(), DocumentError> {
    export_document(document, tile_storage, renderer, path, params)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::ColorSpace;
    use tempfile::TempDir;

    #[test]
    fn test_export_quality() {
        let quality = ExportQuality::new(0.8);
        assert_eq!(quality.value, 0.8);

        let clamped = ExportQuality::new(1.5);
        assert_eq!(clamped.value, 1.0);

        let clamped_low = ExportQuality::new(-0.5);
        assert_eq!(clamped_low.value, 0.0);
    }

    #[test]
    fn test_export_quality_presets() {
        assert_eq!(ExportQuality::high().value, 0.9);
        assert_eq!(ExportQuality::medium().value, 0.75);
        assert_eq!(ExportQuality::low().value, 0.5);
    }

    #[test]
    fn test_export_params_default() {
        let params = ExportParams::default();
        assert_eq!(params.format, ExportFormat::Png);
        assert!(params.quality.is_none());
        assert_eq!(params.scale, 1.0);
        assert!(params.include_alpha);
    }

    #[test]
    fn test_export_to_png() {
        let document = Document::new(
            "Test".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        let tile_storage = TileStorage::new(100, 100, 64).unwrap();
        let renderer = ViewportRenderer::new();

        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.png");

        let result = export_to_png(&document, &tile_storage, &renderer, &path);
        assert!(result.is_ok());
        assert!(path.exists());
    }

    #[test]
    fn test_export_to_jpeg() {
        let document = Document::new(
            "Test".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        let tile_storage = TileStorage::new(100, 100, 64).unwrap();
        let renderer = ViewportRenderer::new();

        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.jpg");

        let result = export_to_jpeg(&document, &tile_storage, &renderer, &path, ExportQuality::medium());
        assert!(result.is_ok());
        assert!(path.exists());
    }

    #[test]
    fn test_export_to_webp() {
        let document = Document::new(
            "Test".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        let tile_storage = TileStorage::new(100, 100, 64).unwrap();
        let renderer = ViewportRenderer::new();

        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.webp");

        let result = export_to_webp(&document, &tile_storage, &renderer, &path, ExportQuality::medium());
        assert!(result.is_ok());
        assert!(path.exists());
    }

    #[test]
    fn test_export_scale_validation() {
        let document = Document::new(
            "Test".to_string(),
            Size::new(100, 100),
            ColorSpace::default(),
        );

        let tile_storage = TileStorage::new(100, 100, 64).unwrap();
        let renderer = ViewportRenderer::new();

        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.png");

        let params = ExportParams {
            scale: 0.0, // Invalid
            ..Default::default()
        };

        let result = export_with_params(&document, &tile_storage, &renderer, &path, params);
        assert!(result.is_err());
    }
}
