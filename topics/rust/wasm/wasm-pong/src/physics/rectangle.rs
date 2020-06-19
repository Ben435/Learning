use wasm_bindgen::prelude::*;
use crate::physics::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub origin: Point,  // Top left.
    pub width: f32,
    pub height: f32,
}

impl CollideWith<Point> for Rectangle {
    fn collision(&self, other: Point) -> Option<CollisionType> {
        let top_left_corner = Point{x: self.origin.x, y: self.origin.y};
        let top_right_corner = Point{x: self.origin.x + self.width, y: self.origin.y};
        let bottom_left_corner = Point{x: self.origin.x, y: self.origin.y + self.height};
        let bottom_right_corner = Point{x: self.origin.x + self.width, y: self.origin.y + self.height};

        // Within horizontal bounds
        if other.x > self.origin.x && other.x < self.origin.x + self.width {
            // Within vertical bounds
            if other.y > self.origin.y && other.y < self.origin.y + self.height {
                // We're at a collision, just need to figure out which side.
                let distance_to_top_edge = distance_between_line_and_point(top_left_corner, top_right_corner, other);
                let distance_to_bottom_edge = distance_between_line_and_point(bottom_left_corner, bottom_right_corner, other);
                let minimal_distance = distance_to_top_edge.min(distance_to_bottom_edge);

                return match minimal_distance {
                    m if m == distance_to_top_edge => Some(CollisionType::Top),
                    m if m == distance_to_bottom_edge => Some(CollisionType::Bottom),
                    _ => None,
                }
            }
        }

        return None;
    }
}

// distance_between_line_and_point: calculate distance between line and point.
//  Based on https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line#Line_defined_by_two_points
fn distance_between_line_and_point(p1: Point, p2: Point, p0: Point) -> f32 {
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
}
