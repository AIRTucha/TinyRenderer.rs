mod common;
mod engine;
mod get;
mod obj;
mod run;

use engine::Engine;
use engine::Scene;
use get::get;
use obj::Obj;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::ImageData;
use web_sys::*;

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
    let engine = Engine::new("canvas");
    let mut scene = Scene::new(100, 100);
    scene.clear();
    // for x in 5..50 {
    //     scene.dot(x, 50, 0.0, 255, 0, 0, 255)
    // }
    diamond(100, &mut scene);
    engine.render(&mut scene);
    run!(async {
        let resp = Obj::new(&"obj/african_head/african_head.obj").await;
        console::log_1(&JsValue::from(resp.vertices[0].x.to_string()));
    });
    Ok(())
}

fn diamond(size: usize, scene: &mut Scene) {
    for row in 0..size {
        diamond_row(row, size, scene);
    }
}

fn diamond_row(position: usize, size: usize, scene: &mut Scene) {
    let mut counter = 0;
    let hsize = size / 2;
    let index = if hsize >= position {
        hsize - position
    } else {
        position - hsize
    };
    loop {
        if counter == size {
            break;
        } else if counter >= index && counter < size - index {
            scene.dot(counter, position, 0.0, 255, 0, 0, 255);
        }
        counter += 1;
    }
}

fn draw_figure(figure: Vec<Vec<char>>) {
    figure.iter().for_each(|row| {
        println!["{}", String::from_iter(row)];
    });
}
