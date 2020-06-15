mod utils;

use wasm_bindgen::prelude::*;
use std::collections::VecDeque;
use std::f32::consts;

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
    pub angle: f32, // radians, 0 points east on a Cartesian Plane
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

trait InRange {
    fn in_range(self, begin: Self, end: Self) -> bool;
}

impl InRange for f32 {
    fn in_range(self, begin: f32, end: f32) -> bool {
        self >= begin && self < end
    }
}

fn update_from_vel(vel: Velocity, step_time: u32, point: Point) -> Point  {
    // Cardinal angles for conversation to quadrant angle.
    let pi_2 = 2.0 * consts::PI;

    let relative_angle = match vel.angle {
        angle if angle.in_range(0.0, pi_2) => Ok(angle),
        _ => Err("Angle out of range, should be 0<=angle<=2pi (radians)")
    };

    let relative_angle = relative_angle.unwrap();

    let relative_speed = vel.speed * ((1000 / step_time) as f32);

    let translate_x = relative_angle.cos() * relative_speed;
    let translate_y = relative_angle.sin() * relative_speed;

    println!("Got: {}, {}, tx: {}, ty: {}", relative_angle, relative_speed, translate_x, translate_y);
    
    Point{
        x: point.x + translate_x, 
        y: point.y + translate_y,
    }
}


impl Ball {
    fn update_position(&mut self, step_time: u32) {
        self.position = update_from_vel(self.velocity, step_time, self.position);
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
            velocity: Velocity{ speed: 1.0, angle: consts::FRAC_PI_4 },
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::EPSILON;

    // Floats are a pain, this allows for a few stacked precision errors, but not drastic issues.
    const CMP_EPSILON: f32 = EPSILON * 5.0;

    fn assert_float32_eq(subject: f32, expected: f32) {
        let lower = expected - CMP_EPSILON;
        let upper = expected + CMP_EPSILON;
        assert!(subject.in_range(lower, upper), "expected {} to be in range lower={}, upper={}", subject, lower, upper)
    }

    #[test]
    fn update_ball_position_horizontal() {
        let mut ball = Ball{
            position: Point{x: 0.0, y: 0.0},
            velocity: Velocity{ angle: 0.0, speed: 2.0 }    // East
        };

        ball.update_position(1000);

        assert_float32_eq(ball.position.get_x(), 2.0);
        assert_float32_eq(ball.position.get_y(), 0.0);
    }

    #[test]
    fn update_ball_position_vertical() {        
        let mut ball = Ball{ 
            position: Point{x: 0.0, y: 0.0},
            velocity: Velocity{ angle: consts::FRAC_PI_2, speed: 2.0 }  // South
        };

        ball.update_position(1000);

        assert_float32_eq(ball.position.get_x(), 0.0);
        assert_float32_eq(ball.position.get_y(), 2.0);
    }

    #[test]
    fn update_ball_position_45deg() {        
        let mut ball = Ball{ 
            position: Point{x: 0.0, y: 0.0},
            velocity: Velocity{ angle: consts::FRAC_PI_4, speed: consts::SQRT_2 }  // South-East
        };

        ball.update_position(1000);

        assert_float32_eq(ball.position.get_x(), 1.0);
        assert_float32_eq(ball.position.get_y(), 1.0);
    }

    #[test]
    fn update_ball_position_outside_first_quadrant() {        
        let mut ball = Ball{
            position: Point{x: 2.0, y: 3.0},
            velocity: Velocity{ angle: consts::PI + consts::FRAC_PI_6, speed: 2.0 }  // North-West-Ish
        };

        ball.update_position(1000);

        assert_float32_eq(ball.position.get_x(), 2.0 + -1.0 * (3.0 as f32).sqrt());
        assert_float32_eq(ball.position.get_y(), 3.0 + -1.0);
    }
}
