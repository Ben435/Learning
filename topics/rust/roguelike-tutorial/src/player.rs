use rltk::{Rltk,VirtualKeyCode,Point};
use specs::prelude::*;
use std::cmp::{min,max};
use crate::{State,RunState};
use crate::components::{Position,Player,Viewshed};
use crate::map::{TileType,Map};
use crate::constants::*;

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut player_position = ecs.write_resource::<Point>();
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_map_tile = map.get_tile(pos.x + delta_x, pos.y + delta_y);

        let valid_destination = destination_map_tile
            .map(|tile| tile != TileType::Wall)
            .unwrap_or(false);

        if valid_destination {
            pos.x = min(WORLD_WIDTH-1, max(0, pos.x + delta_x));
            pos.y = min(WORLD_HEIGHT-1, max(0, pos.y + delta_y));

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
            VirtualKeyCode::Q => ctx.quit(),
            _ => { return RunState::Paused },
        }
    }

    RunState::Running
}
