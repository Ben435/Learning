use crate::physics::*;

pub fn distance_between_points(p1: Point, p2: Point) -> f32 {
    // Pythag formula
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

pub fn point_within_rect(p1: Point, rect: Rectangle) -> bool {
    if p1.x > rect.origin.x && p1.x < (rect.origin.x + rect.width) {
        // Within horizontal
        if p1.y > rect.origin.y && p1.y < (rect.origin.y + rect.height) {
            // Within vertical
            return true;
        }
    }

    return false;
}

// distance_between_line_and_point: calculate distance between line and point.
//  Based on https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line#Line_defined_by_two_points
//  Returns absolute distance between a point p0 and the closest point along the line defined by p1 and p2
pub fn distance_between_line_and_point(p1: Point, p2: Point, p0: Point) -> f32 {
    let y_diff = p2.y - p1.y;
    let x_diff = p2.x - p1.x;

    let denominator = (y_diff * p0.x - x_diff * p0.y + p2.x * p1.y - p2.y * p1.x).abs();
    let numerator = (y_diff.powi(2) + x_diff.powi(2)).sqrt();

    denominator / numerator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_between_points_cases() {
        assert_eq!(distance_between_points(Point::new(1.0, 0.0), Point::new(3.0, 0.0)), 2.0);
        assert_eq!(distance_between_points(Point::new(-1.0, 0.0), Point::new(1.0, 0.0)), 2.0);
        assert_eq!(distance_between_points(Point::new(-3.0, 0.0), Point::new(-1.0, 0.0)), 2.0);
        assert_eq!(distance_between_points(Point::new(0.0, 0.0), Point::new(3.0, 4.0)), 5.0);
    }

    #[test]
    fn dist_between_ln_and_pt_simple() {
        let p1 = Point{x: 0.0, y: 0.0};
        let p2 = Point{x: 1.0, y: 0.0};
        let p0 = Point{x: 0.0, y: 1.0};

        assert_eq!(distance_between_line_and_point(p1, p2, p0), 1.0);
    }

    #[test]
    fn dist_between_ln_and_pt_center_above_line() {
        let p1 = Point{x: 0.0, y: 0.0};
        let p2 = Point{x: 2.0, y: 0.0};
        let p0 = Point{x: 1.0, y: 1.0};

        assert_eq!(distance_between_line_and_point(p1, p2, p0), 1.0);
    }

    #[test]
    fn dist_between_ln_and_pt_center_below_line() {
        let p1 = Point{x: 0.0, y: 0.0};
        let p2 = Point{x: 2.0, y: 0.0};
        let p0 = Point{x: 1.0, y: -1.0};

        assert_eq!(distance_between_line_and_point(p1, p2, p0), 1.0);
    }

    #[test]
    fn point_within_rect_within_x_not_y() {
        let rect = Rectangle::new(0.0, 0.0, 1.0, 1.0);
        let p = Point::new(0.5, 2.0);

        assert_eq!(point_within_rect(p, rect), false)
    }

    #[test]
    fn point_within_rect_within_y_not_x() {
        let rect = Rectangle::new(0.0, 0.0, 1.0, 1.0);
        let p = Point::new(2.0, 0.5);

        assert_eq!(point_within_rect(p, rect), false)
    }

    #[test]
    fn point_within_rect_within_x_and_y() {
        let rect = Rectangle::new(0.0, 0.0, 1.0, 1.0);
        let p = Point::new(0.1, 0.5);

        assert_eq!(point_within_rect(p, rect), true)
    }
}
