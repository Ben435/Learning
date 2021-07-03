use cgmath::Point2;use super::Geometry2D;

/// Axis aligned rectangle
/// TODO: Handle rotation
pub struct Rectangle {
    // Top left
    pub origin: Point2<f64>,
    pub width: f64,
    pub height: f64,
}

impl Geometry2D for Rectangle {
    fn set_position(&mut self, new_position: Point2<f64>) {
        self.origin = new_position;
    }

    fn get_position(&self) -> Point2<f64> {
        self.origin
    }
}

