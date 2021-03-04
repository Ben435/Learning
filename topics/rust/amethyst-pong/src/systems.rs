use amethyst::{
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings}
};
use crate::pong::{Paddle,Side,ARENA_HEIGHT,PADDLE_HEIGHT};
use log::info;

#[derive(SystemDesc)]
pub struct PaddleSystem {}

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };

            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let scaled_amount = 1.2 * mv_amount as f32;
                    let paddle_y = transform.translation().y;
                    transform.set_translation_y(
                        (paddle_y + scaled_amount)
                            .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
                            .max(PADDLE_HEIGHT * 0.5));
                }
            }
        }
    }
}
