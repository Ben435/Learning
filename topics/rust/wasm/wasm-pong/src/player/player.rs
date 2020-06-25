use wasm_bindgen::prelude::*;
use crate::objects::Paddle;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub paddle: Paddle,
    pub score: u32,
}

impl Player {
    pub fn new(paddle: Paddle) -> Player {
        Player{
            paddle,
            score: 0,
        }
    }
}
