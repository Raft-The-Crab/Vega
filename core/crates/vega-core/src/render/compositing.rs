//! CPU compositing operations with blend modes
//!
//! Provides efficient CPU-based compositing of pixel data with various blend modes.

use crate::color::types::rgba::RGBA8;
use crate::document::nodes::BlendMode;

/// Blend operation function type.
pub type BlendOp = fn(RGBA8, RGBA8) -> RGBA8;

/// Compositor for blending pixel data.
pub struct Compositor {
    blend_mode: BlendMode,
}

impl Compositor {
    /// Creates a new compositor with the given blend mode.
    pub fn new(blend_mode: BlendMode) -> Self {
        Self { blend_mode }
    }

    /// Composites a source pixel over a destination pixel.
    pub fn composite(&self, dest: RGBA8, src: RGBA8) -> RGBA8 {
        let blend_fn = self.blend_fn();
        let blended = blend_fn(dest, src);
        alpha_blend(dest, blended)
    }

    /// Returns the blend function for the current blend mode.
    fn blend_fn(&self) -> BlendOp {
        match self.blend_mode {
            BlendMode::Normal => blend_normal,
            BlendMode::Multiply => blend_multiply,
            BlendMode::Screen => blend_screen,
            BlendMode::Overlay => blend_overlay,
            BlendMode::Darken => blend_darken,
            BlendMode::Lighten => blend_lighten,
            BlendMode::ColorDodge => blend_color_dodge,
            BlendMode::ColorBurn => blend_color_burn,
            BlendMode::HardLight => blend_hard_light,
            BlendMode::SoftLight => blend_soft_light,
            BlendMode::Difference => blend_difference,
            BlendMode::Exclusion => blend_exclusion,
            BlendMode::Hue => blend_hue,
            BlendMode::Saturation => blend_saturation,
            BlendMode::Color => blend_color,
            BlendMode::Luminosity => blend_luminosity,
        }
    }
}

/// Blends two pixels using alpha compositing (destination over source).
pub fn alpha_blend(dest: RGBA8, src: RGBA8) -> RGBA8 {
    let src_a = src.a as f32 / 255.0;
    let dest_a = dest.a as f32 / 255.0;

    if src_a == 0.0 {
        return dest;
    }
    if src_a == 1.0 {
        return src;
    }

    let out_a = src_a + dest_a * (1.0 - src_a);
    if out_a == 0.0 {
        return RGBA8::TRANSPARENT;
    }

    let src_weight = src_a / out_a;
    let dest_weight = dest_a * (1.0 - src_a) / out_a;

    RGBA8::new(
        (src.r as f32 * src_weight + dest.r as f32 * dest_weight).round() as u8,
        (src.g as f32 * src_weight + dest.g as f32 * dest_weight).round() as u8,
        (src.b as f32 * src_weight + dest.b as f32 * dest_weight).round() as u8,
        (out_a * 255.0).round() as u8,
    )
}

/// Blends two arrays of pixels.
pub fn blend_pixels(dest: &mut [RGBA8], src: &[RGBA8], blend_mode: BlendMode) {
    let compositor = Compositor::new(blend_mode);
    for (d, s) in dest.iter_mut().zip(src.iter()) {
        *d = compositor.composite(*d, *s);
    }
}

// Blend mode implementations

fn blend_normal(_dest: RGBA8, src: RGBA8) -> RGBA8 {
    src
}

fn blend_multiply(dest: RGBA8, src: RGBA8) -> RGBA8 {
    RGBA8::new(
        (dest.r as u32 * src.r as u32 / 255) as u8,
        (dest.g as u32 * src.g as u32 / 255) as u8,
        (dest.b as u32 * src.b as u32 / 255) as u8,
        src.a,
    )
}

fn blend_screen(dest: RGBA8, src: RGBA8) -> RGBA8 {
    RGBA8::new(
        (255 - (255 - dest.r as u32) * (255 - src.r as u32) / 255) as u8,
        (255 - (255 - dest.g as u32) * (255 - src.g as u32) / 255) as u8,
        (255 - (255 - dest.b as u32) * (255 - src.b as u32) / 255) as u8,
        src.a,
    )
}

fn blend_overlay(dest: RGBA8, src: RGBA8) -> RGBA8 {
    fn overlay_channel(d: u8, s: u8) -> u8 {
        if d <= 128 {
            (2 * d as u32 * s as u32 / 255) as u8
        } else {
            (255 - 2 * (255 - d as u32) * (255 - s as u32) / 255) as u8
        }
    }

    RGBA8::new(
        overlay_channel(dest.r, src.r),
        overlay_channel(dest.g, src.g),
        overlay_channel(dest.b, src.b),
        src.a,
    )
}

fn blend_darken(dest: RGBA8, src: RGBA8) -> RGBA8 {
    RGBA8::new(
        dest.r.min(src.r),
        dest.g.min(src.g),
        dest.b.min(src.b),
        src.a,
    )
}

fn blend_lighten(dest: RGBA8, src: RGBA8) -> RGBA8 {
    RGBA8::new(
        dest.r.max(src.r),
        dest.g.max(src.g),
        dest.b.max(src.b),
        src.a,
    )
}

fn blend_color_dodge(dest: RGBA8, src: RGBA8) -> RGBA8 {
    fn dodge_channel(d: u8, s: u8) -> u8 {
        if s == 255 {
            255
        } else {
            let result = (d as u32 * 255) / (255 - s as u32);
            result.min(255) as u8
        }
    }

    RGBA8::new(
        dodge_channel(dest.r, src.r),
        dodge_channel(dest.g, src.g),
        dodge_channel(dest.b, src.b),
        src.a,
    )
}

fn blend_color_burn(dest: RGBA8, src: RGBA8) -> RGBA8 {
    fn burn_channel(d: u8, s: u8) -> u8 {
        if s == 0 {
            0
        } else {
            let result = 255 - ((255 - d as u32) * 255 / s as u32);
            result.min(255) as u8
        }
    }

    RGBA8::new(
        burn_channel(dest.r, src.r),
        burn_channel(dest.g, src.g),
        burn_channel(dest.b, src.b),
        src.a,
    )
}

fn blend_hard_light(dest: RGBA8, src: RGBA8) -> blend_overlay(dest, src)

fn blend_soft_light(dest: RGBA8, src: RGBA8) -> RGBA8 {
    // Simplified soft light implementation
    fn soft_light_channel(d: u8, s: u8) -> u8 {
        let d_f = d as f32 / 255.0;
        let s_f = s as f32 / 255.0;

        if s_f <= 0.5 {
            (d_f - (1.0 - 2.0 * s_f) * d_f * (1.0 - d_f) * 255.0).round() as u8
        } else {
            let result = d_f + (2.0 * s_f - 1.0) * (((16.0 * d_f - 12.0) * d_f + 4.0) * d_f - d_f.sqrt());
            (result * 255.0).round() as u8
        }
    }

    RGBA8::new(
        soft_light_channel(dest.r, src.r),
        soft_light_channel(dest.g, src.g),
        soft_light_channel(dest.b, src.b),
        src.a,
    )
}

fn blend_difference(dest: RGBA8, src: RGBA8) -> RGBA8 {
    RGBA8::new(
        (dest.r as i32 - src.r as i32).abs() as u8,
        (dest.g as i32 - src.g as i32).abs() as u8,
        (dest.b as i32 - src.b as i32).abs() as u8,
        src.a,
    )
}

fn blend_exclusion(dest: RGBA8, src: RGBA8) -> RGBA8 {
    RGBA8::new(
        (dest.r as u32 + src.r as u32 - 2 * dest.r as u32 * src.r as u32 / 255) as u8,
        (dest.g as u32 + src.g as u32 - 2 * dest.g as u32 * src.g as u32 / 255) as u8,
        (dest.b as u32 + src.b as u32 - 2 * dest.b as u32 * src.b as u32 / 255) as u8,
        src.a,
    )
}

// HSL-based blend modes (simplified implementations)

fn blend_hue(dest: RGBA8, src: RGBA8) -> RGBA8 {
    // Simplified: just return src for now
    // Full implementation would convert to HSL, blend hue, convert back
    src
}

fn blend_saturation(dest: RGBA8, src: RGBA8) -> RGBA8 {
    // Simplified: just return src for now
    src
}

fn blend_color(dest: RGBA8, src: RGBA8) -> RGBA8 {
    // Simplified: just return src for now
    src
}

fn blend_luminosity(dest: RGBA8, src: RGBA8) -> RGBA8 {
    // Simplified: just return src for now
    src
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alpha_blend() {
        let dest = RGBA8::new(100, 150, 200, 255);
        let src = RGBA8::new(50, 75, 100, 128);
        let result = alpha_blend(dest, src);
        assert!(result.a > 128 && result.a < 255);
    }

    #[test]
    fn test_blend_normal() {
        let dest = RGBA8::BLACK;
        let src = RGBA8::WHITE;
        let compositor = Compositor::new(BlendMode::Normal);
        let result = compositor.composite(dest, src);
        assert_eq!(result, src);
    }

    #[test]
    fn test_blend_multiply() {
        let dest = RGBA8::new(200, 200, 200, 255);
        let src = RGBA8::new(128, 128, 128, 255);
        let compositor = Compositor::new(BlendMode::Multiply);
        let result = compositor.composite(dest, src);
        assert!(result.r < 200);
    }

    #[test]
    fn test_blend_screen() {
        let dest = RGBA8::new(100, 100, 100, 255);
        let src = RGBA8::new(100, 100, 100, 255);
        let compositor = Compositor::new(BlendMode::Screen);
        let result = compositor.composite(dest, src);
        assert!(result.r > 100);
    }

    #[test]
    fn test_blend_darken() {
        let dest = RGBA8::new(200, 100, 50, 255);
        let src = RGBA8::new(100, 150, 75, 255);
        let compositor = Compositor::new(BlendMode::Darken);
        let result = compositor.composite(dest, src);
        assert_eq!(result.r, 100);
        assert_eq!(result.g, 100);
        assert_eq!(result.b, 50);
    }

    #[test]
    fn test_blend_lighten() {
        let dest = RGBA8::new(100, 150, 50, 255);
        let src = RGBA8::new(200, 100, 75, 255);
        let compositor = Compositor::new(BlendMode::Lighten);
        let result = compositor.composite(dest, src);
        assert_eq!(result.r, 200);
        assert_eq!(result.g, 150);
        assert_eq!(result.b, 75);
    }

    #[test]
    fn test_blend_pixels() {
        let mut dest = vec![RGBA8::BLACK; 4];
        let src = vec![RGBA8::WHITE; 4];
        blend_pixels(&mut dest, &src, BlendMode::Normal);
        assert_eq!(dest[0], RGBA8::WHITE);
    }
}
