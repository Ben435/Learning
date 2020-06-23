mod utils;
mod objects;
mod physics;

use wasm_bindgen::prelude::*;
use objects::*;
use physics::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static ARROW_DOWN: u32 = 38;
static ARROW_UP: u32 = 40;

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct GameState {
    pub play_space: PlaySpace,
    pub player_paddle: Paddle,
    pub ai_paddle: Paddle,
    pub ball: Ball,
}

#[wasm_bindgen]
impl GameState {
    pub fn new(
        width: f32, height: f32, 
        paddle_width: f32, paddle_height: f32,
        ball_size: f32, ball_speed: f32,
    ) -> GameState {
        let player_paddle = Paddle{
            body: Rectangle{
                origin: Point{ x: width / 10.0, y: height / 2.0 },
                width: paddle_width,
                height: paddle_height,
            }
        };
    
        let ai_paddle = Paddle{
            body: Rectangle{
                origin: Point{ x: width - (width / 10.0), y: height / 2.0 },
                width: paddle_width,
                height: paddle_height,
            }
        };
    
        let ball = Ball{
            body: Rectangle::new(
                width / 3.0, height / 3.0, 
                ball_size, ball_size,
            ),
            velocity: Velocity{ x_speed: ball_speed, y_speed: 0.0 },
        };
    
        let play_space = PlaySpace{
            width,
            height
        };
    
        return GameState{
            play_space,
            player_paddle,
            ai_paddle,
            ball,
        }
    }

    pub fn get_player_paddle_position(&self) -> Point {
        self.player_paddle.body.origin
    }

    pub fn get_ai_paddle_position(&self) -> Point {
        self.ai_paddle.body.origin
    }

    pub fn get_ball_position(&self) -> Point {
        self.ball.body.origin
    }

    pub fn tick(&mut self, step_time: u32, current_keys: &[u32]) {
        // No-op if no time has passed.
        if step_time == 0 {
            return;
        }

        self.process_events(step_time, current_keys);
        self.ball.update_position(step_time);

        self.ball.handle_play_space_collision(self.play_space);
        self.ball.handle_rect_collision(self.player_paddle.body);
        self.ball.handle_rect_collision(self.player_paddle.body);
    }

    fn process_events(&mut self, step_time: u32, current_keys: &[u32]) {
        let usable_key_codes = [ARROW_UP, ARROW_DOWN];
        let upper_bound = self.play_space.height - self.player_paddle.body.height;
        let lower_bound = 0.0;

        let move_speed = 100.0;

        for key_code in current_keys.iter().filter(|key_code| usable_key_codes.contains(key_code)) {
            let base_speed = match *key_code {
                k if k == ARROW_UP => move_speed,
                k if k == ARROW_DOWN => -move_speed,
                _ => 0.0,
            };

            let relative_speed = (step_time as f32 / 1000.0) * base_speed;
            let new_y = self.player_paddle.body.origin.y + relative_speed;

            self.player_paddle.body.origin.y = match new_y {
                new_y if new_y > upper_bound => upper_bound,
                new_y if new_y < lower_bound => lower_bound,
                new_y => new_y,
            };
        }
    }
}
