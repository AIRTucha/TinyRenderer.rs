mod engine;
use engine::Engine;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::ImageData;
use web_sys::*;
// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    let engine = Engine::new("canvas".to_string());

    let image = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut [100; 40000]), 100, 100)?;
    engine.render(&image);
    Ok(())
}
