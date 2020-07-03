use specs::prelude::*;
use crate::gamelog::GameLog;
use crate::components::*;

pub struct ItemCollectionSystem {}

impl<'a> System<'a> for ItemCollectionSystem {
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        WriteStorage<'a, WantsToPickupItem>,
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, InBackpack>,
                      );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, mut gamelog, mut wants_pickup, mut positions, names, mut backpack) = data;

        for pickup in wants_pickup.join() {
            positions.remove(pickup.item);
            backpack.insert(pickup.item, InBackpack{ owner: pickup.collected_by }).expect("Unable to insert backpack entry");

            if pickup.collected_by == *player_entity {
                gamelog.info(format!("You pick up the {}.", names.get(pickup.item).unwrap().name));
            }
        }
        wants_pickup.clear();
    }
}

pub struct PotionUseSystem {}

impl<'a> System<'a> for PotionUseSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        Entities<'a>,
                        WriteStorage<'a, WantsToDrinkPotion>,
                        ReadStorage<'a, Name>,
                        ReadStorage<'a, Potion>,
                        WriteStorage<'a, CombatStats>
                      );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, mut gamelog, entities, mut wants_drink, names, potions, mut combat_stats) = data;

        for (entity, drink, stats) in (&entities, &wants_drink, &mut combat_stats).join() {
            let potion = potions.get(drink.potion);

            match potion {
                None => {},
                Some(potion) => {
                    stats.hp = i32::min(stats.max_hp, stats.hp + potion.heal_amount);
                    let name = &names.get(drink.potion).unwrap().name;
                    if entity == *player_entity {
                        gamelog.info(format!("You drink to {}, healing {} hp", name, potion.heal_amount))
                    }
                    entities.delete(drink.potion).expect("Delete failed!");
                }
            }
        }

        wants_drink.clear();
    }
}
