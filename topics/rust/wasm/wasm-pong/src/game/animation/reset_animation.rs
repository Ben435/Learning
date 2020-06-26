
use std::f32::consts::PI;
use crate::game::RandomManager;
use crate::game::GameObjects;
use crate::game::animation::Animation;

pub struct ResetAnimation {
    current_time_millis: u32,
    duration_millis: u32,
    stage: u32,
    random_num: f32,
}

impl ResetAnimation {
    pub fn new(random_manager: &mut RandomManager) -> ResetAnimation {
        ResetAnimation {
            current_time_millis: 0,
            duration_millis: 3000,
            stage: 0,
            random_num: random_manager.next_in_range(0.0, PI),
        }
    }
}

impl Animation for ResetAnimation {
    fn tick(&mut self, step_time: u32, game_objects: &mut GameObjects) {
        self.current_time_millis += step_time;

        if self.stage < 1 && self.current_time_millis > (self.duration_millis / 3) {
            self.stage = 1;

            game_objects.ball.body.origin.x = 250.0;
            game_objects.ball.body.origin.y = 250.0;
        } else if self.stage < 2 && self.current_time_millis > ((2 * self.duration_millis) / 3) {
            self.stage = 2;

            game_objects.human_player.paddle.body.origin.y = 250.0;
            game_objects.ai_player.paddle.body.origin.y = 250.0;
        } else if self.stage < 3 && self.current_time_millis > self.duration_millis {
            self.stage = 3;

            // Initialize ball with random angle.
            // Make limit angle to reasonable (eg: not vertical)
            let clipped_angle = match self.random_num {
                a if a < (PI/2.0) => a - (PI/4.0),
                a => a + (PI/4.0),
            };
            let speed_init = 200.0;
            
            game_objects.ball.velocity.x_speed = clipped_angle.cos() * speed_init;
            game_objects.ball.velocity.y_speed = clipped_angle.sin() * speed_init;      
        }
    }

    fn is_done(&self) -> bool {
        return self.stage >= 3;
    }

    fn block_game(&self) -> bool {
        true
    }
}
