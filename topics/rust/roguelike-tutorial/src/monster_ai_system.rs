use specs::prelude::*;
use crate::components::{Monster,Viewshed,Position,WantsToMelee};
use crate::map::Map;
use rltk::{Point};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = ( ReadExpect<'a, Map>,
                        ReadExpect<'a, Point>,
                        ReadExpect<'a, Entity>,
                        Entities<'a>,
                        WriteStorage<'a, Viewshed>,
                        ReadStorage<'a, Monster>,
                        WriteStorage<'a, Position>,
                        WriteStorage<'a, WantsToMelee>);

    fn run(&mut self, data: Self::SystemData) {
        let (map, player_pos, player_entity, entities, mut viewshed, monster, mut position, mut wants_to_melee) = data;

        for (entity, viewshed,_monster, pos) in (&entities, &mut viewshed, &monster, &mut position).join() {

            let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
            if distance < 1.5 {
                wants_to_melee.insert(entity, WantsToMelee{ target: *player_entity }).expect("Unable to attack!");
            } else if viewshed.visible_tiles.contains(&*player_pos) {
                let path = rltk::a_star_search(
                    map.xy_idx(pos.x, pos.y) as i32,
                    map.xy_idx(player_pos.x, player_pos.y) as i32,
                    &* map,
                );

                if path.success && path.steps.len() > 1 {
                    pos.x = path.steps[1] as i32 % map.width;
                    pos.y = path.steps[1] as i32 / map.width;
                    viewshed.dirty = true;
                }
            }
        }
    }
}
