use std::f64;
use std::vec;
use std::vec::Vec;
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
    pub fn render(&self, img: &mut Scene) -> &self::Engine {
        self.context.put_image_data(&img.image(), 0.0, 0.0);
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

pub struct Scene {
    width: usize,
    height: usize,
    pixel_count: usize,
    data_size: usize,
    image: Vec<u8>,
    z_buffer: Vec<Vec<f32>>,
}

impl Scene {
    pub fn dot(&mut self, x: usize, y: usize, z: f32, r: u8, g: u8, b: u8, a: u8) {
        let red_index = (self.width * y + x) * 4;
        self.image[red_index] = r;
        self.image[red_index + 1] = g;
        self.image[red_index + 2] = b;
        self.image[red_index + 3] = a;
        self.z_buffer[x][y] = z;
    }
    pub fn image(&mut self) -> ImageData {
        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut self.image),
            self.width as u32,
            self.height as u32,
        )
        .unwrap()
    }
    pub fn clear(&mut self) {
        let mut i = 0;
        while i < self.data_size {
            self.image[i] = 0;
            self.image[i + 1] = 0;
            self.image[i + 2] = 0;
            self.image[i + 3] = 255;
            i += 4;
        }
    }
    pub fn new(width: usize, height: usize) -> Scene {
        let pixel_count = width * height;
        let data_size = pixel_count * 4;
        Scene {
            width: width,
            height: height,
            pixel_count: pixel_count,
            data_size: data_size,
            image: vec![0; data_size],
            z_buffer: vec![vec![-100.0; height]; width],
        }
    }
}
