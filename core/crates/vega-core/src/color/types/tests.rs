#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgba8_new() {
        let color = Rgba8::new(255, 128, 64, 255);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_rgba8_rgb() {
        let color = Rgba8::rgb(100, 150, 200);
        assert_eq!(color.r, 100);
        assert_eq!(color.g, 150);
        assert_eq!(color.b, 200);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_rgba8_transparent() {
        let color = Rgba8::transparent();
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 0);
    }

    #[test]
    fn test_rgba8_alpha_f32() {
        let color = Rgba8::new(0, 0, 0, 128);
        assert!((color.alpha_f32() - 0.502).abs() < 0.01);
    }

    #[test]
    fn test_rgba8_to_premultiplied() {
        let color = Rgba8::new(255, 128, 64, 128);
        let premultiplied = color.to_premultiplied();
        assert_eq!(premultiplied.a, 128);
        assert!(premultiplied.r <= 255);
        assert!(premultiplied.g <= 255);
        assert!(premultiplied.b <= 255);
    }

    #[test]
    fn test_rgba8_from_premultiplied() {
        let color = Rgba8::new(255, 128, 64, 128);
        let premultiplied = color.to_premultiplied();
        let restored = premultiplied.from_premultiplied();
        assert_eq!(restored.a, 128);
        // Values should be close to original (rounding errors possible)
        assert!((restored.r as i32 - color.r as i32).abs() <= 1);
    }

    #[test]
    fn test_rgba16_new() {
        let color = Rgba16::new(65535, 32768, 16384, 65535);
        assert_eq!(color.r, 65535);
        assert_eq!(color.g, 32768);
        assert_eq!(color.b, 16384);
        assert_eq!(color.a, 65535);
    }

    #[test]
    fn test_rgba16_from_rgba8() {
        let rgba8 = Rgba8::new(255, 128, 64, 255);
        let rgba16 = Rgba16::from_rgba8(rgba8);
        assert_eq!(rgba16.r, 65535);
        assert_eq!(rgba16.g, 32768);
        assert_eq!(rgba16.b, 16384);
        assert_eq!(rgba16.a, 65535);
    }

    #[test]
    fn test_rgba16_to_rgba8() {
        let rgba16 = Rgba16::new(65535, 32768, 16384, 65535);
        let rgba8 = rgba16.to_rgba8();
        assert_eq!(rgba8.r, 255);
        assert_eq!(rgba8.g, 128);
        assert_eq!(rgba8.b, 64);
        assert_eq!(rgba8.a, 255);
    }

    #[test]
    fn test_rgba32_new() {
        let color = Rgba32::new(1.0, 0.5, 0.25, 1.0);
        assert_eq!(color.r, 1.0);
        assert_eq!(color.g, 0.5);
        assert_eq!(color.b, 0.25);
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn test_rgba32_clamp() {
        let color = Rgba32::new(1.5, -0.5, 0.5, 2.0);
        let clamped = color.clamp();
        assert_eq!(clamped.r, 1.0);
        assert_eq!(clamped.g, 0.0);
        assert_eq!(clamped.b, 0.5);
        assert_eq!(clamped.a, 1.0);
    }

    #[test]
    fn test_rgba32_to_premultiplied() {
        let color = Rgba32::new(1.0, 0.5, 0.25, 0.5);
        let premultiplied = color.to_premultiplied();
        assert_eq!(premultiplied.r, 0.5);
        assert_eq!(premultiplied.g, 0.25);
        assert_eq!(premultiplied.b, 0.125);
        assert_eq!(premultiplied.a, 0.5);
    }

    #[test]
    fn test_rgba32_from_premultiplied() {
        let color = Rgba32::new(1.0, 0.5, 0.25, 0.5);
        let premultiplied = color.to_premultiplied();
        let restored = premultiplied.from_premultiplied();
        assert_eq!(restored.r, 1.0);
        assert_eq!(restored.g, 0.5);
        assert_eq!(restored.b, 0.25);
        assert_eq!(restored.a, 0.5);
    }

    #[test]
    fn test_rgba32_from_rgba8() {
        let rgba8 = Rgba8::new(255, 128, 64, 255);
        let rgba32 = Rgba32::from_rgba8(rgba8);
        assert!((rgba32.r - 1.0).abs() < 0.01);
        assert!((rgba32.g - 0.502).abs() < 0.01);
        assert!((rgba32.b - 0.251).abs() < 0.01);
        assert_eq!(rgba32.a, 1.0);
    }

    #[test]
    fn test_rgba32_to_rgba8() {
        let rgba32 = Rgba32::new(1.0, 0.5, 0.25, 1.0);
        let rgba8 = rgba32.to_rgba8();
        assert_eq!(rgba8.r, 255);
        assert_eq!(rgba8.g, 128);
        assert_eq!(rgba8.b, 64);
        assert_eq!(rgba8.a, 255);
    }

    #[test]
    fn test_rgba32_transparent() {
        let color = Rgba32::transparent();
        assert_eq!(color.r, 0.0);
        assert_eq!(color.g, 0.0);
        assert_eq!(color.b, 0.0);
        assert_eq!(color.a, 0.0);
    }
}
