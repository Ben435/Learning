use rltk::{Rltk,VirtualKeyCode,Point};
use specs::prelude::*;
use std::cmp::{min,max};
use crate::{State,RunState};
use crate::components::*;
use crate::map::Map;

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let entities = ecs.entities();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

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

            viewshed.dirty = true;
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => { return RunState::AwaitingInput },
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::Grave => {
                gs.debug_mode = !gs.debug_mode;
                return RunState::AwaitingInput;
            }
            VirtualKeyCode::Space => {},    // Pass turn
            VirtualKeyCode::Q => ctx.quit(),
            _ => { return RunState::AwaitingInput },
        }
    }

    RunState::PlayerTurn
}
