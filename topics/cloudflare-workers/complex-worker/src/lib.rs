extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn run(num: u32) -> u32 {
    let res = fibonnaci(num);

    return res;
}

pub fn fibonnaci(num: u32) -> u32 {
    let mut cur = 1;
    let mut prev = 0;
    let mut i = 0;

    while i < num {
        let tmp = cur + prev;
        prev = cur;
        cur = tmp;

        i += 1;
    }

    return cur
}
