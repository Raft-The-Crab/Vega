//! HSL color space conversions and blend modes
//!
//! Provides HSL (Hue, Saturation, Luminosity) color space operations
//! for advanced blend modes.

use crate::color::types::Rgba8;

/// HSL color representation.
#[derive(Debug, Clone, Copy)]
pub struct Hsl {
    /// Hue: 0.0 to 360.0 degrees
    pub h: f32,
    /// Saturation: 0.0 to 1.0
    pub s: f32,
    /// Lightness: 0.0 to 1.0
    pub l: f32,
}

impl Hsl {
    /// Creates a new HSL color.
    pub fn new(h: f32, s: f32, l: f32) -> Self {
        Self { h, s, l }
    }

    /// Converts RGB to HSL.
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let l = (max + min) / 2.0;

        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };

        Self {
            h: h % 360.0,
            s: s.clamp(0.0, 1.0),
            l: l.clamp(0.0, 1.0),
        }
    }

    /// Converts HSL to RGB.
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        let c = (1.0 - (2.0 * self.l - 1.0).abs()) * self.s;
        let x = c * (1.0 - ((self.h / 60.0) % 2.0 - 1.0).abs());
        let m = self.l - c / 2.0;

        let (r, g, b) = if self.h < 60.0 {
            (c, x, 0.0)
        } else if self.h < 120.0 {
            (x, c, 0.0)
        } else if self.h < 180.0 {
            (0.0, c, x)
        } else if self.h < 240.0 {
            (0.0, x, c)
        } else if self.h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        (
            ((r + m) * 255.0).round() as u8,
            ((g + m) * 255.0).round() as u8,
            ((b + m) * 255.0).round() as u8,
        )
    }

    /// Sets the hue.
    pub fn with_hue(mut self, h: f32) -> Self {
        self.h = h % 360.0;
        self
    }

    /// Sets the saturation.
    pub fn with_saturation(mut self, s: f32) -> Self {
        self.s = s.clamp(0.0, 1.0);
        self
    }

    /// Sets the luminosity.
    pub fn with_luminosity(mut self, l: f32) -> Self {
        self.l = l.clamp(0.0, 1.0);
        self
    }
}

/// Helper function to get luminance of a color (for luminosity blend mode).
pub fn luminance(r: u8, g: u8, b: u8) -> f32 {
    // Rec. 709 luma coefficients
    0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32
}

/// HSL blend mode implementations.

/// Hue blend mode: takes hue from source, saturation and luminosity from destination.
pub fn blend_hue(dest: RGBA8, src: RGBA8) -> RGBA8 {
    let dest_hsl = Hsl::from_rgb(dest.r, dest.g, dest.b);
    let src_hsl = Hsl::from_rgb(src.r, src.g, src.b);

    let result = Hsl::new(src_hsl.h, dest_hsl.s, dest_hsl.l);
    let (r, g, b) = result.to_rgb();

    RGBA8::new(r, g, b, src.a)
}

/// Saturation blend mode: takes saturation from source, hue and luminosity from destination.
pub fn blend_saturation(dest: RGBA8, src: RGBA8) -> RGBA8 {
    let dest_hsl = Hsl::from_rgb(dest.r, dest.g, dest.b);
    let src_hsl = Hsl::from_rgb(src.r, src.g, src.b);

    let result = Hsl::new(dest_hsl.h, src_hsl.s, dest_hsl.l);
    let (r, g, b) = result.to_rgb();

    RGBA8::new(r, g, b, src.a)
}

/// Color blend mode: takes hue and saturation from source, luminosity from destination.
pub fn blend_color(dest: RGBA8, src: RGBA8) -> RGBA8 {
    let dest_hsl = Hsl::from_rgb(dest.r, dest.g, dest.b);
    let src_hsl = Hsl::from_rgb(src.r, src.g, src.b);

    let result = Hsl::new(src_hsl.h, src_hsl.s, dest_hsl.l);
    let (r, g, b) = result.to_rgb();

    RGBA8::new(r, g, b, src.a)
}

/// Luminosity blend mode: takes luminosity from source, hue and saturation from destination.
pub fn blend_luminosity(dest: RGBA8, src: RGBA8) -> RGBA8 {
    let dest_hsl = Hsl::from_rgb(dest.r, dest.g, dest.b);
    let src_hsl = Hsl::from_rgb(src.r, src.g, src.b);

    let result = Hsl::new(dest_hsl.h, dest_hsl.s, src_hsl.l);
    let (r, g, b) = result.to_rgb();

    RGBA8::new(r, g, b, src.a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsl_conversion() {
        // Test red to HSL
        let hsl = Hsl::from_rgb(255, 0, 0);
        assert_eq!(hsl.h, 0.0);
        assert_eq!(hsl.s, 1.0);
        assert_eq!(hsl.l, 0.5);

        // Test HSL to RGB
        let (r, g, b) = hsl.to_rgb();
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
    }

    #[test]
    fn test_hsl_white() {
        let hsl = Hsl::from_rgb(255, 255, 255);
        assert_eq!(hsl.l, 1.0);
        assert_eq!(hsl.s, 0.0);
    }

    #[test]
    fn test_hsl_black() {
        let hsl = Hsl::from_rgb(0, 0, 0);
        assert_eq!(hsl.l, 0.0);
        assert_eq!(hsl.s, 0.0);
    }

    #[test]
    fn test_blend_hue() {
        let dest = RGBA8::new(100, 100, 100, 255);
        let src = RGBA8::new(255, 0, 0, 255);
        let result = blend_hue(dest, src);
        // Result should have hue from red (0 degrees) but saturation/luminosity from gray
        assert!(result.r > 100); // More red due to hue
    }

    #[test]
    fn test_blend_saturation() {
        let dest = RGBA8::new(128, 128, 128, 255);
        let src = RGBA8::new(255, 0, 0, 255);
        let result = blend_saturation(dest, src);
        // Result should have saturation from red
        assert_ne!(result.r, result.g);
    }

    #[test]
    fn test_luminance() {
        let lum = luminance(255, 255, 255);
        assert_eq!(lum, 255.0);

        let lum = luminance(0, 0, 0);
        assert_eq!(lum, 0.0);
    }
}
