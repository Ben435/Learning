use rltk::{Rltk,RGB};
use crate::components::*;
use crate::constants::*;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * WORLD_WIDTH as usize) + x as usize
}

/// Boundary wall + random walls strewn around the map.
pub fn new_random_map(player_start: &Position) -> Vec<TileType> {
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
    let player_start_idx = xy_idx(player_start.x, player_start.y);
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
