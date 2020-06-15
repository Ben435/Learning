mod utils;

use wasm_bindgen::prelude::*;
use std::collections::VecDeque;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
#[wasm_bindgen]
impl Point {
    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Velocity {
    pub speed: f32, // units / sec
    pub angle: f32, // degs
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Paddle {
    pub width: u32,
    pub height: u32,
    pub position: Point
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Ball {
    pub position: Point,
    pub velocity: Velocity,
}

impl Ball {
    fn update_position(&mut self, step_time: u32) {
        self.position.x += (step_time as f32 / 1000.0) * self.velocity.speed;
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct PlaySpace {
    pub width: f32,
    pub height: f32,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameEvent {
    PlayerMoveUp = 0,
    PlayerMoveDown = 1
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct GameState {
    pub play_space: PlaySpace,
    pub player_paddle: Paddle,
    pub ai_paddle: Paddle,
    pub ball: Ball,
    pending_events: VecDeque<GameEvent>,
    fps: u32,
    time_through_current_frame: u32,
}

#[wasm_bindgen]
impl GameState {
    pub fn new(
        fps: u32, 
        width: f32, height: f32, 
        paddle_width: u32, paddle_height: u32
    ) -> GameState {
        let player_paddle = Paddle{
            position: Point{ x: width / 10.0, y: height / 2.0 },
            width: paddle_width,
            height: paddle_height,
        };
    
        let ai_paddle = Paddle{
            position: Point{ x: width - (width / 10.0), y: height / 2.0 },
            width: paddle_width,
            height: paddle_height,
        };
    
        let ball = Ball{
            position: Point{ x: width / 2.0, y: height / 2.0},
            velocity: Velocity{ speed: 1.0, angle: 0.0 },
        };
    
        let play_space = PlaySpace{
            width,
            height
        };

        let pending_events = VecDeque::with_capacity(5);
    
        return GameState{
            play_space,
            player_paddle,
            ai_paddle,
            ball,
            pending_events,
            fps,
            time_through_current_frame: 0,
        }
    }

    pub fn get_player_paddle_position(&self) -> Point {
        self.player_paddle.position
    }

    pub fn get_ai_paddle_position(&self) -> Point {
        self.ai_paddle.position
    }

    pub fn get_ball_position(&self) -> Point {
        self.ball.position
    }

    pub fn tick(&mut self, step_time: u32) {
        self.time_through_current_frame += step_time;
        let time_per_frame = 1000 / self.fps;

        while self.time_through_current_frame > time_per_frame {
            self.time_through_current_frame -= time_per_frame;

            self.ball.update_position(time_per_frame);
        }
    }
}
