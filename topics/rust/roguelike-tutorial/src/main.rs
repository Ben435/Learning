mod map;
mod constants;
mod components;
mod player;
mod rect;
mod visibility_system;
mod monster_ai_system;
use map::*;
use components::*;
use player::*;
use visibility_system::VisibilitySystem;
use monster_ai_system::MonsterAI;

use rltk::{Rltk,GameState,RGB,Point};
use specs::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { Paused, Running }

pub struct State {
    pub ecs: World,
    pub runstate : RunState,
    pub debug_mode: bool,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        if self.runstate == RunState::Running {
            self.run_systems();

            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
        }       

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();
        
        draw_map(&map, ctx, self.debug_mode);

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);

            // Only render visible renderables.
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            } else if self.debug_mode {
                ctx.set(pos.x, pos.y, render.fg.desaturate(), render.bg, render.glyph);
            }
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

        let mut monster_ai = MonsterAI{};
        monster_ai.run_now(&self.ecs);

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
        runstate: RunState::Running,
        debug_mode: false,
    };
    
    register_components(&mut gs.ecs);

    let map = Map::new_map_rooms_and_corridors();

    // Start the player in the center of a room
    let (player_x, player_y) = map.rooms[0].center();
    let player_start = Position { x: player_x, y: player_y };

    gs.ecs
        .create_entity()
        .with(player_start)
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Name{ name: "Player".to_string() })
        .with(Viewshed {
            visible_tiles: Vec::new(), 
            range: 8,
            dirty: true,
        })
        .build();

    gs.ecs.insert(Point::new(player_x, player_y));

    // Spawn some mobs
    let mut rng = rltk::RandomNumberGenerator::new();
    for (idx,room) in map.rooms.iter().skip(1).enumerate() {
        let (x,y) = room.center();
        let glyph: u16;
        let name: String;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => { 
                glyph = rltk::to_cp437('g');
                name = "Goblin".to_string();
            }
            _ => { 
                glyph = rltk::to_cp437('o');
                name = "Orc".to_string();
            }
        }

        gs.ecs.create_entity()
            .with(Position{ x, y })
            .with(Renderable{
                glyph: glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Name{ name: format!("{} #{}", name, idx) })
            .with(Viewshed{ visible_tiles : Vec::new(), range: 8, dirty: true })
            .with(Monster {})
            .build();
    }

    gs.ecs.insert(map);

    rltk::main_loop(ctx, gs).unwrap();
}

fn draw_map(map: &Map, ctx: &mut Rltk, debug_mode: bool) {
    let mut x = 0;
    let mut y = 0;

    for (idx,tile) in map.tiles.iter().enumerate() {
        // If visible, then draw
        if map.revealed_tiles[idx] {
            let glyph;
            let mut fg;
            match tile {
                TileType::Floor => {
                    glyph = rltk::to_cp437('.');
                    fg = RGB::from_f32(0.5, 0.5, 0.5);
                }
                TileType::Wall => {
                    glyph = rltk::to_cp437('#');
                    fg = RGB::from_f32(0.0, 1.0, 0.0);
                }
            }
            if !map.visible_tiles[idx] {
                fg = fg.to_greyscale();
            }
            ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
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

#[cfg(test)]
mod test {
    #[test]
    fn vec_test() {
        let mut a = vec![1, 2, 3];
        assert_eq!(a[1], 2);
        a.clear();
        a[2] = 6;
        assert_eq!(a[1], 2);
    }
}
