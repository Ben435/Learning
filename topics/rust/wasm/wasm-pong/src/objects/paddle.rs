use wasm_bindgen::prelude::*;
use crate::physics::Point;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Paddle {
    pub width: u32,
    pub height: u32,
    pub position: Point
}
