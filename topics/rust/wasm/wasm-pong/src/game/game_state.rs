use wasm_bindgen::prelude::*;
use js_sys::Array;
use crate::objects::*;
use crate::player::*;
use crate::physics::*;
use crate::game::*;

#[wasm_bindgen]
pub struct GameState {
    pub play_space: PlaySpace,
    pub ball: Ball,
    pub human_player: Player,
    pub ai_player: Player,
    pause_handler: PauseHandler,
}

#[wasm_bindgen]
impl GameState {
    pub fn new(
        width: f32, height: f32, 
        paddle_width: f32, paddle_height: f32,
        ball_size: f32, ball_speed: f32, ball_starting_angle: f32,
    ) -> GameState {
        let player_paddle = Paddle{
            body: Rectangle{
                origin: Point{ x: width / 10.0, y: height / 2.0 },
                width: paddle_width,
                height: paddle_height,
            }
        };

        let human_player = Player{
            paddle: player_paddle,
        };
    
        let ai_paddle = Paddle{
            body: Rectangle{
                origin: Point{ x: width - (width / 10.0), y: height / 2.0 },
                width: paddle_width,
                height: paddle_height,
            }
        };

        let ai_player = Player{
            paddle: ai_paddle,
        };

        // Convert initial angle into x and y speeds.
        let x_speed = ball_starting_angle.sin() * ball_speed;
        let y_speed = ball_starting_angle.cos() * ball_speed;
        let ball = Ball{
            body: Rectangle::new(
                width / 3.0, height / 3.0, 
                ball_size, ball_size,
            ),
            velocity: Velocity{ x_speed, y_speed },
        };
    
        let play_space = PlaySpace{
            width,
            height
        };

        let pause_handler = PauseHandler::new(false);
    
        return GameState{
            human_player,
            ai_player,
            ball,
            play_space,
            pause_handler,
        }
    }

    pub fn get_rects(&self) -> Array {
        let rects = [
            self.ball.body,
            self.human_player.paddle.body,
            self.ai_player.paddle.body,
        ];
        
        return rects
            .iter()
            .enumerate()
            .fold(
                Array::new_with_length(rects.len() as u32), 
                |arr, (index, rect)| {
                    let rect_spec = Array::new_with_length(4);
                    rect_spec.set(0, JsValue::from(rect.origin.x));
                    rect_spec.set(1, JsValue::from(rect.origin.y));
                    rect_spec.set(2, JsValue::from(rect.width));
                    rect_spec.set(3, JsValue::from(rect.height));
                    
                    arr.set(index as u32, JsValue::from(rect_spec));

                    return arr;
                });
    }

    pub fn tick(&mut self, step_time: u32, current_keys: &[u32]) {
        // No-op if no time has passed.
        // Simplifies logic, as a bunch of stuff divides by the step_time, 
        // and this guarantees its never a div by zero.
        if step_time == 0 {
            return;
        }

        self.process_events(step_time, current_keys);

        if self.pause_handler.get_paused() {
            return;
        }

        update_simple_ai(self, step_time);
        self.ball.update_position(step_time);

        self.ball.handle_play_space_collision(self.play_space);
        self.ball.handle_rect_collision(self.human_player.paddle.body);
        self.ball.handle_rect_collision(self.ai_player.paddle.body);
    }

    fn process_events(&mut self, step_time: u32, current_keys: &[u32]) {
        const ARROW_KEYS: [u32; 2] = [keys::ARROW_UP, keys::ARROW_DOWN];
        for key_code in current_keys.iter() {
            match *key_code {
                k if ARROW_KEYS.contains(key_code) => self.process_movement_key(k, step_time),
                k if k == keys::P => self.process_paused_key(),
                _ => {},
            }
        }
    }

    fn process_paused_key(&mut self) {
        self.pause_handler.key_toggle_pause();
    }

    fn process_movement_key(&mut self, key_code: u32, step_time: u32) {
        if self.pause_handler.get_paused() {
            // No moving while paused!
            return;
        }

        let upper_bound = self.play_space.height - self.human_player.paddle.body.height;
        let lower_bound = 0.0;
        let move_speed = 100.0;

        let base_speed = match key_code {
            k if k == keys::ARROW_UP => move_speed,
            k if k == keys::ARROW_DOWN => -move_speed,
            _ => 0.0,
        };

        let relative_speed = (step_time as f32 / 1000.0) * base_speed;
        let new_y = self.human_player.paddle.body.origin.y + relative_speed;

        self.human_player.paddle.body.origin.y = match new_y {
            new_y if new_y > upper_bound => upper_bound,
            new_y if new_y < lower_bound => lower_bound,
            new_y => new_y,
        };
    }
}
