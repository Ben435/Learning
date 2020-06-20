use wasm_bindgen::prelude::*;
use crate::physics::*;
use std::f32::consts;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct PlaySpace {
    pub width: f32,
    pub height: f32,
}

impl CollideWith<Circle> for PlaySpace {
    fn collision(&self, obj: Circle, movement: Velocity) -> (Point, Velocity) {
        let mut new_origin = obj.origin;
        let mut new_vel = movement;

        if new_origin.x + obj.radius > self.width {
            new_vel = Velocity{
                angle: consts::PI - new_vel.angle,
                speed: movement.speed,
            };
            new_origin = Point{
                x: self.width - obj.radius,
                y: new_origin.y,
            };
        } else if new_origin.x - obj.radius < 0.0 {
            new_vel = Velocity{
                angle: consts::PI - new_vel.angle,
                speed: movement.speed,
            };
            new_origin = Point{
                x: obj.radius,
                y: new_origin.y,
            };
        }
        
        if new_origin.y + obj.radius > self.height {
            new_vel = Velocity{
                angle: -1.0 * new_vel.angle,
                speed: movement.speed,
            };
            new_origin = Point{
                x: new_origin.x,
                y: self.height - obj.radius,
            };
        } else if obj.origin.y - obj.radius < 0.0 {
            new_vel = Velocity{
                angle: -1.0 * new_vel.angle,
                speed: movement.speed,
            };
            new_origin = Point{
                x: new_origin.x,
                y: obj.radius,
            };
        }
        
        (new_origin, new_vel)
    }
}
