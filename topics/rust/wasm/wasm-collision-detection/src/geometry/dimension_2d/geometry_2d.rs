use cgmath::Point2;

pub trait Geometry2D {
    fn get_position(&self) -> Point2<f64>;
    fn set_position(&mut self, new_position: Point2<f64>);
    // TODO: Rotation
}
