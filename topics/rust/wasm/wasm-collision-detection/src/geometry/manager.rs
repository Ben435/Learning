use super::dimension_2d::{Circle,Rectangle};

pub type GeometryId = i64;

pub trait GeometryManager {
    fn new() -> Self;
    fn add_circle(&mut self, circle: Circle) -> GeometryId;
    fn add_rectangle(&mut self, rectangle: Rectangle) -> GeometryId;
}
