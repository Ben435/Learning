use specs::prelude::*;
use rltk::console;
use super::{CombatStats, SufferDamage, Player};

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = ( WriteStorage<'a, CombatStats>,
                        WriteStorage<'a, SufferDamage> );

    fn run(&mut self, data : Self::SystemData) {
        let (mut stats, mut damage) = data;

        // Apply damage for round
        for (mut stats, damage) in (&mut stats, &damage).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();
        }

        // Clear for next round
        damage.clear();
    }
}

pub fn delete_the_dead(ecs: &mut World) {
    let mut dead: Vec<Entity> = Vec::new();
    
    {
        let combat_stats = ecs.read_storage::<CombatStats>();
        let entities = ecs.entities();
        let players = ecs.read_storage::<Player>();
        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.hp < 1 {
                match players.get(entity) {
                    None => dead.push(entity),
                    Some(_) => console::log("You are dead")
                }
                
            }
        }
    }

    for victim in dead {
        ecs.delete_entity(victim).expect("Unable to delete");
    }
}
