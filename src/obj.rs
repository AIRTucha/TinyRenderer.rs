use crate::common::{Indices, Vec2, Vec3, Vertex};
use crate::get::get;
use crate::texture::Texture;

pub fn tokenize(text: &str) -> Vec<Vec<&str>> {
    text.split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
}

fn parse_face(v: Vec<&str>) -> Indices {
    Indices::new(
        v[0].parse::<usize>().unwrap() - 1,
        v[1].parse::<usize>().unwrap() - 1,
        v[2].parse::<usize>().unwrap() - 1,
    )
}

fn parse_v(data: &Vec<Vec<&str>>) -> Vec<Vec3> {
    data.iter()
        .filter(|v| v[0] == "v")
        .map(|v| {
            Vec3::new(
                v[1].parse::<f64>().unwrap(),
                v[2].parse::<f64>().unwrap(),
                v[3].parse::<f64>().unwrap(),
            )
        })
        .collect::<Vec<Vec3>>()
}

fn parse_vn(data: &Vec<Vec<&str>>) -> Vec<Vec3> {
    data.iter()
        .filter(|v| v[0] == "vn")
        .map(|v| {
            Vec3::new(
                v[2].parse::<f64>().unwrap(),
                v[3].parse::<f64>().unwrap(),
                v[4].parse::<f64>().unwrap(),
            )
        })
        .collect::<Vec<Vec3>>()
}
fn parse_vt(data: &Vec<Vec<&str>>) -> Vec<Vec2> {
    data.iter()
        .filter(|v| v[0] == "vt")
        .map(|v| Vec2::new(v[2].parse::<f64>().unwrap(), v[3].parse::<f64>().unwrap()))
        .collect::<Vec<Vec2>>()
}

fn parse_f(data: &Vec<Vec<&str>>) -> Vec<(Indices, Indices, Indices)> {
    data.iter()
        .filter(|v| v[0] == "f")
        .map(|value| {
            (
                parse_face(value[1].split("/").collect::<Vec<&str>>()),
                parse_face(value[2].split("/").collect::<Vec<&str>>()),
                parse_face(value[3].split("/").collect::<Vec<&str>>()),
            )
        })
        .collect::<Vec<(Indices, Indices, Indices)>>()
}

pub struct Obj {
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    textures: Vec<Vec2>,
    faces: Vec<(Indices, Indices, Indices)>,
    pub defuse: Texture,
    pub normals_tex: Texture,
    pub specular: Texture,
}

impl Obj {
    pub async fn new(model_url: &str, diffuse_url: &str, nm_url: &str, spec_url: &str) -> Obj {
        let text_obj = get(model_url).await;
        let raw_obj = tokenize(text_obj.as_str());
        Obj {
            vertices: parse_v(&raw_obj),
            normals: parse_vn(&raw_obj),
            textures: parse_vt(&raw_obj),
            faces: parse_f(&raw_obj),
            defuse: Texture::new(diffuse_url).await,
            normals_tex: Texture::new(nm_url).await,
            specular: Texture::new(spec_url).await,
        }
    }
    pub fn faces(&self) -> Vec<(Vertex, Vertex, Vertex)> {
        self.faces
            .iter()
            .map(|(fst, snd, trd)| {
                (
                    Vertex::new(
                        self.vertices[fst.vertex],
                        self.normals[fst.normal],
                        self.textures[fst.texture],
                    ),
                    Vertex::new(
                        self.vertices[snd.vertex],
                        self.normals[snd.normal],
                        self.textures[snd.texture],
                    ),
                    Vertex::new(
                        self.vertices[trd.vertex],
                        self.normals[trd.normal],
                        self.textures[trd.texture],
                    ),
                )
            })
            .collect()
    }
}
