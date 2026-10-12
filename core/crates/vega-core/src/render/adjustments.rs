//! Adjustment layer rendering
//!
//! Provides rendering for non-destructive adjustment layers.

use crate::color::types::rgba::RGBA8;
use crate::document::nodes::AdjustmentType;

/// Applies an adjustment to a pixel.
pub fn apply_adjustment(pixel: RGBA8, adjustment_type: AdjustmentType, value: f32) -> RGBA8 {
    match adjustment_type {
        AdjustmentType::BrightnessContrast => apply_brightness_contrast(pixel, value),
        AdjustmentType::HueSaturation => apply_hue_saturation(pixel, value),
        AdjustmentType::Levels => apply_levels(pixel, value),
        AdjustmentType::Curves => apply_curves(pixel, value),
        AdjustmentType::ColorBalance => apply_color_balance(pixel, value),
        AdjustmentType::Exposure => apply_exposure(pixel, value),
        AdjustmentType::Vibrance => apply_vibrance(pixel, value),
        AdjustmentType::Invert => apply_invert(pixel),
        AdjustmentType::Grayscale => apply_grayscale(pixel),
        AdjustmentType::Sepia => apply_sepia(pixel),
    }
}

fn apply_brightness_contrast(pixel: RGBA8, value: f32) -> RGBA8 {
    // Simplified: brightness adjustment
    let factor = 1.0 + value / 100.0;
    RGBA8::new(
        (pixel.r as f32 * factor).clamp(0.0, 255.0).round() as u8,
        (pixel.g as f32 * factor).clamp(0.0, 255.0).round() as u8,
        (pixel.b as f32 * factor).clamp(0.0, 255.0).round() as u8,
        pixel.a,
    )
}

fn apply_hue_saturation(pixel: RGBA8, value: f32) -> RGBA8 {
    // Simplified: saturation adjustment
    let factor = 1.0 + value / 100.0;
    let avg = (pixel.r as f32 + pixel.g as f32 + pixel.b as f32) / 3.0;
    RGBA8::new(
        (avg + (pixel.r as f32 - avg) * factor).clamp(0.0, 255.0).round() as u8,
        (avg + (pixel.g as f32 - avg) * factor).clamp(0.0, 255.0).round() as u8,
        (avg + (pixel.b as f32 - avg) * factor).clamp(0.0, 255.0).round() as u8,
        pixel.a,
    )
}

fn apply_levels(pixel: RGBA8, value: f32) -> RGBA8 {
    // Simplified: level adjustment (black point)
    let black_point = value;
    let factor = 255.0 / (255.0 - black_point);
    RGBA8::new(
        ((pixel.r as f32 - black_point) * factor).clamp(0.0, 255.0).round() as u8,
        ((pixel.g as f32 - black_point) * factor).clamp(0.0, 255.0).round() as u8,
        ((pixel.b as f32 - black_point) * factor).clamp(0.0, 255.0).round() as u8,
        pixel.a,
    )
}

fn apply_curves(pixel: RGBA8, value: f32) -> RGBA8 {
    // Simplified: gamma correction
    let gamma = if value > 0.0 { 2.2 - value / 100.0 } else { 2.2 };
    let inv_gamma = 1.0 / gamma;
    RGBA8::new(
        (255.0 * (pixel.r as f32 / 255.0).powf(inv_gamma)).round() as u8,
        (255.0 * (pixel.g as f32 / 255.0).powf(inv_gamma)).round() as u8,
        (255.0 * (pixel.b as f32 / 255.0).powf(inv_gamma)).round() as u8,
        pixel.a,
    )
}

fn apply_color_balance(pixel: RGBA8, value: f32) -> RGBA8 {
    // Simplified: red channel balance
    let offset = value;
    RGBA8::new(
        (pixel.r as f32 + offset).clamp(0.0, 255.0).round() as u8,
        pixel.g,
        pixel.b,
        pixel.a,
    )
}

fn apply_exposure(pixel: RGBA8, value: f32) -> RGBA8 {
    // Simplified: exposure adjustment
    let factor = 2.0_f32.powf(value / 100.0);
    RGBA8::new(
        (pixel.r as f32 * factor).clamp(0.0, 255.0).round() as u8,
        (pixel.g as f32 * factor).clamp(0.0, 255.0).round() as u8,
        (pixel.b as f32 * factor).clamp(0.0, 255.0).round() as u8,
        pixel.a,
    )
}

fn apply_vibrance(pixel: RGBA8, value: f32) -> RGBA8 {
    // Simplified: vibrance adjustment
    let avg = (pixel.r as f32 + pixel.g as f32 + pixel.b as f32) / 3.0;
    let max = pixel.r.max(pixel.g).max(pixel.b) as f32;
    let min = pixel.r.min(pixel.g).min(pixel.b) as f32;
    let saturation = (max - min) / (max + min).max(0.001);
    let factor = 1.0 + (value / 100.0) * (1.0 - saturation);
    RGBA8::new(
        (avg + (pixel.r as f32 - avg) * factor).clamp(0.0, 255.0).round() as u8,
        (avg + (pixel.g as f32 - avg) * factor).clamp(0.0, 255.0).round() as u8,
        (avg + (pixel.b as f32 - avg) * factor).clamp(0.0, 255.0).round() as u8,
        pixel.a,
    )
}

fn apply_invert(pixel: RGBA8, _value: f32) -> RGBA8 {
    RGBA8::new(255 - pixel.r, 255 - pixel.g, 255 - pixel.b, pixel.a)
}

fn apply_grayscale(pixel: RGBA8, _value: f32) -> RGBA8 {
    let gray = (pixel.r as f32 * 0.299 + pixel.g as f32 * 0.587 + pixel.b as f32 * 0.114).round() as u8;
    RGBA8::new(gray, gray, gray, pixel.a)
}

fn apply_sepia(pixel: RGBA8, _value: f32) -> RGBA8 {
    let r = (pixel.r as f32 * 0.393 + pixel.g as f32 * 0.769 + pixel.b as f32 * 0.189).clamp(0.0, 255.0).round() as u8;
    let g = (pixel.r as f32 * 0.349 + pixel.g as f32 * 0.686 + pixel.b as f32 * 0.168).clamp(0.0, 255.0).round() as u8;
    let b = (pixel.r as f32 * 0.272 + pixel.g as f32 * 0.534 + pixel.b as f32 * 0.131).clamp(0.0, 255.0).round() as u8;
    RGBA8::new(r, g, b, pixel.a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_invert() {
        let pixel = RGBA8::new(100, 150, 200, 255);
        let result = apply_invert(pixel, 0.0);
        assert_eq!(result, RGBA8::new(155, 105, 55, 255));
    }

    #[test]
    fn test_apply_grayscale() {
        let pixel = RGBA8::new(100, 150, 200, 255);
        let result = apply_grayscale(pixel, 0.0);
        assert_eq!(result.r, result.g);
        assert_eq!(result.g, result.b);
    }

    #[test]
    fn test_apply_brightness() {
        let pixel = RGBA8::new(100, 100, 100, 255);
        let result = apply_brightness_contrast(pixel, 50.0);
        assert!(result.r > 100);
    }
}
