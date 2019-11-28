use std::vec;
use std::vec::Vec;
use wasm_bindgen::Clamped;

pub struct Scene {
    width: usize,
    height: usize,
    pixelCount: usize,
    dataSize: usize,
    image: Vec<u8>,
    z_buffer: Vec<Vec<f32>>,
}

impl Scene {
    pub fn dot(&mut self, x: usize, y: usize, z: f32, r: u8, g: u8, b: u8, a: u8) {
        let redIndex = (self.width * y + x) * 4;
        self.image[redIndex] = r;
        self.image[redIndex + 1] = g;
        self.image[redIndex + 2] = b;
        self.image[redIndex + 3] = a;
        self.z_buffer[x][y] = z;
    }
    pub fn image(&mut self) -> &mut Vec<u8> {
        &mut self.image
    }
    pub fn clear(&mut self) {
        let mut i = 0;
        while i < self.dataSize {
            self.image[i] = 0;
            self.image[i + 1] = 0;
            self.image[i + 2] = 0;
            self.image[i + 3] = 255;
            i += 4;
        }
    }
    pub fn new(width: usize, height: usize) -> Scene {
        let pixelCount = width * height;
        let dataSize = pixelCount * 4;
        Scene {
            width: width,
            height: height,
            pixelCount: pixelCount,
            dataSize: dataSize,
            image: vec![0; dataSize],
            z_buffer: Vec::with_capacity(width),
        }
    }
}
