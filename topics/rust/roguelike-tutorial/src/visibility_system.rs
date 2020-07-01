use specs::prelude::*;
use rltk::{field_of_view,Point};
use crate::components::{Viewshed, Position, Player};
use crate::map::Map;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = ( WriteExpect<'a, Map>,
                        Entities<'a>,
                        WriteStorage<'a, Viewshed>,
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Player>);
    
    fn run(&mut self, (mut map, entities, mut viewshed, pos, player): Self::SystemData) {
        for (entity,viewshed,pos) in (&entities, &mut viewshed, &pos).join() {
            // Only update dirty viewshed's.
            if !viewshed.dirty {
                continue;
            }
            viewshed.dirty = false;

            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x <= map.width && p.y >= 0 && p.y <= map.height);

            // If a player, add to revealed tiles
            let _p: Option<&Player> = player.get(entity);
            if let Some(_p) = _p {
                // Reset visible tiles;
                for t in map.visible_tiles.iter_mut() { 
                    *t = false;
                }
                for vis in viewshed.visible_tiles.iter() {
                    map.set_visible(vis.x, vis.y, true);
                    map.set_revealed(vis.x, vis.y, true);
                }
            }
        }
    }
}
