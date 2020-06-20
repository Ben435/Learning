use wasm_bindgen::prelude::*;
use crate::physics::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub origin: Point,
    pub radius: f32,
}

impl Circle {
    pub fn new(x: f32, y: f32, radius: f32) -> Circle {
        Circle{
            origin: Point{ x, y },
            radius,
        }
    }
}

impl CollideWith<Circle> for Circle {
    fn collision(&self, obj: Circle, movement: Velocity) -> (Point, Velocity) {
        let is_colliding = distance_between_points(self.origin, obj.origin) > self.radius + obj.radius;

        if !is_colliding {
            return (obj.origin, movement);
        }

        // Calculate point of contact, and "reflect" movement.
        

        return (obj.origin, movement);
    }
}
