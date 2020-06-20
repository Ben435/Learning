use crate::physics::*;

pub trait CollideWith<T> {
    // Returns new position and velocity.
    // Position will only be updated to correct for polygon overlaps (common at low fps or after lag spikes)
    fn collision(&self, other: T, movement: Velocity) -> (Point, Velocity);
}

pub fn distance_between_points(p1: Point, p2: Point) -> f32 {
    // Pythag formula
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
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
}
