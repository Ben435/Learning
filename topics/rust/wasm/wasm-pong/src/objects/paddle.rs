use wasm_bindgen::prelude::*;
use crate::physics::Rectangle;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Paddle {
    pub body: Rectangle,
}
