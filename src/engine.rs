use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use web_sys::ImageData;
use web_sys::*;

pub struct Engine {
    width: u32,
    height: u32,
    id: String,
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
}

impl Engine {
    pub fn render(&self, img: &mut Vec<u8>) -> &self::Engine {
        self.context.put_image_data(
            &ImageData::new_with_u8_clamped_array_and_sh(Clamped(img), self.width, self.height)
                .unwrap(),
            0.0,
            0.0,
        );
        self
    }
    pub fn new(id: &str) -> Engine {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas: web_sys::HtmlCanvasElement = document
            .get_element_by_id(id)
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        Engine {
            width: canvas.width(),
            height: canvas.height(),
            id: String::from(id),
            canvas: canvas,
            context: context,
        }
    }
}
