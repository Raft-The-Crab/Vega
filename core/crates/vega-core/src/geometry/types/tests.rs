#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_new() {
        let point = Point::new(10, 20);
        assert_eq!(point.x, 10);
        assert_eq!(point.y, 20);
    }

    #[test]
    fn test_point_origin() {
        let point = Point::origin();
        assert_eq!(point.x, 0);
        assert_eq!(point.y, 0);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(10, 20);
        let p2 = Point::new(5, 15);
        let result = p1.add(p2);
        assert_eq!(result.x, 15);
        assert_eq!(result.y, 35);
    }

    #[test]
    fn test_point_sub() {
        let p1 = Point::new(10, 20);
        let p2 = Point::new(5, 15);
        let result = p1.sub(p2);
        assert_eq!(result.x, 5);
        assert_eq!(result.y, 5);
    }

    #[test]
    fn test_point_manhattan_distance() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(10, 20);
        assert_eq!(p1.manhattan_distance(p2), 30);
    }

    #[test]
    fn test_size_new() {
        let size = Size::new(100, 200);
        assert_eq!(size.width, 100);
        assert_eq!(size.height, 200);
    }

    #[test]
    fn test_size_zero() {
        let size = Size::zero();
        assert_eq!(size.width, 0);
        assert_eq!(size.height, 0);
    }

    #[test]
    fn test_size_is_zero() {
        assert!(Size::zero().is_zero());
        assert!(Size::new(0, 100).is_zero());
        assert!(Size::new(100, 0).is_zero());
        assert!(!Size::new(100, 100).is_zero());
    }

    #[test]
    fn test_size_area() {
        let size = Size::new(10, 20);
        assert_eq!(size.area(), 200);
    }

    #[test]
    fn test_size_contains() {
        let size1 = Size::new(100, 100);
        let size2 = Size::new(50, 50);
        assert!(size1.contains(size2));
        assert!(!size2.contains(size1));
    }

    #[test]
    fn test_size_clamp() {
        let size = Size::new(150, 250);
        let clamped = size.clamp(100, 200);
        assert_eq!(clamped.width, 100);
        assert_eq!(clamped.height, 200);
    }

    #[test]
    fn test_rect_new() {
        let rect = Rect::new(10, 20, 100, 200);
        assert_eq!(rect.x, 10);
        assert_eq!(rect.y, 20);
        assert_eq!(rect.width, 100);
        assert_eq!(rect.height, 200);
    }

    #[test]
    fn test_rect_from_origin_size() {
        let origin = Point::new(10, 20);
        let size = Size::new(100, 200);
        let rect = Rect::from_origin_size(origin, size);
        assert_eq!(rect.x, 10);
        assert_eq!(rect.y, 20);
        assert_eq!(rect.width, 100);
        assert_eq!(rect.height, 200);
    }

    #[test]
    fn test_rect_from_size() {
        let size = Size::new(100, 200);
        let rect = Rect::from_size(size);
        assert_eq!(rect.x, 0);
        assert_eq!(rect.y, 0);
        assert_eq!(rect.width, 100);
        assert_eq!(rect.height, 200);
    }

    #[test]
    fn test_rect_origin() {
        let rect = Rect::new(10, 20, 100, 200);
        let origin = rect.origin();
        assert_eq!(origin.x, 10);
        assert_eq!(origin.y, 20);
    }

    #[test]
    fn test_rect_size() {
        let rect = Rect::new(10, 20, 100, 200);
        let size = rect.size();
        assert_eq!(size.width, 100);
        assert_eq!(size.height, 200);
    }

    #[test]
    fn test_rect_right() {
        let rect = Rect::new(10, 20, 100, 200);
        assert_eq!(rect.right(), 110);
    }

    #[test]
    fn test_rect_bottom() {
        let rect = Rect::new(10, 20, 100, 200);
        assert_eq!(rect.bottom(), 220);
    }

    #[test]
    fn test_rect_is_empty() {
        assert!(Rect::new(0, 0, 0, 100).is_empty());
        assert!(Rect::new(0, 0, 100, 0).is_empty());
        assert!(!Rect::new(0, 0, 100, 100).is_empty());
    }

    #[test]
    fn test_rect_contains_point() {
        let rect = Rect::new(10, 20, 100, 200);
        assert!(rect.contains_point(Point::new(10, 20)));
        assert!(rect.contains_point(Point::new(50, 100)));
        assert!(!rect.contains_point(Point::new(5, 20)));
        assert!(!rect.contains_point(Point::new(110, 20)));
    }

    #[test]
    fn test_rect_contains_rect() {
        let rect1 = Rect::new(0, 0, 100, 100);
        let rect2 = Rect::new(10, 10, 50, 50);
        assert!(rect1.contains_rect(rect2));
        assert!(!rect2.contains_rect(rect1));
    }

    #[test]
    fn test_rect_intersects() {
        let rect1 = Rect::new(0, 0, 100, 100);
        let rect2 = Rect::new(50, 50, 100, 100);
        assert!(rect1.intersects(rect2));

        let rect3 = Rect::new(200, 200, 100, 100);
        assert!(!rect1.intersects(rect3));
    }

    #[test]
    fn test_rect_intersection() {
        let rect1 = Rect::new(0, 0, 100, 100);
        let rect2 = Rect::new(50, 50, 100, 100);
        let intersection = rect1.intersection(rect2).unwrap();
        assert_eq!(intersection.x, 50);
        assert_eq!(intersection.y, 50);
        assert_eq!(intersection.width, 50);
        assert_eq!(intersection.height, 50);
    }

    #[test]
    fn test_rect_intersection_none() {
        let rect1 = Rect::new(0, 0, 100, 100);
        let rect2 = Rect::new(200, 200, 100, 100);
        assert!(rect1.intersection(rect2).is_none());
    }

    #[test]
    fn test_rect_union() {
        let rect1 = Rect::new(0, 0, 100, 100);
        let rect2 = Rect::new(50, 50, 100, 100);
        let union = rect1.union(rect2);
        assert_eq!(union.x, 0);
        assert_eq!(union.y, 0);
        assert_eq!(union.width, 150);
        assert_eq!(union.height, 150);
    }

    #[test]
    fn test_rect_inflate() {
        let rect = Rect::new(10, 20, 100, 200);
        let inflated = rect.inflate(10);
        assert_eq!(inflated.x, 0);
        assert_eq!(inflated.y, 10);
        assert_eq!(inflated.width, 120);
        assert_eq!(inflated.height, 220);
    }
}
