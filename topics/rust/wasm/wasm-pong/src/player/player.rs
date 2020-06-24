use wasm_bindgen::prelude::*;
use crate::objects::Paddle;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub paddle: Paddle,
}
