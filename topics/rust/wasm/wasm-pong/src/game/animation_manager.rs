
use wasm_bindgen::prelude::*;
use std::collections::VecDeque;
use web_sys::console;
use crate::game::GameObjects;


#[wasm_bindgen]
pub struct AnimationManager {
    currently_running_animations: VecDeque<Box<dyn Animation>>
}

pub trait Animation {
    // tick: return true if done, false, if more frames.
    fn tick(&mut self, step_time: u32, game_objects: &mut GameObjects);
    fn is_done(&self) -> bool;
    fn block_game(&self) -> bool;
}

impl AnimationManager {
    pub fn new() -> AnimationManager {
        AnimationManager{
            currently_running_animations: VecDeque::new(),
        }
    }
}

impl AnimationManager {
    pub fn tick(&mut self, step_time: u32, game_objects: &mut GameObjects) {
        let mut to_process = self.currently_running_animations.len();

        while to_process > 0  {
            let mut anim = self.currently_running_animations.pop_front().unwrap();

            anim.tick(step_time, game_objects);

            if !anim.is_done() {
                self.currently_running_animations.push_back(anim);
            }

            to_process -= 1;
        }
    }

    pub fn block_game(&self) -> bool {
        self.currently_running_animations
            .iter()
            .fold(false, |prev, anim| prev || anim.block_game())
    }

    pub fn trigger_animation(&mut self, animation: Box<dyn Animation>) {
        self.currently_running_animations.push_back(animation);
    }
}

pub struct ResetAnimation {
    pub current_time_millis: u32,
    pub duration_millis: u32,
    pub ran: bool,
}

impl ResetAnimation {
    pub fn new() -> ResetAnimation {
        ResetAnimation {
            current_time_millis: 0,
            duration_millis: 3000,
            ran: false,
        }
    }
}

impl Animation for ResetAnimation {
    fn tick(&mut self, step_time: u32, game_objects: &mut GameObjects) {
        self.current_time_millis += step_time;

        if !self.ran && self.current_time_millis > self.duration_millis {
            self.ran = true;

            game_objects.ball.body.origin.x = 250.0;
            game_objects.ball.body.origin.y = 250.0;

            game_objects.human_player.paddle.body.origin.y = 250.0;
            game_objects.ai_player.paddle.body.origin.y = 250.0;
        }
    }

    fn is_done(&self) -> bool {
        return self.current_time_millis > self.duration_millis;
    }

    fn block_game(&self) -> bool {
        true
    }
}
