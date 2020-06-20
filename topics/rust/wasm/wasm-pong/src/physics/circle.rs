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
    fn collision(&self, obj: Circle, vel: Velocity) -> (Point, Velocity) {
        let distance_between_circles = distance_between_points(self.origin, obj.origin);
        let penetration_distance = distance_between_circles - (self.radius + obj.radius);

        // If not overlapping, just return original position and velocity
        if penetration_distance < 0.0 {
            return (obj.origin, vel);
        }

        // Correct for penetration
        // Get angle between 2 circles, and adjust obj by penetration_distance
        

        // Calculate point of contact
        let collision_point_x = ((self.origin.x * obj.radius) + (self.origin.x * self.radius)) / (self.radius + obj.radius);
        let collision_point_y = ((self.origin.y * obj.radius) + (self.origin.y * self.radius)) / (self.radius + obj.radius);

        

        return (obj.origin, vel);
    }
}
