
use std::collections::VecDeque;
use crate::game::GameObjects;
use crate::physics::Rectangle;

pub struct AnimationManager {
    currently_running_animations: VecDeque<Box<dyn Animation>>
}

pub trait Animation {
    // tick: return true if done, false, if more frames.
    fn tick(&mut self, step_time: u32, game_objects: &mut GameObjects);
    fn is_done(&self) -> bool;
    fn block_game(&self) -> bool;

    fn get_elements(&self) -> Vec<Rectangle> {
        Vec::new()
    }
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

    pub fn get_elements(&self) -> Vec<Rectangle> {
        self.currently_running_animations
            .iter()
            .flat_map(|anim| anim.get_elements())
            .collect()
    }
}
