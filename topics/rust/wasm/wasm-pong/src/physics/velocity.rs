use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Velocity {
    // units / sec
    pub x_speed: f32,
    pub y_speed: f32,
}
