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
    fn collision(&self, obj: Circle, movement: Velocity) -> Velocity {
        let constricted_angle = match movement.angle % (2.0 * consts::PI) {
            angle if angle > consts::PI => angle - (2.0 * consts::PI),
            angle if angle < -consts::PI => angle + (2.0 * consts::PI),
            angle => angle,
        };

        if obj.origin.x + obj.radius > self.width {
            if constricted_angle > -consts::FRAC_PI_2 && constricted_angle < consts::FRAC_PI_2 {
                return Velocity{
                    angle: consts::PI - constricted_angle,
                    speed: movement.speed,
                }
            }
        } else if obj.origin.x - obj.radius < 0.0 {
            if constricted_angle < -consts::FRAC_PI_2 || constricted_angle > consts::FRAC_PI_2 {
                return Velocity{
                    angle: consts::PI - constricted_angle,
                    speed: movement.speed,
                }
            }
        } else if obj.origin.y + obj.radius > self.height {
            if constricted_angle > 0.0 {
                return Velocity{
                    angle: -1.0 * constricted_angle,
                    speed: movement.speed,
                }
            }
        } else if obj.origin.y - obj.radius < 0.0 {
            if constricted_angle < 0.0 {
                return Velocity{
                    angle: -1.0 * constricted_angle,
                    speed: movement.speed,
                }
            }
        }

        return movement
    }
}
