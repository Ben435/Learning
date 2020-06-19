use wasm_bindgen::prelude::*;
use std::f32::consts;
use crate::physics::*;
use crate::objects::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Ball {
    pub position: Point,
    pub velocity: Velocity,
}

impl Ball {
    pub fn update_position(&mut self, step_time: u32) {
        let new_point = self.position.transform(self.velocity, step_time);

        self.position = new_point;
    }

    pub fn check_collisions(&mut self, play_space: PlaySpace, player_paddle: Paddle, ai_paddle: Paddle) {
        // Restrict angle to between PI and -PI
        let constricted_angle = match self.velocity.angle % (2.0 * consts::PI) {
            angle if angle > consts::PI => angle - (2.0 * consts::PI),
            angle if angle < -consts::PI => angle + (2.0 * consts::PI),
            angle => angle,
        };

        let collision = play_space
            .collision(self.position)
            .or(player_paddle.body.collision(self.position))
            .or(ai_paddle.body.collision(self.position));

        match collision {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::EPSILON;
    use crate::utils::InRange;

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

    #[test]
    fn update_ball_position_relative_to_time_passed() {        
        let mut ball = Ball{ 
            position: Point{x: 0.0, y: 0.0},
            velocity: Velocity{ angle: 0.0, speed: 2.0 }  // East
        };

        ball.update_position(500);

        assert_float32_eq(ball.position.get_x(), 1.0);
        assert_float32_eq(ball.position.get_y(), 0.0);
    }
}
