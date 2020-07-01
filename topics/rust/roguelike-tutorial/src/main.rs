mod map;
mod constants;
mod components;
mod player;
mod rect;
use map::*;
use components::*;
use player::*;

use rltk::{Rltk,GameState,RGB};
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
        let map = self.ecs.fetch::<Map>();
        
        draw_map(&map, ctx);

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }

        if self.debug_mode {
            ctx.print(1, 1, format!("{:.2}fps", ctx.fps));
        }
    }
}

impl State {
    fn run_systems(&mut self) {
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
        .build();

    rltk::main_loop(ctx, gs).unwrap();
}
