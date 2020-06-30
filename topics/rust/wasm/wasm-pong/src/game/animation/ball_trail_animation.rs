use crate::physics::Point;
use crate::objects::Ball;
use crate::game::GameObjects;
use crate::game::animation::Animation;
use crate::physics::Rectangle;

const DEFAULT_TRAIL_LENGTH: usize = 5;
const DEFAULT_TRAIL_INCREMENT_MS: u32 = 50;

pub struct BallTrailAnimation {
    init_point: Point,
    trail_width: f32,
    trail_height: f32,
    current_time_ms: u32,
    prev_trail_recorded_ms: u32,
    trail_increment_ms: u32,
    pub trail_index: usize,
    pub trail: Box<[Point; DEFAULT_TRAIL_LENGTH]>,
}

impl BallTrailAnimation {
    pub fn new(ball: &Ball) -> BallTrailAnimation {
        BallTrailAnimation {
            init_point: ball.body.origin,
            trail_width: ball.body.width - (0.1 * ball.body.width),
            trail_height: ball.body.height - (0.1 * ball.body.height),
            current_time_ms: 0,
            trail_increment_ms: DEFAULT_TRAIL_INCREMENT_MS,
            prev_trail_recorded_ms: 0,
            trail_index: 0,
            trail: Box::new([ball.body.origin; DEFAULT_TRAIL_LENGTH]),
        }
    }

    pub fn reset_trail(&mut self) {
        self.trail = Box::new([self.init_point; DEFAULT_TRAIL_LENGTH]);
    }
}

impl Animation for BallTrailAnimation {
    fn tick(&mut self, step_time: u32, game_objects: &mut GameObjects) {
        self.current_time_ms += step_time;

        if self.current_time_ms - self.prev_trail_recorded_ms > self.trail_increment_ms {
            self.trail[self.trail_index] = game_objects.ball.body.origin.clone();
            self.trail_index = (self.trail_index + 1) % self.trail.len();

            self.prev_trail_recorded_ms = self.current_time_ms;
        }
    }

    fn get_elements(&self) -> Vec<Rectangle> {
        let point_to_rect = |p: &Point| Rectangle{
            origin: *p, 
            width: self.trail_width, 
            height: self.trail_height,
        };

        self.trail
            .iter()
            .map(point_to_rect)
            .collect::<Vec<Rectangle>>()
    }

    fn is_done(&self) -> bool {
        false
    }

    fn block_game(&self) -> bool {
        false
    }
}
