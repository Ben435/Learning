use wasm_bindgen::prelude::*;
use crate::physics::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub origin: Point,  // Top left.
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Rectangle {
        Rectangle{
            origin: Point{x, y},
            width,
            height,
        }
    }
}
