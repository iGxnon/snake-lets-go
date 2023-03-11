mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

/// Represent a cell in snake-lets-go
/// each cell is represented as a single byte
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Blank = 0,  // a blank cell
    Body = 1,   // a snake body cell
    Head1 = 2,  // a snake head cell
    Head2 = 3,  //
    Food1 = 10, //
}
