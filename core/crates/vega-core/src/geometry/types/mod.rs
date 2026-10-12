//! Basic geometry types for 2D operations.

use serde::{Deserialize, Serialize};
use std::fmt;

/// A 2D point with integer coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Create a new point.
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// The origin point (0, 0).
    pub const fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    /// Add another point to this one.
    pub fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    /// Subtract another point from this one.
    pub fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    /// Get the Manhattan distance to another point.
    pub fn manhattan_distance(self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// A 2D size with integer dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    /// Create a new size.
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// A zero size.
    pub const fn zero() -> Self {
        Self { width: 0, height: 0 }
    }

    /// Check if the size is zero.
    pub fn is_zero(self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Calculate the area.
    pub fn area(self) -> u64 {
        self.width as u64 * self.height as u64
    }

    /// Check if this size can contain another size.
    pub fn contains(self, other: Size) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    /// Clamp the size to maximum dimensions.
    pub fn clamp(self, max_width: u32, max_height: u32) -> Size {
        Size {
            width: self.width.min(max_width),
            height: self.height.min(max_height),
        }
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

/// A 2D rectangle with integer bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    /// Create a new rectangle.
    pub const fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }

    /// Create a rectangle from origin and size.
    pub const fn from_origin_size(origin: Point, size: Size) -> Self {
        Self {
            x: origin.x,
            y: origin.y,
            width: size.width,
            height: size.height,
        }
    }

    /// Create a rectangle covering the entire area of a given size starting at origin.
    pub const fn from_size(size: Size) -> Self {
        Self {
            x: 0,
            y: 0,
            width: size.width,
            height: size.height,
        }
    }

    /// Get the origin (top-left corner).
    pub const fn origin(self) -> Point {
        Point { x: self.x, y: self.y }
    }

    /// Get the size.
    pub const fn size(self) -> Size {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    /// Get the right edge (x + width).
    pub const fn right(self) -> i32 {
        self.x + self.width as i32
    }

    /// Get the bottom edge (y + height).
    pub const fn bottom(self) -> i32 {
        self.y + self.height as i32
    }

    /// Check if the rectangle is empty (zero width or height).
    pub fn is_empty(self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Check if a point is contained within the rectangle.
    pub fn contains_point(self, point: Point) -> bool {
        point.x >= self.x
            && point.x < self.right()
            && point.y >= self.y
            && point.y < self.bottom()
    }

    /// Check if this rectangle completely contains another rectangle.
    pub fn contains_rect(self, other: Rect) -> bool {
        other.x >= self.x
            && other.right() <= self.right()
            && other.y >= self.y
            && other.bottom() <= self.bottom()
    }

    /// Check if this rectangle intersects with another.
    pub fn intersects(self, other: Rect) -> bool {
        self.x < other.right()
            && self.right() > other.x
            && self.y < other.bottom()
            && self.bottom() > other.y
    }

    /// Compute the intersection of two rectangles.
    pub fn intersection(self, other: Rect) -> Option<Rect> {
        if !self.intersects(other) {
            return None;
        }

        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let right = self.right().min(other.right());
        let bottom = self.bottom().min(other.bottom());

        Some(Rect {
            x,
            y,
            width: (right - x) as u32,
            height: (bottom - y) as u32,
        })
    }

    /// Compute the union of two rectangles.
    pub fn union(self, other: Rect) -> Rect {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let right = self.right().max(other.right());
        let bottom = self.bottom().max(other.bottom());

        Rect {
            x,
            y,
            width: (right - x) as u32,
            height: (bottom - y) as u32,
        }
    }

    /// Expand the rectangle by a margin on all sides.
    pub fn inflate(self, margin: i32) -> Rect {
        Rect {
            x: self.x - margin,
            y: self.y - margin,
            width: (self.width as i32 + 2 * margin) as u32,
            height: (self.height as i32 + 2 * margin) as u32,
        }
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.width, self.height)
    }
}

pub mod tests;
