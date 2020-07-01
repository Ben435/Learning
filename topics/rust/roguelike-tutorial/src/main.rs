mod map;
mod constants;
mod components;
mod player;
mod rect;
mod visibility_system;
use map::*;
use components::*;
use player::*;
use visibility_system::VisibilitySystem;

use rltk::{Rltk,GameState,RGB,Point};
use specs::prelude::*;


pub struct State {
    ecs: World,
    debug_mode: bool,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        
        draw_map(&self.ecs, ctx, self.debug_mode);

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }

        if self.debug_mode {
            ctx.print_color(
                1, 1, 
                RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), 
                format!("{:.2}fps", ctx.fps)
            );
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
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

    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    let player_start = Position { x: player_x, y: player_y };

    gs.ecs.insert(map);

    gs.ecs
        .create_entity()
        .with(player_start)
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed {
            visible_tiles: Vec::new(), range: 8,
        })
        .build();

    rltk::main_loop(ctx, gs).unwrap();
}

fn draw_map(ecs: &World, ctx: &mut Rltk, debug_mode: bool) {
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (_player, viewshed) in (&mut players, &mut viewsheds).join() {
        let mut x = 0;
        let mut y = 0;
        for tile in map.tiles.iter() {
            let pt = Point::new(x, y);

            // If visible, then draw
            if viewshed.visible_tiles.contains(&pt) {
                match tile {
                    TileType::Floor => {
                        ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
                    }
                }
            } else if debug_mode {
                match tile {
                    TileType::Floor => {
                        ctx.set(x, y, RGB::from_f32(0.1, 0.1, 0.1), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, RGB::from_f32(0.0, 0.2, 0.0), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
                    }
                }
            }
            
            x += 1;
            if x > map.width - 1 {
                x = 0;
                y += 1;
            }
        }
    }
}
