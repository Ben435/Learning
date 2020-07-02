use specs::prelude::*;
use crate::components::{Name,Monster,Viewshed,Position};
use crate::map::Map;
use rltk::{Point};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = ( ReadExpect<'a, Map>,
                        ReadExpect<'a, Point>, 
                        WriteStorage<'a, Viewshed>,
                        ReadStorage<'a, Monster>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, Position>,);

    fn run(&mut self, data: Self::SystemData) {
        let (map, player_pos, mut viewshed, monster, name, mut position) = data;

        for (viewshed,_monster, name, pos) in (&mut viewshed, &monster, &name, &mut position).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
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
