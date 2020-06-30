use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct PlaySpace {
    pub width: f32,
    pub height: f32,
}
