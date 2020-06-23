use wasm_bindgen::prelude::*;
use crate::physics::*;
use crate::objects::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Ball {
    pub body: Rectangle,
    pub velocity: Velocity,
}

impl Ball {
    pub fn update_position(&mut self, step_time: u32) {
        self.body.origin = self.body.origin.transform(self.velocity, step_time);
    }

    pub fn handle_play_space_collision(&mut self, play_space: PlaySpace) {
        if self.body.origin.x < 0.0 {
            self.body.origin.x = 0.0;
            self.velocity.x_speed = self.velocity.x_speed.abs()
        } else if (self.body.origin.x + self.body.width) > play_space.width {
            self.body.origin.x = play_space.width - self.body.width;
            self.velocity.x_speed = self.velocity.x_speed.abs() * -1.0;
        }

        if self.body.origin.y < 0.0 {
            self.body.origin.y = 0.0;
            self.velocity.y_speed = self.velocity.y_speed.abs()
        } else if (self.body.origin.y + self.body.height) > play_space.height {
            self.body.origin.y = play_space.height - self.body.height;
            self.velocity.y_speed = self.velocity.y_speed.abs() * -1.0;
        }
    }

    pub fn handle_rect_collision(&mut self, rect: Rectangle) {
        let top_left_corner = self.body.origin;
        let top_left_in = point_within_rect(top_left_corner, rect);

        let top_right_corner = Point::new(self.body.origin.x + self.body.width, self.body.origin.y);
        let top_right_in = point_within_rect(top_right_corner, rect);

        let bottom_right_corner = Point::new(self.body.origin.x + self.body.width, self.body.origin.y + self.body.height);
        let bottom_right_in = point_within_rect(bottom_right_corner, rect);

        let bottom_left_corner = Point::new(self.body.origin.x, self.body.origin.y + self.body.height);
        let bottom_left_in = point_within_rect(bottom_left_corner, rect);


        // TODO: Handle corners better (currently just defaults to horiz)        
        if top_right_in && bottom_right_in {
            // Right edge fully contained
            self.velocity.x_speed = self.velocity.x_speed.abs() * -1.0;
            self.body.origin.x = rect.origin.x - self.body.width;
        } else if top_left_in && bottom_left_in {
            // Left edge fully contained
            self.velocity.x_speed = self.velocity.x_speed.abs();
            self.body.origin.x = rect.origin.x + rect.width;
        } else if bottom_left_in && bottom_right_in {
            // Bottom edge fully contained
            self.velocity.y_speed = self.velocity.y_speed.abs() * -1.0;
            self.body.origin.y = rect.origin.y - self.body.height;
        } else if top_left_in && top_right_in {
            // Top edge fully contained
            self.velocity.y_speed = self.velocity.y_speed.abs();
            self.body.origin.y = rect.origin.y + rect.height;
        } else if top_left_in || bottom_left_in {
            // Only top left corner in
            // Treat like left edge contained
            self.velocity.x_speed = self.velocity.x_speed.abs();
            self.body.origin.x = rect.origin.x + rect.width;
        } else if top_right_in || bottom_right_in {
            // Only bottom left corner in
            // Treat like right edge contained
            self.velocity.x_speed = self.velocity.x_speed.abs() * -1.0;
            self.body.origin.x = rect.origin.x - self.body.width;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::EPSILON;
    use crate::utils::InRange;

    // Floats are a pain, this allows for a few compounded precision errors, but not drastic issues.
    const CMP_EPSILON: f32 = EPSILON * 5.0;
    const DEFAULT_BODY: Rectangle = Rectangle::new(0.0, 0.0, 5.0, 5.0);

    fn assert_float32_eq(subject: f32, expected: f32) {
        let lower = expected - CMP_EPSILON;
        let upper = expected + CMP_EPSILON;
        assert!(subject.in_range(lower, upper), "expected {} to be in range lower={}, upper={}", subject, lower, upper)
    }

    #[test]
    fn update_ball_position_horizontal() {
        let mut ball = Ball{
            body: DEFAULT_BODY,
            velocity: Velocity{ x_speed: 2.0, y_speed: 0.0 }    // East
        };

        ball.update_position(1000);

        assert_float32_eq(ball.body.origin.get_x(), 2.0);
        assert_float32_eq(ball.body.origin.get_y(), 0.0);
    }

    #[test]
    fn update_ball_position_vertical() {        
        let mut ball = Ball{
            body: DEFAULT_BODY,
            velocity: Velocity{ y_speed: -2.0, x_speed: 0.0 }  // South
        };

        ball.update_position(1000);

        assert_float32_eq(ball.body.origin.get_x(), 0.0);
        assert_float32_eq(ball.body.origin.get_y(), 2.0);
    }

    #[test]
    fn update_ball_position_45deg() {        
        let mut ball = Ball{
            body: DEFAULT_BODY,
            velocity: Velocity{ x_speed: 1.0, y_speed: 1.0 }  // South-East
        };

        ball.update_position(1000);

        assert_float32_eq(ball.body.origin.get_x(), 1.0);
        assert_float32_eq(ball.body.origin.get_y(), 1.0);
    }

    #[test]
    fn update_ball_position_relative_to_time_passed() {        
        let mut ball = Ball{
            body: DEFAULT_BODY,
            velocity: Velocity{ x_speed: 2.0, y_speed: 0.0 }  // East
        };

        ball.update_position(500);

        assert_float32_eq(ball.body.origin.get_x(), 1.0);
        assert_float32_eq(ball.body.origin.get_y(), 0.0);
    }
}
