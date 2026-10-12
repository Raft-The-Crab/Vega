//! RGBA color types with various bit depths.

use serde::{Deserialize, Serialize};
use std::fmt;

/// A color in RGBA format with 8 bits per channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rgba8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba8 {
    /// Create a new RGBA8 color.
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create an opaque RGBA8 color.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create a transparent RGBA8 color.
    pub const fn transparent() -> Self {
        Self { r: 0, g: 0, b: 0, a: 0 }
    }

    /// Get the alpha value as a float in [0.0, 1.0].
    pub fn alpha_f32(self) -> f32 {
        self.a as f32 / 255.0
    }

    /// Convert to premultiplied alpha.
    pub fn to_premultiplied(self) -> Self {
        let a = self.alpha_f32();
        Self {
            r: (self.r as f32 * a).round() as u8,
            g: (self.g as f32 * a).round() as u8,
            b: (self.b as f32 * a).round() as u8,
            a: self.a,
        }
    }

    /// Convert from premultiplied alpha.
    pub fn from_premultiplied(self) -> Self {
        if self.a == 0 {
            return Self::transparent();
        }
        let a = self.alpha_f32();
        Self {
            r: (self.r as f32 / a).round() as u8,
            g: (self.g as f32 / a).round() as u8,
            b: (self.b as f32 / a).round() as u8,
            a: self.a,
        }
    }
}

impl fmt::Display for Rgba8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgba8({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

/// A color in RGBA format with 16 bits per channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rgba16 {
    pub r: u16,
    pub g: u16,
    pub b: u16,
    pub a: u16,
}

impl Rgba16 {
    /// Create a new RGBA16 color.
    pub const fn new(r: u16, g: u16, b: u16, a: u16) -> Self {
        Self { r, g, b, a }
    }

    /// Create an opaque RGBA16 color.
    pub const fn rgb(r: u16, g: u16, b: u16) -> Self {
        Self { r, g, b, a: 65535 }
    }

    /// Create a transparent RGBA16 color.
    pub const fn transparent() -> Self {
        Self { r: 0, g: 0, b: 0, a: 0 }
    }

    /// Get the alpha value as a float in [0.0, 1.0].
    pub fn alpha_f32(self) -> f32 {
        self.a as f32 / 65535.0
    }

    /// Convert from RGBA8 to RGBA16.
    pub fn from_rgba8(color: Rgba8) -> Self {
        Self {
            r: ((color.r as u16) << 8) | (color.r as u16),
            g: ((color.g as u16) << 8) | (color.g as u16),
            b: ((color.b as u16) << 8) | (color.b as u16),
            a: ((color.a as u16) << 8) | (color.a as u16),
        }
    }

    /// Convert to RGBA8.
    pub fn to_rgba8(self) -> Rgba8 {
        Rgba8 {
            r: (self.r >> 8) as u8,
            g: (self.g >> 8) as u8,
            b: (self.b >> 8) as u8,
            a: (self.a >> 8) as u8,
        }
    }
}

impl fmt::Display for Rgba16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgba16({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

/// A color in RGBA format with 32 bits per channel (float).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rgba32 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba32 {
    /// Create a new RGBA32 color. Values should be in [0.0, 1.0].
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Create an opaque RGBA32 color.
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Create a transparent RGBA32 color.
    pub fn transparent() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
    }

    /// Clamp all channels to [0.0, 1.0].
    pub fn clamp(self) -> Self {
        Self {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
            a: self.a.clamp(0.0, 1.0),
        }
    }

    /// Convert to premultiplied alpha.
    pub fn to_premultiplied(self) -> Self {
        Self {
            r: self.r * self.a,
            g: self.g * self.a,
            b: self.b * self.a,
            a: self.a,
        }
    }

    /// Convert from premultiplied alpha.
    pub fn from_premultiplied(self) -> Self {
        if self.a == 0.0 {
            return Self::transparent();
        }
        Self {
            r: self.r / self.a,
            g: self.g / self.a,
            b: self.b / self.a,
            a: self.a,
        }
    }

    /// Convert from RGBA8.
    pub fn from_rgba8(color: Rgba8) -> Self {
        Self {
            r: color.r as f32 / 255.0,
            g: color.g as f32 / 255.0,
            b: color.b as f32 / 255.0,
            a: color.a as f32 / 255.0,
        }
    }

    /// Convert to RGBA8.
    pub fn to_rgba8(self) -> Rgba8 {
        let self = self.clamp();
        Rgba8 {
            r: (self.r * 255.0).round() as u8,
            g: (self.g * 255.0).round() as u8,
            b: (self.b * 255.0).round() as u8,
            a: (self.a * 255.0).round() as u8,
        }
    }
}

impl fmt::Display for Rgba32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgba32({:.3}, {:.3}, {:.3}, {:.3})", self.r, self.g, self.b, self.a)
    }
}

/// Generic RGBA color type alias for convenience.
pub type Rgba = Rgba8;

#[cfg(test)]
mod tests;
