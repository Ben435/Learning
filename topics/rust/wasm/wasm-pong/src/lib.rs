mod utils;

use wasm_bindgen::prelude::*;
use std::collections::VecDeque;
use std::f32::consts;
use web_sys::console;

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

    pub fn transform(&self, vel: Velocity, step_time: u32) -> Point {
        let relative_speed = vel.speed * (step_time as f32 / 1000.0);

        let translate_x = vel.angle.cos() * relative_speed;
        let translate_y = vel.angle.sin() * relative_speed;
        
        Point{
            x: self.x + translate_x, 
            y: self.y + translate_y,
        }
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

impl Ball {
    fn update_position(&mut self, step_time: u32, play_space: PlaySpace) {
        let new_point = self.position.transform(self.velocity, step_time);

        let constricted_angle = match self.velocity.angle % (2.0 * consts::PI) {
            angle if angle > consts::PI => angle - (2.0 * consts::PI),
            angle if angle < -consts::PI => angle + (2.0 * consts::PI),
            angle => angle,
        };

        match play_space.is_out_of_bounds(new_point) {
            Some(CollisionType::Right) => {
                // If angled right, then flip over the Y axis
                if constricted_angle > -consts::FRAC_PI_2 && constricted_angle < consts::FRAC_PI_2 {
                    self.velocity.angle = consts::PI - constricted_angle;
                }
            },
            Some(CollisionType::Left) => {
                // If angled left, then flip over the Y axis
                if constricted_angle < -consts::FRAC_PI_2 || constricted_angle > consts::FRAC_PI_2 {
                    self.velocity.angle = consts::PI - constricted_angle;
                }
            },
            Some(CollisionType::Top) => {
                // If angled top, then flip over the X axis
                if constricted_angle < 0.0 {
                    self.velocity.angle = -1.0 * constricted_angle;
                }
            }
            Some(CollisionType::Bottom) => {
                // If angled bottom, then flip over the X axis
                if constricted_angle > 0.0 {
                    self.velocity.angle = -1.0 * constricted_angle;
                }
            }
            None => {}
        }

        self.position = new_point;
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct PlaySpace {
    pub width: f32,
    pub height: f32,
}

// CollisionType: Describes collision.
pub enum CollisionType {
    Top = 0,
    Right = 1,
    Bottom = 2,
    Left = 3,
}

impl PlaySpace {
    pub fn is_out_of_bounds(&self, point: Point) -> Option<CollisionType> {
        match point {
            p if p.get_x() < 0.0 => Some(CollisionType::Left),
            p if p.get_x() > self.width => Some(CollisionType::Right),
            p if p.get_y() < 0.0 => Some(CollisionType::Top),
            p if p.get_y() > self.height => Some(CollisionType::Bottom),
            _ => None,
        }
    }
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
}

#[wasm_bindgen]
impl GameState {
    pub fn new(
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
            position: Point{ x: width / 3.0, y: height / 2.0},
            velocity: Velocity{ speed: 50.0, angle: consts::FRAC_PI_4 },
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
        // No-op if no time has passed.
        if step_time == 0 {
            return;
        }

        self.ball.update_position(step_time, self.play_space);
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

    static DUMMY_PLAY_SPACE: PlaySpace = PlaySpace{
        width: 100.0,
        height: 100.0,
    };

    #[test]
    fn update_ball_position_horizontal() {
        let mut ball = Ball{
            position: Point{x: 0.0, y: 0.0},
            velocity: Velocity{ angle: 0.0, speed: 2.0 }    // East
        };

        ball.update_position(1000, DUMMY_PLAY_SPACE);

        assert_float32_eq(ball.position.get_x(), 2.0);
        assert_float32_eq(ball.position.get_y(), 0.0);
    }

    #[test]
    fn update_ball_position_vertical() {        
        let mut ball = Ball{ 
            position: Point{x: 0.0, y: 0.0},
            velocity: Velocity{ angle: consts::FRAC_PI_2, speed: 2.0 }  // South
        };

        ball.update_position(1000, DUMMY_PLAY_SPACE);

        assert_float32_eq(ball.position.get_x(), 0.0);
        assert_float32_eq(ball.position.get_y(), 2.0);
    }

    #[test]
    fn update_ball_position_45deg() {        
        let mut ball = Ball{ 
            position: Point{x: 0.0, y: 0.0},
            velocity: Velocity{ angle: consts::FRAC_PI_4, speed: consts::SQRT_2 }  // South-East
        };

        ball.update_position(1000, DUMMY_PLAY_SPACE);

        assert_float32_eq(ball.position.get_x(), 1.0);
        assert_float32_eq(ball.position.get_y(), 1.0);
    }

    #[test]
    fn update_ball_position_outside_first_quadrant() {        
        let mut ball = Ball{
            position: Point{x: 2.0, y: 3.0},
            velocity: Velocity{ angle: consts::PI + consts::FRAC_PI_6, speed: 2.0 }  // North-West-Ish
        };

        ball.update_position(1000, DUMMY_PLAY_SPACE);

        assert_float32_eq(ball.position.get_x(), 2.0 + -1.0 * (3.0 as f32).sqrt());
        assert_float32_eq(ball.position.get_y(), 3.0 + -1.0);
    }

    #[test]
    fn update_ball_position_relative_to_time_passed() {        
        let mut ball = Ball{ 
            position: Point{x: 0.0, y: 0.0},
            velocity: Velocity{ angle: 0.0, speed: 2.0 }  // East
        };

        ball.update_position(500, DUMMY_PLAY_SPACE);

        assert_float32_eq(ball.position.get_x(), 1.0);
        assert_float32_eq(ball.position.get_y(), 0.0);
    }
}
