use rltk::{RGB,Rltk};
use crate::components::{Player,CombatStats};
use specs::prelude::*;
use crate::constants::*;
use crate::gamelog::GameLog;

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    ctx.draw_box(0, WORLD_HEIGHT, WORLD_WIDTH-1, 6, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));

    let combat_states = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();
    for (_player, stats) in (&players, &combat_states).join() {
        let health = format!("HP: {} / {}", stats.hp, stats.max_hp);
        ctx.print_color(12, WORLD_HEIGHT, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &health);

        ctx.draw_bar_horizontal(28, 43, 51, stats.hp, stats.max_hp, RGB::named(rltk::RED), RGB::named(rltk::BLACK));
    }

    let log = ecs.fetch::<GameLog>();
    let mut y = 44;
    for s in log.entries().iter().rev() {
        if y < 49 {
            ctx.print(2, y, s);
        }
        y += 1;
    }

    
}