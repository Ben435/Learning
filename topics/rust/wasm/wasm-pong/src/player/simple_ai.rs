use crate::game::GameState;

pub fn update_simple_ai(game_state: &mut GameState, step_time: u32) {
    let ai_base_speed = 100.0;
    let relative_speed = (step_time as f32 / 1000.0) * ai_base_speed;

    // Move towards ball y.
    let target_y = game_state.ball.body.origin.y - (game_state.ai_player.paddle.body.height / 2.0);
    let current_y = game_state.ai_player.paddle.body.origin.y;

    let new_y;
    if target_y > current_y {
        new_y = current_y + relative_speed.min(target_y - current_y);
    } else {
        new_y = current_y - relative_speed.min(current_y - target_y);
    }

    game_state.ai_player.paddle.body.origin.y = new_y;
}
