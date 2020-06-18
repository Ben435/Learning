use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Velocity {
    pub speed: f32, // units / sec
    pub angle: f32, // radians, 0 points east on a Cartesian Plane
}
