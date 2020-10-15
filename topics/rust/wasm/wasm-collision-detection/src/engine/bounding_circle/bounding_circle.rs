use cgmath::Point2;
use crate::geometry::dimension_2d::{Circle,Rectangle,Geometry2D};

pub struct BoundingCircle<G> where G: Geometry2D {
    pub wrapped_geometry: G,
    pub bounding_circle: Circle,
}

impl BoundingCircle<Circle> {
    pub fn wrap(geometry: Circle) -> BoundingCircle<Circle> {
        BoundingCircle {
            bounding_circle: geometry.clone(),
            wrapped_geometry: geometry,
        }
    }
}

impl BoundingCircle<Rectangle> {
    pub fn wrap(geometry: Rectangle) -> BoundingCircle<Rectangle> {
        // TODO: This only handle axis-aligned rectangles atm. Will need to extend for rotation.
        let origin_x = geometry.origin.x + geometry.width / 2.0;
        let origin_y = geometry.origin.y - geometry.height / 2.0;
        BoundingCircle {
            bounding_circle: Circle::new(
                Point2::new(origin_x, origin_y), 
                geometry.width.max(geometry.height),
            ),
            wrapped_geometry: geometry,
        }
    }
}
