use rltk::{Rltk,RGB};
use std::cmp::{min,max};
use crate::components::*;
use crate::constants::*;
use crate::rect::*;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * WORLD_WIDTH as usize) + x as usize
}

/// Boundary wall + random walls strewn around the map.
pub fn new_random_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; WORLD_WIDTH as usize * WORLD_HEIGHT as usize];

    // Map boundaries
    for x in 0..WORLD_WIDTH {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, WORLD_HEIGHT - 1)] = TileType::Wall;
    };
    for y in 0..WORLD_HEIGHT {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(WORLD_WIDTH - 1, y)] = TileType::Wall;
    };

    // Random walls
    let mut rng = rltk::RandomNumberGenerator::new();
    let player_start_idx = xy_idx(WORLD_WIDTH/2, WORLD_HEIGHT/2);
    for _ in 0..400 {
        let x = rng.roll_dice(1, WORLD_WIDTH - 1);
        let y = rng.roll_dice(1, WORLD_HEIGHT - 1);

        let idx = xy_idx(x, y);
        if idx != player_start_idx {
            map[idx] = TileType::Wall;
        }
    };

    map
}

pub fn new_map_rooms_and_corridors() -> (Vec<Rect>, Vec<TileType>) {
    let mut map = vec![TileType::Wall; WORLD_WIDTH as usize * WORLD_HEIGHT as usize];

    let mut rooms : Vec<Rect> = Vec::new();
    const MAX_ROOMS : i32 = 30;
    const MIN_SIZE : i32 = 6;
    const MAX_SIZE : i32 = 10;

    let mut rng = rltk::RandomNumberGenerator::new();
    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, WORLD_WIDTH - w - 1) - 1;
        let y = rng.roll_dice(1, WORLD_HEIGHT - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);

        let ok = rooms
            .iter() // Ok if no other room intersects with this one.
            .all(|other_room| !other_room.intersect(&new_room));

        if ok {
            apply_room_to_map(&new_room, &mut map);

            if !rooms.is_empty() {
                // Connect to existing rooms
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                if rng.range(0, 2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                }
            }

            rooms.push(new_room);
        }
    }

    (rooms, map)
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1 ..= room.y2 {
        for x in room.x1 + 1 ..= room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1:i32, x2:i32, y:i32) {
    for x in min(x1,x2) ..= max(x1,x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < (WORLD_WIDTH*WORLD_HEIGHT) as usize {
            map[idx as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1:i32, y2:i32, x:i32) {
    for y in min(y1,y2) ..= max(y1,y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < (WORLD_WIDTH*WORLD_HEIGHT) as usize {
            map[idx as usize] = TileType::Floor;
        }
    }
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut x = 0;
    let mut y = 0;

    for tile in map.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
            }
        }

        x += 1;
        if x > WORLD_WIDTH - 1 {
            x = 0;
            y += 1;
        }
    }
}
