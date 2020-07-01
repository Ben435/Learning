use rltk::{Rltk,GameState,RGB};
use specs_derive::Component;
use specs::prelude::*;

const WORLD_WIDTH: i32 = 80;
const WORLD_HEIGHT: i32 = 50;

struct State {
    ecs: World,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker{};
        lw.run_now(&self.ecs);
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
struct LeftMover {
    pub distance: i32,
}

struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>,
                        WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (lefty,pos) in (&lefty, &mut pos).join() {
            pos.x -= lefty.distance;
            if pos.x < 0 {
                pos.x = WORLD_WIDTH - 1;
            }
        }
    }
}

#[derive(Component)]
struct Player {}

fn main() {
    use rltk::RltkBuilder;
    let ctx = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()
        .unwrap();

    let mut gs = State {
        ecs: World::new(),
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<LeftMover>();

    for i in 0..5 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 10, y: 20 })
            .with(Renderable {
                glyph: rltk::to_cp437('#'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(LeftMover {
                distance: (i % 3) + 1,
            })
            .build();
    }
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();

    rltk::main_loop(ctx, gs).unwrap();
}
