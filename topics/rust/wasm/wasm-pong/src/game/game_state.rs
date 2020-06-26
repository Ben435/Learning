use wasm_bindgen::prelude::*;
use js_sys::Array;
use crate::objects::*;
use crate::player::*;
use crate::physics::*;
use crate::game::*;
use crate::game::animation::*;

#[wasm_bindgen]
pub struct GameState {
    pub play_space: PlaySpace,
    game_objects: GameObjects,
    pause_manager: PauseManager,
    animation_manager: AnimationManager,
    random_manager: RandomManager,
}

pub struct GameObjects {
    pub ball: Ball,
    pub human_player: Player,
    pub ai_player: Player,
}

impl GameObjects {
    pub fn new(ball: Ball, human_player: Player, ai_player: Player) -> GameObjects {
        GameObjects {
            ball,
            human_player,
            ai_player,
        }
    }
}

impl GameState {
    fn can_progress_game(&self) -> bool {
        self.pause_manager.get_paused() || self.animation_manager.block_game()
    }

    fn begin_reset_animation(&mut self) {
        self.animation_manager.trigger_animation(Box::new(ResetAnimation::new(&mut self.random_manager)));
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
        self.pause_manager.key_toggle_pause();
    }

    fn process_movement_key(&mut self, key_code: u32, step_time: u32) {
        if self.can_progress_game() {
            return;
        }

        let upper_bound = self.play_space.height - self.game_objects.human_player.paddle.body.height;
        let lower_bound = 0.0;
        let move_speed = 100.0;

        let base_speed = match key_code {
            k if k == keys::ARROW_UP => move_speed,
            k if k == keys::ARROW_DOWN => -move_speed,
            _ => 0.0,
        };

        let relative_speed = (step_time as f32 / 1000.0) * base_speed;
        let new_y = self.game_objects.human_player.paddle.body.origin.y + relative_speed;

        self.game_objects.human_player.paddle.body.origin.y = match new_y {
            new_y if new_y > upper_bound => upper_bound,
            new_y if new_y < lower_bound => lower_bound,
            new_y => new_y,
        };
    }
}

#[wasm_bindgen]
impl GameState {
    pub fn new(
        width: f32, height: f32, 
        paddle_width: f32, paddle_height: f32,
        ball_size: f32, ball_speed: f32, ball_starting_angle: f32,
        seed: u64,
    ) -> GameState {
        let player_paddle = Paddle{
            body: Rectangle{
                origin: Point{ x: width / 10.0, y: height / 2.0 },
                width: paddle_width,
                height: paddle_height,
            }
        };

        let human_player = Player::new(player_paddle);
    
        let ai_paddle = Paddle{
            body: Rectangle{
                origin: Point{ x: width - (width / 10.0), y: height / 2.0 },
                width: paddle_width,
                height: paddle_height,
            }
        };

        let ai_player = Player::new(ai_paddle);

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

        let game_objects = GameObjects::new(
            ball, human_player, ai_player
        );
    
        let play_space = PlaySpace{
            width,
            height
        };

        let pause_manager = PauseManager::new(false);
        let random_manager = RandomManager::new(seed);
        let animation_manager = AnimationManager::new();
    
        return GameState{
            game_objects,
            play_space,
            pause_manager,
            animation_manager,
            random_manager,
        }
    }

    pub fn get_rects(&self) -> Array {
        let rects = [
            self.game_objects.ball.body,
            self.game_objects.human_player.paddle.body,
            self.game_objects.ai_player.paddle.body,
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

    pub fn get_score(&self) -> Array {
        let player_score = self.game_objects.human_player.score;
        let ai_score = self.game_objects.ai_player.score;

        let ret_arr = Array::new_with_length(2);
        ret_arr.set(0, JsValue::from(player_score));
        ret_arr.set(1, JsValue::from(ai_score));

        return ret_arr;
    }

    pub fn is_paused(&self) -> bool {
        self.pause_manager.get_paused()
    }

    pub fn tick(&mut self, step_time: u32, current_keys: &[u32]) {
        // No-op if no time has passed.
        // Simplifies logic, as a bunch of stuff divides by the step_time, 
        // and this guarantees its never a div by zero.
        if step_time == 0 {
            return;
        }

        self.process_events(step_time, current_keys);

        if self.pause_manager.get_paused() {
            return;
        }

        self.animation_manager.tick(step_time, &mut self.game_objects);

        if self.can_progress_game() {
            return;
        }

        update_simple_ai(&self.play_space, &mut self.game_objects, step_time);
        self.game_objects.ball.update_position(step_time);

        let collided_with_wall = self.game_objects.ball.handle_play_space_collision(self.play_space);
        if collided_with_wall {
            if self.game_objects.ball.body.origin.x == 0.0 {
                // Hit left wall. AI gets point.
                self.game_objects.ai_player.score += 1;
                self.begin_reset_animation();
            } else if self.game_objects.ball.body.origin.x == (self.play_space.width - self.game_objects.ball.body.width) {
                // Hit right wall. Player gets point.
                self.game_objects.human_player.score += 1;
                self.begin_reset_animation();
            }
        }

        self.game_objects.ball.handle_rect_collision(self.game_objects.human_player.paddle.body);
        self.game_objects.ball.handle_rect_collision(self.game_objects.ai_player.paddle.body);
    }
}
