use crate::common::Vec3;
use crate::matrix::Matrix4x4;
use std::f64;
use std::vec;
use std::vec::Vec;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::ImageData;

pub struct Engine {
    width: u32,
    height: u32,
    context: web_sys::CanvasRenderingContext2d,
}

impl Engine {
    pub fn render(&self, img: &mut Scene) {
        self.context.put_image_data(&img.image(), 0.0, 0.0);
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
            context: context,
        }
    }
    pub fn create_scene(&self) -> Scene {
        Scene::new(self.width as usize, self.height as usize)
    }
}

pub struct Scene {
    width: usize,
    height: usize,
    data_size: usize,
    image: Vec<u8>,
    z_buffer: Vec<Vec<f64>>,
    matrix: Matrix4x4,
}

impl Scene {
    pub fn dot(&mut self, x: usize, y: usize, z: f64, r: u8, g: u8, b: u8, a: u8) {
        if self.z_buffer[x][y] < z {
            let red_index = (self.width * y + x) * 4;
            self.image[red_index] = r;
            self.image[red_index + 1] = g;
            self.image[red_index + 2] = b;
            self.image[red_index + 3] = a;
            self.z_buffer[x][y] = z;
        }
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
    pub fn scale(&self, vec: &Vec3) -> Vec3 {
        (*vec) * &self.matrix
    }

    pub fn new(width: usize, height: usize) -> Scene {
        let pixel_count = width * height;
        let data_size = pixel_count * 4;
        let widthf = width as f64;
        let heightf = height as f64;
        Scene {
            width: width,
            height: height,
            data_size: data_size,
            image: vec![0; data_size],
            z_buffer: vec![vec![-100.0; height]; width],
            matrix: Matrix4x4::new(
                [widthf / 2.0, 0.0, 0.0, widthf / 2.0],
                [0.0, -heightf / 2.0, 0.0, heightf / 2.0],
                [0.0, 0.0, 1.0, 1.0],
                [0.0, 0.0, 0.0, 0.0],
            ),
        }
    }
}
