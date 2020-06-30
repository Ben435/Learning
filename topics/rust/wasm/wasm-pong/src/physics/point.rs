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
    pub fn new(x: f32, y: f32) -> Point {
        Point{x, y}
    }
    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn transform(&self, vel: Velocity, step_time: u32) -> Point {
        // Time units = 1 unit per 1000ms
        let time_units_passed = step_time as f32 / 1000.0;
        let relative_x_speed = vel.x_speed * time_units_passed;
        let relative_y_speed = vel.y_speed * time_units_passed;
        
        Point{
            x: self.x + relative_x_speed, 
            y: self.y + relative_y_speed,
        }
    }
}
