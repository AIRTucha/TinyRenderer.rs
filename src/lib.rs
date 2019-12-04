mod common;
mod engine;
mod get;
mod matrix;
mod obj;
mod pipeline;
mod run;
mod texture;

use crate::pipeline::{Pipeline, Renderer};
use engine::Engine;
use obj::Obj;
use wasm_bindgen::prelude::*;

use wasm_bindgen::JsValue;

use std::iter::FromIterator;

#[macro_use]
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
    run!(async {
        let engine = Engine::new("canvas");
        let mut scene = engine.create_scene();
        let obj = Obj::new(
            &"obj/african_head/african_head.obj",
            &"diffuse",
            &"nm",
            &"spec",
        )
        .await;

        let mut render = Renderer {};

        render.draw(&obj, &mut scene);
        engine.render(&mut scene);
    });
    Ok(())
}
