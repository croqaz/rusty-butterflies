extern crate alloc;
extern crate console_error_panic_hook;

use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

// SAFETY: This App is single threaded, using AssumeSingleThreaded is allowed
#[global_allocator]
static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
    unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

mod game;
mod geom;
mod map_actor;
mod map_common;
mod map_decor;
mod map_things;
mod rng;
// mod util;

use crate::game::Game;
use crate::geom::Direction;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Wasm {
    game: Game,
}

#[wasm_bindgen]
impl Wasm {
    #[wasm_bindgen(constructor)]
    pub fn new(view_w: i16, view_h: i16, map_w: i16, map_h: i16, seed: i64) -> Self {
        console_error_panic_hook::set_once();
        let game = Game::new(view_w, view_h, map_w, map_h, seed);
        match game {
            Some(g) => Self { game: g },
            None => {
                panic!("Invalid game params!")
            }
        }
    }

    pub fn render(&mut self) -> JsValue {
        let grid = self.game.render();
        serde_wasm_bindgen::to_value(&grid).unwrap()
    }

    pub fn slide_view(&mut self, dir: Direction) -> JsValue {
        let (d_x, d_y) = dir.to_offset();
        let ok = self.game.slide_view(d_x, d_y);
        serde_wasm_bindgen::to_value(&ok).unwrap()
    }
}
