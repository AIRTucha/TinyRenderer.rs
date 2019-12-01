use core::option::Option;
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Request, RequestInit, RequestMode, Response};

use web_sys::console;
/// A struct to hold some data from the github Branch API.
///
/// Note how we don't have to define every member -- serde will ignore extra
/// data when deserializing
#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub commit: CommitDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitDetails {
    pub author: Signature,
    pub committer: Signature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signature {
    pub name: String,
    pub email: String,
}

pub async fn get(url: &str) {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts).unwrap();

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json");

    let window = web_sys::window().unwrap();
    let result = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request));
    let r = result.await.unwrap();
    console::log_1(&r);
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
                v[1].parse::<f32>().unwrap(),
                v[2].parse::<f32>().unwrap(),
                v[3].parse::<f32>().unwrap(),
            )
        })
        .collect::<Vec<Vec3>>()
}

fn parse_vn(data: &Vec<Vec<&str>>) -> Vec<Vec3> {
    data.iter()
        .filter(|v| v[0] == "vn")
        .map(|v| {
            Vec3::new(
                v[2].parse::<f32>().unwrap(),
                v[3].parse::<f32>().unwrap(),
                v[4].parse::<f32>().unwrap(),
            )
        })
        .collect::<Vec<Vec3>>()
}
fn parse_vt(data: &Vec<Vec<&str>>) -> Vec<Vec2> {
    data.iter()
        .filter(|v| v[0] == "vt")
        .map(|v| Vec2::new(v[2].parse::<f32>().unwrap(), v[3].parse::<f32>().unwrap()))
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
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub textures: Vec<Vec2>,
    pub faces: Vec<(Indices, Indices, Indices)>,
}

impl Obj {
    pub async fn new(model_url: &str) -> Obj {
        let text_obj = get(model_url).await;
        let raw_obj = tokenize(text_obj.as_str());
        Obj {
            vertices: parse_v(&raw_obj),
            normals: parse_vn(&raw_obj),
            textures: parse_vt(&raw_obj),
            faces: parse_f(&raw_obj),
        }
    }
}
