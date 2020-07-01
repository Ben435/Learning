use rltk::{Rltk,VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{min,max};
use crate::State;
use crate::components::{Position,Player};
use crate::map::{TileType,xy_idx};
use crate::constants::*;

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);

        if map[destination_idx] != TileType::Wall {
            pos.x = min(WORLD_WIDTH-1, max(0, pos.x + delta_x));
            pos.y = min(WORLD_HEIGHT-1, max(0, pos.y + delta_y));
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {},
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::Grave => gs.debug_mode = !gs.debug_mode,
            VirtualKeyCode::Q => ctx.quit(),
            _ => {},
        }
    }
}
