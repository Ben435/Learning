use rltk::{Algorithm2D,BaseMap,Point,SmallVec,RGB,Rltk};
use specs::{Entity,World};
use std::cmp::{min,max};
use crate::constants::*;
use crate::rect::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles : Vec<bool>,
    pub visible_tiles : Vec<bool>,
    pub blocked_tiles: Vec<bool>,
    pub tile_content: Vec<Vec<Entity>>,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn idx_in_bounds(&self, idx: usize) -> bool {
        idx > 0 && idx < (self.width*self.height) as usize
    }

    /// Set tile at co-ords to tile. Returns true if successful, false if not (eg: outside of bounds)
    pub fn set_tile(&mut self, x: i32, y: i32, tile: TileType) -> bool {
        let idx = self.xy_idx(x, y);
        if self.idx_in_bounds(idx) {
            self.tiles[idx] = tile;
            return true;
        }

        return false;
    }

    /// Set co-ords to revealed. Returns true if successful, false if not (eg: outside of bounds)
    pub fn set_revealed(&mut self, x: i32, y: i32, revealed: bool) -> bool {
        let idx = self.xy_idx(x, y);
        if self.idx_in_bounds(idx) {
            self.revealed_tiles[idx] = revealed;
            return true;
        }

        return false;
    }

    /// Set co-ords to visible. Returns true if successful, false if not (eg: outside of bounds)
    pub fn set_visible(&mut self, x: i32, y: i32, revealed: bool) -> bool {
        let idx = self.xy_idx(x, y);
        if self.idx_in_bounds(idx) {
            self.visible_tiles[idx] = revealed;
            return true;
        }

        return false;
    }

    pub fn populate_blocked(&mut self) {
        for (i,tile) in self.tiles.iter().enumerate() {
            self.blocked_tiles[i] = *tile == TileType::Wall;
        }
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    pub fn new_map_rooms_and_corridors() -> Map {
        let tile_count = WORLD_WIDTH as usize * WORLD_HEIGHT as usize;
        let mut map = Map {
            tiles: vec![TileType::Wall; tile_count],
            rooms: Vec::new(),
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
            revealed_tiles: vec![false; tile_count],
            visible_tiles: vec![false; tile_count],
            blocked_tiles: vec![false; tile_count],
            tile_content: vec![Vec::new(); tile_count],
        };

        const MAX_ROOMS : i32 = 30;
        const MIN_SIZE : i32 = 6;
        const MAX_SIZE : i32 = 10;
    
        let mut rng = rltk::RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, WORLD_WIDTH - w - 1) - 1;
            let y = rng.roll_dice(1, WORLD_HEIGHT - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
    
            let ok = map.rooms
                .iter() // Ok if no other room intersects with this one.
                .all(|other_room| !other_room.intersect(&new_room));
    
            if ok {
                map.apply_room_to_map(&new_room);
    
                if !map.rooms.is_empty() {
                    // Connect to existing rooms
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
    
                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }
    
                map.rooms.push(new_room);
            }
        }
    
        map
    }
    
    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                self.set_tile(x, y, TileType::Floor);
            }
        }
    }
    
    fn apply_horizontal_tunnel(&mut self, x1:i32, x2:i32, y:i32) {
        for x in min(x1,x2) ..= max(x1,x2) {
            self.set_tile(x, y, TileType::Floor);
        }
    }
    
    fn apply_vertical_tunnel(&mut self, y1:i32, y2:i32, x:i32) {
        for y in min(y1,y2) ..= max(y1,y2) {
            self.set_tile(x, y, TileType::Floor);
        }
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width-1 || y < 1 || y > self.height-1 { 
            return false;
        }
        let idx = self.xy_idx(x, y);

        !self.blocked_tiles[idx]
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);

        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;

        // Cardinal directions
        if self.is_exit_valid(x-1, y) { exits.push((idx-1, 1.0)) };
        if self.is_exit_valid(x+1, y) { exits.push((idx+1, 1.0)) };
        if self.is_exit_valid(x, y-1) { exits.push((idx-w, 1.0)) };
        if self.is_exit_valid(x, y+1) { exits.push((idx+w, 1.0)) };

        // Diagonals
        if self.is_exit_valid(x-1, y-1) { exits.push(((idx-w)-1, 1.45)); }
        if self.is_exit_valid(x+1, y-1) { exits.push(((idx-w)+1, 1.45)); }
        if self.is_exit_valid(x-1, y+1) { exits.push(((idx+w)-1, 1.45)); }
        if self.is_exit_valid(x+1, y+1) { exits.push(((idx+w)+1, 1.45)); }

        exits
    }
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk, debug_mode: bool) {
    let map = ecs.fetch::<Map>();

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
