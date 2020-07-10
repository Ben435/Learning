mod utils;
mod objects;
mod physics;
mod player;
mod game;

use wasm_bindgen::prelude::*;
use js_sys::Date;
use game::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn new_game(
    width: f32, height: f32, 
    paddle_width: f32, paddle_height: f32,
    ball_size: f32, ball_speed: f32, ball_init_angle: f32,
) -> GameState {
    return GameState::new(
        width, height, 
        paddle_width, paddle_height,
        ball_size, ball_speed, ball_init_angle,
        Date::now().round().abs() as u64,
    );
}
