//! Color space definitions.

use serde::{Deserialize, Serialize};

/// Color space definitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorSpace {
    /// sRGB color space (standard for web and most displays).
    Srgb,

    /// Linear RGB (no gamma correction).
    LinearRgb,

    /// Adobe RGB (wider gamut than sRGB).
    AdobeRgb,

    /// Display P3 (common on Apple displays).
    DisplayP3,

    /// ProPhoto RGB (very wide gamut for professional photography).
    ProPhotoRgb,

    /// CIE XYZ (device-independent color space).
    CieXyz,

    /// CIE LAB (perceptually uniform color space).
    CieLab,

    /// Grayscale.
    Grayscale,

    /// CMYK (for print output).
    Cmyk,
}

impl ColorSpace {
    /// Check if this is a linear color space.
    pub fn is_linear(self) -> bool {
        matches!(self, ColorSpace::LinearRgb | ColorSpace::CieXyz)
    }

    /// Check if this is a device-independent color space.
    pub fn is_device_independent(self) -> bool {
        matches!(self, ColorSpace::CieXyz | ColorSpace::CieLab)
    }

    /// Get the default color space for new documents.
    pub fn default() -> Self {
        ColorSpace::Srgb
    }
}

impl Default for ColorSpace {
    fn default() -> Self {
        Self::default()
    }
}

impl std::fmt::Display for ColorSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorSpace::Srgb => write!(f, "sRGB"),
            ColorSpace::LinearRgb => write!(f, "Linear RGB"),
            ColorSpace::AdobeRgb => write!(f, "Adobe RGB"),
            ColorSpace::DisplayP3 => write!(f, "Display P3"),
            ColorSpace::ProPhotoRgb => write!(f, "ProPhoto RGB"),
            ColorSpace::CieXyz => write!(f, "CIE XYZ"),
            ColorSpace::CieLab => write!(f, "CIE LAB"),
            ColorSpace::Grayscale => write!(f, "Grayscale"),
            ColorSpace::Cmyk => write!(f, "CMYK"),
        }
    }
}
