use crate::matrix::Matrix4x4;
use std::ops::Mul;

#[macro_export]
macro_rules! sq {
    ( $x:expr ) => {
        $x * $x
    };
}

pub fn clamp(grad: f64, min: f64, max: f64) -> f64 {
    assert!(min <= max);
    let mut x = grad;
    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
    x
}

pub fn interpolate(min: f64, max: f64, gradient: f64) -> f64 {
    min + (max - min) * clamp(gradient, 0.0, 1.0)
}

pub fn cross_product_vec3(vec1: &Vec3, vec2: &Vec3) -> Vec3 {
    Vec3::new(
        vec1.y * vec2.z - vec2.y * vec1.z,
        -(vec1.x * vec2.z - vec2.x * vec1.z),
        vec1.x * vec2.y - vec1.y * vec2.x,
    )
}

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x: x, y: y }
    }
}

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }
}

impl Mul<&Matrix4x4> for Vec3 {
    type Output = Vec3;

    fn mul(self, matrix: &Matrix4x4) -> Vec3 {
        let Matrix4x4(value) = matrix;
        Vec3::new(
            self.x * value[0][0] + self.y * value[0][1] + self.z * value[0][2] + value[0][3],
            self.x * value[1][0] + self.y * value[1][1] + self.z * value[1][2] + value[1][3],
            self.x * value[2][0] + self.y * value[2][1] + self.z * value[2][2] + value[2][3],
        )
    }
}

pub struct Indices {
    pub vertex: usize,
    pub texture: usize,
    pub normal: usize,
}

impl Indices {
    pub fn new(vertex: usize, texture: usize, normal: usize) -> Indices {
        Indices {
            vertex: vertex,
            texture: texture,
            normal: normal,
        }
    }
}
pub struct Vertex {
    pub vertex: Vec3,
    pub normal: Vec3,
    pub texture: Vec2,
}

impl Vertex {
    pub fn new(vertex: Vec3, normal: Vec3, texture: Vec2) -> Vertex {
        Vertex {
            vertex: vertex,
            normal: normal,
            texture: texture,
        }
    }
}

pub fn dot_product(vec1: &Vec3, vec2: &Vec3) -> f64 {
    vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z
}

pub fn normalize(vec: &Vec3) -> Vec3 {
    let length = (sq!(vec.x) + sq!(vec.y) + sq!(vec.z)).sqrt();
    Vec3::new(vec.x / length, vec.y / length, vec.z / length)
}
