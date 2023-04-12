extern crate alloc;
extern crate console_error_panic_hook;

use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

// SAFETY: This App is single threaded, using AssumeSingleThreaded is allowed
#[global_allocator]
static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
    unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

mod game;
mod rng;

use crate::game::Game;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Wasm {
    game: Game,
}

#[wasm_bindgen]
impl Wasm {
    // #[wasm_bindgen(constructor)]
    // pub fn new(view_w: i16, view_h: i16, map_w: i16, map_h: i16, seed: i64) -> Self {
    //     console_error_panic_hook::set_once();
    //     let game = Game::new(..);
    // }

    pub fn render(&mut self) {
        //
    }

    pub fn slide_view(&mut self) {
        //
    }
}
