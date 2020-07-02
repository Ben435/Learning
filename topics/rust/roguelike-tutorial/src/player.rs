use rltk::{Rltk,VirtualKeyCode,Point};
use specs::prelude::*;
use std::cmp::{min,max};
use crate::{State,RunState};
use crate::components::*;
use crate::map::Map;

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut player_position = ecs.write_resource::<Point>();
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let entities = ecs.entities();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();
    let map = ecs.fetch::<Map>();

    for (entity, _player, pos, viewshed) in (&entities, &mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        for potential_target in map.tile_content[destination_idx].iter() {
            let target = combat_stats.get(*potential_target);
            match target {
                None => {}  // Nothing to hit
                Some(_t) => {
                    // Attack, and don't move.
                    wants_to_melee
                        .insert(entity, WantsToMelee{ target: *potential_target })
                        .expect("Add target failed");
                    return;
                }
            }
        }
        if !map.blocked_tiles[destination_idx] {
            pos.x = min(map.width-1, max(0, pos.x + delta_x));
            pos.y = min(map.height-1, max(0, pos.y + delta_y));

            player_position.x = pos.x;
            player_position.y = pos.y;

            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => { return RunState::Paused },
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::Grave => gs.debug_mode = !gs.debug_mode,
            VirtualKeyCode::Space => {},    // Pass turn
            VirtualKeyCode::Q => ctx.quit(),
            _ => { return RunState::Paused },
        }
    }

    RunState::Running
}
