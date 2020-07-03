mod map;
mod constants;
mod components;
mod player;
mod rect;
mod visibility_system;
mod monster_ai_system;
mod map_indexing_system;
mod melee_combat_system;
mod damage_system;
mod gui;
mod gamelog;

use map::*;
use components::*;
use player::*;
use visibility_system::VisibilitySystem;
use monster_ai_system::MonsterAI;
use map_indexing_system::MapIndexingSystem;
use melee_combat_system::MeleeCombatSystem;
use damage_system::{DamageSystem,delete_the_dead};
use gui::draw_ui;
use gamelog::GameLog;

use rltk::{Rltk,GameState,RGB,Point};
use specs::prelude::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RunState { 
    AwaitingInput, 
    PreRun, 
    PlayerTurn, 
    MonsterTurn,
}

pub struct State {
    pub ecs: World,
    pub debug_mode: bool,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        match newrunstate {
            RunState::PreRun => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                newrunstate = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                newrunstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
        }
        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }
        delete_the_dead(&mut self.ecs);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();
        
        draw_map(&self.ecs, ctx, self.debug_mode);

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);

            // Only render visible renderables.
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            } else if self.debug_mode {
                ctx.set(pos.x, pos.y, render.fg.desaturate(), render.bg, render.glyph);
            }
        }

        draw_ui(&self.ecs, ctx);

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

        let mut map_indexing = MapIndexingSystem{};
        map_indexing.run_now(&self.ecs);

        let mut melee_combat_system = MeleeCombatSystem{};
        melee_combat_system.run_now(&self.ecs);

        let mut damage_system = DamageSystem{};
        damage_system.run_now(&self.ecs);

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

    // Start the player in the center of a room
    let (player_x, player_y) = map.rooms[0].center();
    let player_start = Position { x: player_x, y: player_y };

    let player_entity = gs.ecs
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
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5
        })
        .build();

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
            .with(BlocksTile {})
            .with(CombatStats {
                max_hp: 16,
                hp: 16,
                defense: 1,
                power: 4
            })
            .build();
    }

    gs.ecs.insert(GameLog::new(&["Welcome to Rusty Roguelike".to_string()]));
    gs.ecs.insert(rng);
    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::PreRun);

    rltk::main_loop(ctx, gs).unwrap();
}
