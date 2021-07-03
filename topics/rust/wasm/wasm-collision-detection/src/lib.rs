mod geometry;
mod engine;
mod utils;

use cgmath::Point2;
use geometry::{GeometryId,GeometryManager,dimension_2d::*};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub struct GeometryManagerWrapper {
    geometry_manager: GeometryManager2D
}

#[wasm_bindgen]
impl GeometryManagerWrapper {
    pub fn new() -> GeometryManagerWrapper {
        GeometryManagerWrapper {
            geometry_manager: GeometryManager2D::new(),
        }
    }

    pub fn add_circle(&mut self, origin_x: f64, origin_y: f64, radius: f64) -> GeometryId {
        let circle = Circle::new(Point2::new(origin_x, origin_y), radius);
    
        self.geometry_manager.add_circle(circle)
    }
}

#[wasm_bindgen]
pub fn get_2d_manager() -> GeometryManagerWrapper {
    GeometryManagerWrapper::new()
}
