use cgmath::Point2;
use super::Geometry2D;

#[derive(Clone,Debug)]
pub struct Circle {
    pub origin: Point2<f64>,

    pub radius: f64,
}

impl Circle {
    pub fn new(origin: Point2<f64>, radius: f64) -> Circle {
        Circle {
            origin,
            radius
        }
    }
}

impl Geometry2D for Circle {
    fn set_position(&mut self, new_position: Point2<f64>) {
        self.origin = new_position;
    }

    fn get_position(&self) -> Point2<f64> {
        self.origin
    }
}
