use std::vec;
use std::vec::Vec;
use wasm_bindgen::Clamped;
use web_sys::ImageData;

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
            z_buffer: Vec::with_capacity(width),
        }
    }
}
