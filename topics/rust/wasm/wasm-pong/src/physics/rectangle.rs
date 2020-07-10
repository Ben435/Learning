use wasm_bindgen::prelude::*;
use crate::physics::*;

// [x, y, width, height]
pub type RectTuple = (f32, f32, f32, f32);

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

    // to_arr: used to pass back to renderer.
    pub fn to_arr(&self) -> RectTuple {
        (self.origin.x, self.origin.y, self.width, self.height)
    }
}
