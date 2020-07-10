use wasm_bindgen::prelude::*;
use crate::physics::Rectangle;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Paddle {
    pub body: Rectangle,
}

impl Paddle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Paddle {
        Paddle{
            body: Rectangle::new(x, y, width, height),
        }
    }
}
