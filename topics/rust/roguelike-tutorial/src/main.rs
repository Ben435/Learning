extern crate serde;

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
mod spawner;
mod inventory_system;
mod saveload_system;

use map::*;
use components::*;
use player::*;
use visibility_system::VisibilitySystem;
use monster_ai_system::MonsterAI;
use map_indexing_system::MapIndexingSystem;
use melee_combat_system::MeleeCombatSystem;
use damage_system::{DamageSystem,delete_the_dead};
use gamelog::GameLog;
use inventory_system::*;

use rltk::{Rltk,GameState,RGB,Point};
use specs::prelude::*;
use specs::saveload::{SimpleMarkerAllocator};

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    MainMenu {
        menu_selection: gui::MainMenuSelection
    },
    SaveGame,
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    ShowInventory,
    ShowDropItem,
    ShowTargeting {
        range: i32,
        item: Entity,
    }
}

pub struct State {
    pub ecs: World,
    pub debug_mode: bool,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        ctx.cls();

        match newrunstate {
            RunState::MainMenu{ .. } => {
                let result = gui::main_menu(self, ctx);
                match result {
                    gui::MainMenuResult::NoSelection{ selected } => newrunstate = RunState::MainMenu{ menu_selection: selected },
                    gui::MainMenuResult::Selected{ selected } => {
                        match selected {
                            gui::MainMenuSelection::NewGame => newrunstate = RunState::PreRun,
                            gui::MainMenuSelection::LoadGame => {
                                saveload_system::load_game(&mut self.ecs);
                                newrunstate = RunState::AwaitingInput;
                                saveload_system::delete_save();
                            }
                            gui::MainMenuSelection::Quit => ctx.quit(),
                        }
                    }
                }
            }
            RunState::SaveGame => {
                saveload_system::save_game(&mut self.ecs);

                newrunstate = RunState::MainMenu{ menu_selection: gui::MainMenuSelection::LoadGame };
            }
            _ => {
                draw_map(&self.ecs, ctx, self.debug_mode);

                let positions = self.ecs.read_storage::<Position>();
                let renderables = self.ecs.read_storage::<Renderable>();
                let map = self.ecs.fetch::<Map>();

                let mut data = (&positions, &renderables).join().collect::<Vec<_>>();
                data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));
                for (pos, render) in data.iter() {
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

                gui::draw_ui(&self.ecs, ctx);
            }
        }

        match newrunstate {
            RunState::PreRun => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                newrunstate = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::ShowInventory => {
                let result = gui::show_inventory(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {},
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let is_ranged = self.ecs.read_storage::<Ranged>();
                        let is_item_ranged = is_ranged.get(item_entity);

                        if let Some(ranged_item) = is_item_ranged {
                            newrunstate = RunState::ShowTargeting{
                                range: ranged_item.range,
                                item: item_entity,
                            };
                        } else {
                            let mut intent = self.ecs.write_storage::<WantsToUseItem>();
                            intent.insert(*self.ecs.fetch::<Entity>(), WantsToUseItem{ item: item_entity, target: None }).expect("Unable to insert intent");
    
                            newrunstate = RunState::PlayerTurn;
                        }
                    }
                }
            }
            RunState::ShowDropItem => {
                let result = gui::drop_item_menu(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let mut intent = self.ecs.write_storage::<WantsToDropItem>();
                        intent.insert(*self.ecs.fetch::<Entity>(), WantsToDropItem{ item: item_entity }).expect("Unable to insert intent");
                        
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::ShowTargeting{range, item} => {
                let result = gui::ranged_target(self, ctx, range);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let mut intent = self.ecs.write_storage::<WantsToUseItem>();
                        intent.insert(*self.ecs.fetch::<Entity>(), WantsToUseItem{ item, target: result.1 }).expect("Unable to insert intent");

                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            _ => {}
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }
        delete_the_dead(&mut self.ecs);
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

        let mut item_system = ItemCollectionSystem{};
        item_system.run_now(&self.ecs);

        let mut item_use_system = ItemUseSystem{};
        item_use_system.run_now(&self.ecs);

        let mut drop_items = ItemDropSystem{};
        drop_items.run_now(&self.ecs);

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
    gs.ecs.insert(rltk::RandomNumberGenerator::new());
    gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());

    let map = Map::new_map_rooms_and_corridors(&mut gs.ecs);

    // Start the player in the center of a room
    let (player_x, player_y) = map.rooms[0].center();

    let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);

    // Spawn some mobs
    for room in map.rooms.iter().skip(1) {
        spawner::spawn_room(&mut gs.ecs, room);
    }

    gs.ecs.insert(map);
    gs.ecs.insert(GameLog::new(&["Welcome to Rusty Roguelike".to_string()]));
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::MainMenu{ menu_selection: gui::MainMenuSelection::NewGame });

    rltk::main_loop(ctx, gs).unwrap();
}
