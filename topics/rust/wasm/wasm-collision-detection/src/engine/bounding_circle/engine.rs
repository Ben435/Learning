use super::BoundingCircle;
use crate::geometry::dimension_2d::{Geometry2D,Circle,Rectangle};
use crate::{GeometryId,GeometryManager};
use crate::engine::CollisionEvent2D;
use std::collections::HashMap;

pub struct BoundingCircleEngine {
    id_seq: GeometryId,
    items: HashMap<GeometryId, BoundingCircle<Geometry2D>>,
}

impl BoundingCircleEngine {
    pub fn check_for_collisions(&self) -> Vec<CollisionEvent2D> {
        vec!()
    }

    pub fn next_id(&mut self) -> GeometryId {
        self.id_seq += 1;
        self.id_seq
    }
}

impl GeometryManager for BoundingCircleEngine {
    pub fn new() -> BoundingCircleEngine {
        BoundingCircleEngine {
            items: HashMap::new(),
            id_seq: 0,
        }
    }

    pub fn add_circle(&mut self, circle: Circle) {
        let id = self.next_id();
        let item = BoundingCircle::<Circle>::wrap(circle);
        self.items.insert(id, item);

        id
    }

    pub fn add_rectangle(&mut self, rectangle: Rectangle) {

    }
}
