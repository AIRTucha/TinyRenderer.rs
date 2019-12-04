#[macro_use]
use crate::common::*;

use crate::engine::Scene;
use crate::obj::Obj;

fn is_not_back_faced(vert1: &Vertex, vert2: &Vertex, vert3: &Vertex) -> bool {
  dot_product(&Vec3::new(0.0, 0.0, 1.0), &vert1.normal) > 0.0
    || dot_product(&Vec3::new(0.0, 0.0, 1.0), &vert2.normal) > 0.0
    || dot_product(&Vec3::new(0.0, 0.0, 1.0), &vert3.normal) > 0.0
}

pub trait Pipeline {
  fn vertex_shader<'a>(&self, vert: Vertex, scene: &Scene) -> Vertex;
  fn pixel_shader(&self, x: f64, y: f64, obj: &Obj) -> (u8, u8, u8, u8);
  fn draw(&mut self, obj: &Obj, scene: &mut Scene) {
    for (v1, v2, v3) in obj.faces() {
      self.triangle(v1, v2, v3, obj, scene)
    }
  }
  fn line(
    &self,
    y: usize,
    vec1: &Vertex,
    vec2: &Vertex,
    vec3: &Vertex,
    vec4: &Vertex,
    obj: &Obj,
    scene: &mut Scene,
  ) {
    let gradientY12 = if vec1.vertex.y != vec2.vertex.y {
      (f64::from(y as u32) - vec1.vertex.y) / (vec2.vertex.y - vec1.vertex.y)
    } else {
      1.0
    };
    let gradientY34 = if vec3.vertex.y != vec4.vertex.y {
      (f64::from(y as u32) - vec3.vertex.y) / (vec4.vertex.y - vec3.vertex.y)
    } else {
      1.0
    };

    let startX = interpolate(vec1.vertex.x, vec2.vertex.x, gradientY12).floor() as usize;
    let endX = interpolate(vec3.vertex.x, vec4.vertex.x, gradientY34).floor() as usize;
    let startZ = interpolate(vec1.vertex.z, vec2.vertex.z, gradientY12);
    let endZ = interpolate(vec3.vertex.z, vec4.vertex.z, gradientY34);
    let startXTex = interpolate(vec1.texture.x, vec2.texture.x, gradientY12);
    let endXTex = interpolate(vec3.texture.x, vec4.texture.x, gradientY34);
    let startYTex = interpolate(vec1.texture.y, vec2.texture.y, gradientY12);
    let endYTex = interpolate(vec3.texture.y, vec4.texture.y, gradientY34);

    for x in startX..endX {
      let gradientX: f64 = (x - startX) as f64 / (endX - startX) as f64;

      let z = interpolate(startZ, endZ, gradientX);
      let xTex = interpolate(startXTex, endXTex, gradientX);
      let yTex = interpolate(startYTex, endYTex, gradientX);
      let (r, g, b, a) = self.pixel_shader(xTex, yTex, obj);

      scene.dot(x, y, z, r, g, b, a);
    }
  }

  fn triangle(&mut self, vec1: Vertex, vec2: Vertex, vec3: Vertex, obj: &Obj, scene: &mut Scene) {
    if is_not_back_faced(&vec1, &vec2, &vec3) {
      self.rasterize(
        &self.vertex_shader(vec1, scene),
        &self.vertex_shader(vec2, scene),
        &self.vertex_shader(vec3, scene),
        obj,
        scene,
      )
    }
  }

  fn rasterize(
    &mut self,
    vert1: &Vertex,
    vert2: &Vertex,
    vert3: &Vertex,
    obj: &Obj,
    scene: &mut Scene,
  ) {
    if vert1.vertex.y > vert2.vertex.y {
      self.rasterize(vert2, vert1, vert3, obj, scene);
    } else if vert2.vertex.y > vert3.vertex.y {
      self.rasterize(vert1, vert3, vert2, obj, scene);
    } else {
      let d1 = if vert2.vertex.y - vert1.vertex.y > 0.0 {
        (vert2.vertex.x - vert1.vertex.x) / (vert2.vertex.y - vert1.vertex.y)
      } else {
        0.0
      };
      let d2 = if vert3.vertex.y - vert1.vertex.y > 0.0 {
        (vert3.vertex.x - vert1.vertex.x) / (vert3.vertex.y - vert1.vertex.y)
      } else {
        0.0
      };

      if d1 > d2 {
        for y in (vert1.vertex.y as usize)..(1 + vert2.vertex.y as usize) {
          self.line(y, vert1, vert3, vert1, vert2, obj, scene);
        }
        for y in (vert2.vertex.y as usize)..(vert3.vertex.y as usize) {
          self.line(y, vert3, vert1, vert3, vert2, obj, scene);
        }
      } else {
        for y in (vert1.vertex.y as usize)..(1 + vert2.vertex.y as usize) {
          self.line(y, vert1, vert2, vert1, vert3, obj, scene);
        }
        for y in (vert2.vertex.y as usize)..(vert3.vertex.y as usize) {
          self.line(y, vert3, vert2, vert3, vert1, obj, scene);
        }
      }
    }
  }
}

pub struct Renderer {}

impl Pipeline for Renderer {
  fn vertex_shader(&self, vert: Vertex, scene: &Scene) -> Vertex {
    let Vertex {
      vertex,
      normal,
      texture,
    } = vert;
    Vertex::new(scene.scale(&vertex), normal, texture)
  }
  fn pixel_shader(&self, x: f64, y: f64, obj: &Obj) -> (u8, u8, u8, u8) {
    let light = Vec3::new(0.65, 0.65, -0.15);
    let normal = normalize(&obj.normals_tex.getVec3(x, y));
    let specular_pow = obj.specular.get_color(x, y);
    let r_plus_l = cross_product_vec3(
      &normal,
      &cross_product_vec3(
        &normal,
        &Vec3::new(-light.x * 2.0, -light.y * 2.0, light.z * 2.0),
      ),
    );
    let r = normalize(&Vec3::new(
      r_plus_l.x - light.x,
      r_plus_l.y - light.y,
      r_plus_l.z - light.z,
    ));
    let color = obj.defuse.get_color(x, y);
    let spec = dot_product(&r, &Vec3::new(0.0, 0.0, 1.0));
    let defuse_intensity = dot_product(&light, &normal);
    (
      (color.r as f64
        * (defuse_intensity + 0.3 * spec.powi(specular_pow.r as i32).abs())
          .min(1.0)
          .min(1.0)) as u8,
      (color.g as f64
        * (defuse_intensity + 0.3 * spec.powi(specular_pow.g as i32).abs())
          .min(1.0)
          .min(1.0)) as u8,
      (color.b as f64 * (defuse_intensity + 0.3 * spec.powi(specular_pow.b as i32).abs()).min(1.0))
        as u8,
      color.a,
    )
  }
}
