//! Transform operations for rendering
//!
//! Provides transform application for rendering with scale, rotation, and translation.

use crate::geometry::{Point, Rect};

/// Applies a transform to a point.
///
/// # Arguments
/// * `point` - The point to transform
/// * `translation` - (x, y) translation
/// * `scale` - (x, y) scale factors
/// * `rotation` - Rotation in degrees
///
/// # Returns
/// The transformed point.
pub fn transform_point(
    point: Point,
    translation: (f32, f32),
    scale: (f32, f32),
    rotation: f32,
) -> Point {
    // Apply scale
    let x = point.x as f32 * scale.0;
    let y = point.y as f32 * scale.1;

    // Apply rotation (counterclockwise)
    let rotation_rad = rotation.to_radians();
    let cos = rotation_rad.cos();
    let sin = rotation_rad.sin();

    let x_rot = x * cos - y * sin;
    let y_rot = x * sin + y * cos;

    // Apply translation
    let x_final = x_rot + translation.0;
    let y_final = y_rot + translation.1;

    Point::new(x_final.round() as i32, y_final.round() as i32)
}

/// Applies a transform to a rectangle.
///
/// # Arguments
/// * `rect` - The rectangle to transform
/// * `translation` - (x, y) translation
/// * `scale` - (x, y) scale factors
/// * `rotation` - Rotation in degrees
///
/// # Returns
/// The transformed rectangle (bounding box of the transformed corners).
pub fn transform_rect(
    rect: Rect,
    translation: (f32, f32),
    scale: (f32, f32),
    rotation: f32,
) -> Rect {
    // Transform all four corners
    let corners = [
        Point::new(rect.x, rect.y),
        Point::new(rect.x + rect.width as i32, rect.y),
        Point::new(rect.x, rect.y + rect.height as i32),
        Point::new(rect.x + rect.width as i32, rect.y + rect.height as i32),
    ];

    let transformed: Vec<Point> = corners
        .iter()
        .map(|&p| transform_point(p, translation, scale, rotation))
        .collect();

    // Find bounding box
    let min_x = transformed.iter().map(|p| p.x).min().unwrap();
    let min_y = transformed.iter().map(|p| p.y).min().unwrap();
    let max_x = transformed.iter().map(|p| p.x).max().unwrap();
    let max_y = transformed.iter().map(|p| p.y).max().unwrap();

    Rect::new(
        min_x,
        min_y,
        (max_x - min_x) as u32,
        (max_y - min_y) as u32,
    )
}

/// Inverse transform a point (from screen space to document space).
///
/// This is useful for hit testing and determining which document coordinate
/// corresponds to a screen coordinate.
pub fn inverse_transform_point(
    point: Point,
    translation: (f32, f32),
    scale: (f32, f32),
    rotation: f32,
) -> Point {
    // Apply inverse translation
    let x = point.x as f32 - translation.0;
    let y = point.y as f32 - translation.1;

    // Apply inverse rotation (clockwise)
    let rotation_rad = (-rotation).to_radians();
    let cos = rotation_rad.cos();
    let sin = rotation_rad.sin();

    let x_rot = x * cos - y * sin;
    let y_rot = x * sin + y * cos;

    // Apply inverse scale
    let x_final = x_rot / scale.0;
    let y_final = y_rot / scale.1;

    Point::new(x_final.round() as i32, y_final.round() as i32)
}

/// Calculates the bounding box of a transformed rectangle without actually transforming it.
///
/// This is more efficient than transforming all corners when you only need the bounds.
pub fn transformed_bounds(
    rect: Rect,
    translation: (f32, f32),
    scale: (f32, f32),
    rotation: f32,
) -> Rect {
    transform_rect(rect, translation, scale, rotation)
}

/// Checks if a transform has any effect (is not identity).
pub fn is_identity_transform(
    translation: (f32, f32),
    scale: (f32, f32),
    rotation: f32,
) -> bool {
    translation.0 == 0.0
        && translation.1 == 0.0
        && scale.0 == 1.0
        && scale.1 == 1.0
        && rotation == 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_point_identity() {
        let point = Point::new(10, 20);
        let transformed = transform_point(point, (0.0, 0.0), (1.0, 1.0), 0.0);
        assert_eq!(transformed, point);
    }

    #[test]
    fn test_transform_point_translation() {
        let point = Point::new(10, 20);
        let transformed = transform_point(point, (5.0, 10.0), (1.0, 1.0), 0.0);
        assert_eq!(transformed, Point::new(15, 30));
    }

    #[test]
    fn test_transform_point_scale() {
        let point = Point::new(10, 20);
        let transformed = transform_point(point, (0.0, 0.0), (2.0, 2.0), 0.0);
        assert_eq!(transformed, Point::new(20, 40));
    }

    #[test]
    fn test_transform_point_rotation() {
        let point = Point::new(10, 0);
        let transformed = transform_point(point, (0.0, 0.0), (1.0, 1.0), 90.0);
        // 90 degrees rotation should move (10, 0) to approximately (0, 10)
        assert!(transformed.x.abs() < 2);
        assert!(transformed.y > 8);
    }

    #[test]
    fn test_transform_rect() {
        let rect = Rect::new(0, 0, 100, 100);
        let transformed = transform_rect(rect, (10.0, 10.0), (1.0, 1.0), 0.0);
        assert_eq!(transformed, Rect::new(10, 10, 100, 100));
    }

    #[test]
    fn test_inverse_transform() {
        let point = Point::new(15, 30);
        let transformed = inverse_transform_point(point, (5.0, 10.0), (1.0, 1.0), 0.0);
        assert_eq!(transformed, Point::new(10, 20));
    }

    #[test]
    fn test_is_identity_transform() {
        assert!(is_identity_transform((0.0, 0.0), (1.0, 1.0), 0.0));
        assert!(!is_identity_transform((1.0, 0.0), (1.0, 1.0), 0.0));
        assert!(!is_identity_transform((0.0, 0.0), (2.0, 1.0), 0.0));
        assert!(!is_identity_transform((0.0, 0.0), (1.0, 1.0), 45.0));
    }
}
