use wasm_bindgen::prelude::*;
use crate::physics::{Point,CollideWith,CollisionType};

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct PlaySpace {
    pub width: f32,
    pub height: f32,
}

impl CollideWith<Point> for PlaySpace {
    fn collision(&self, point: Point) -> Option<CollisionType> {
        match point {
            p if p.get_x() < 0.0 => Some(CollisionType::Left),
            p if p.get_x() > self.width => Some(CollisionType::Right),
            p if p.get_y() < 0.0 => Some(CollisionType::Top),
            p if p.get_y() > self.height => Some(CollisionType::Bottom),
            _ => None,
        }
    }
}
