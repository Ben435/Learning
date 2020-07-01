use rltk::{Rltk,GameState,RGB,VirtualKeyCode};
use specs_derive::Component;
use specs::prelude::*;
use std::cmp::{min,max};

const WORLD_WIDTH: i32 = 80;
const WORLD_HEIGHT: i32 = 50;

struct State {
    ecs: World,
    debug_mode: bool,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        if self.debug_mode {
            ctx.print(1, 1, format!("{:.2}fps", ctx.fps));
        }
        

        player_input(self, ctx);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Vec<TileType>>();
        
        draw_map(&map, ctx);

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: u16,
    fg: RGB,
    bg: RGB,
}

#[derive(Component)]
struct Player {}

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

fn main() {
    use rltk::RltkBuilder;
    let ctx = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()
        .unwrap();

    let mut gs = State {
        ecs: World::new(),
        debug_mode: false,
    };
    
    register_components(&mut gs.ecs);

    let player_start = Position { x: 40, y: 25 };

    gs.ecs.insert(new_map(&player_start));

    gs.ecs
        .create_entity()
        .with(player_start)
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();

    rltk::main_loop(ctx, gs).unwrap();
}

fn register_components(ecs: &mut World) {
    ecs.register::<Position>();
    ecs.register::<Renderable>();
    ecs.register::<Player>();
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
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

fn player_input(gs: &mut State, ctx: &mut Rltk) {
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

fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * WORLD_WIDTH as usize) + x as usize
}

fn new_map(player_start: &Position) -> Vec<TileType> {
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

fn draw_map(map: &[TileType], ctx: &mut Rltk) {
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
