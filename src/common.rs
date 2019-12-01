#[macro_export]
macro_rules! sq {
    ( $x:expr ) => {
        $x * $x
    };
}

#[macro_export]
macro_rules! interpolate {
    ( $min:expr, $max:expr, $gradient:expr ) => {
        $min + ($max - $min) * $gradient.clamp(0.0, 1.0)
    };
}

#[macro_export]
macro_rules! cross_product {
    ( $vec1:expr, $vec2:expr ) => {
        Vec3::new(
            $vec1.y * $vec2.z - $vec2.y * $vec1.z,
            -($vec1.x * $vec2.z - $vec2.x * $vec1.z),
            $vec1.x * $vec2.y - $vec1.y * $vec2.x,
        )
    };
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x: x, y: y }
    }
}

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
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

pub fn dot_product(vec1: Vec3, vec2: Vec3) -> f32 {
    vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z
}

pub fn normalize(vec: Vec3) -> Vec3 {
    let length = (sq!(vec.x) + sq!(vec.y) + sq!(vec.z)).sqrt();
    Vec3::new(vec.x / length, vec.y / length, vec.z / length)
}

pub fn cross_product(vec1: Vec3, vec2: Vec3, vec3: Vec3, vec4: Vec3) -> Vec3 {
    let vec12 = Vec3::new(vec1.x - vec2.x, vec1.y - vec2.y, vec1.z - vec2.z);
    let vec34 = Vec3::new(vec3.x - vec4.x, vec3.y - vec4.y, vec3.z - vec4.z);
    cross_product!(vec12, vec34)
}
