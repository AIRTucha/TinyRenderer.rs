use crate::common::{Color, Vec3};
use std::f64;
use std::vec::Vec;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;

pub struct Texture {
    pub data: Clamped<Vec<u8>>,
    width: f64,
    height: f64,
}

impl Texture {
    pub async fn new(name: &str) -> Texture {
        let document = web_sys::window().unwrap().document().unwrap();
        let image: web_sys::HtmlImageElement = document
            .get_element_by_id(name)
            .unwrap()
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();
        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        canvas.set_width(image.width());
        canvas.set_height(image.height());
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        ctx.draw_image_with_html_image_element(&image, 0.0, 0.0);
        Texture {
            data: ctx
                .get_image_data(
                    0.0,
                    0.0,
                    f64::from(image.width()),
                    f64::from(image.height()),
                )
                .unwrap()
                .data(),
            width: f64::from(image.width()),
            height: f64::from(image.height()),
        }
    }

    pub fn get_color(&self, x: f64, y: f64) -> Color {
        let widthSpan = (self.width * (x)).floor();
        let heightSpan = (self.width * (self.height * (1.0 - y)).floor()).floor();
        let redIndex = ((widthSpan + heightSpan) * 4.0).floor() as usize;
        Color {
            r: self.data[redIndex],
            g: self.data[redIndex + 1],
            b: self.data[redIndex + 2],
            a: self.data[redIndex + 3],
        }
    }
    pub fn getVec3(&self, x: f64, y: f64) -> Vec3 {
        let widthSpan = (self.width * x).floor();
        let heightSpan = (self.width * (self.height * (1.0 - y)).floor()).floor();
        let redIndex = ((widthSpan + heightSpan) * 4.0).floor() as usize;
        Vec3::new(
            f64::from(self.data[redIndex]),
            f64::from(self.data[redIndex + 1]),
            f64::from(self.data[redIndex + 2]),
        )
    }

    pub fn getTiplet(&self, x: f64, y: f64) -> (u8, u8, u8) {
        let widthSpan = (self.width * x).floor();
        let heightSpan = (self.width * (self.height * (1.0 - y))).floor();
        let redIndex = ((widthSpan + heightSpan) * 4.0) as usize;
        (
            self.data[redIndex],
            self.data[redIndex + 1],
            self.data[redIndex + 2],
        )
    }
}
