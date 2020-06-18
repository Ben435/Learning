use wasm_bindgen::prelude::*;
use crate::physics::Velocity;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
#[wasm_bindgen]
impl Point {
    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn transform(&self, vel: Velocity, step_time: u32) -> Point {
        let relative_speed = vel.speed * (step_time as f32 / 1000.0);

        let translate_x = vel.angle.cos() * relative_speed;
        let translate_y = vel.angle.sin() * relative_speed;
        
        Point{
            x: self.x + translate_x, 
            y: self.y + translate_y,
        }
    }
}
