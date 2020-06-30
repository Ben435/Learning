use crate::game::GameObjects;
use crate::game::animation::Animation;
use crate::physics::Rectangle;

pub struct StartGameAnimation {
    current_time_millis: u32,
    duration_millis: u32,
}

impl StartGameAnimation {
    pub fn new() -> StartGameAnimation {
        StartGameAnimation {
            current_time_millis: 0,
            duration_millis: 2000,
        }
    }
}

impl Animation for StartGameAnimation {
    fn tick(&mut self, step_time: u32, _game_objects: &mut GameObjects) {
        self.current_time_millis += step_time;
    }

    fn is_done(&self) -> bool {
        return self.current_time_millis > self.duration_millis;
    }

    fn block_game(&self) -> bool {
        true
    }
}
