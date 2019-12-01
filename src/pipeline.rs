use crate::common::*;

trait Pipeline {
  fn scene(&self) -> &mut Scene;
  fn obj(&self) -> &Obj;
  fn vertex_shader(vert: &Vertex): Vertex;
  fn pixel_shader(x: f32, y: f32, obj: &Obj ): ( Double, Double, Double, Double );
  pub fn draw(&mut self, obj: &Obj, scene: &mut Scene){
      obj.forEachPolygon(self.triangle(scene, obj)
  }
  fn line(
    &mut self,
    y: usize,
    vec1: &Vertex,
    vec2: &Vertex,
    vec3: &Vertex,
    vec4: &Vertex,
   ) {
    let gradientY12 = if vec1.vertex.y != vec2.vertex.y
      (y as f32 - vec1.vertex.y) / (vec2.vertex.y - vec1.vertex.y);
    else 
        1;
    let gradientY34 = if (vec3.vertex.y != vec4.vertex.y)
      (y as f32 - vec3.vertex.y) / (vec4.vertex.y - vec3.vertex.y);
    else 
        1;

    let startX       = interpolate!(vec1.vertex.x, vec2.vertex.x, gradientY12) as usize;
    let endX         = interpolate!(vec3.vertex.x, vec4.vertex.x, gradientY34) as usize;
    let startZ       = interpolate!(vec1.vertex.z, vec2.vertex.z, gradientY12);
    let endZ         = interpolate!(vec3.vertex.z, vec4.vertex.z, gradientY34);
    let startXTex    = interpolate!(vec1.texture.x, vec2.texture.x, gradientY12);
    let endXTex      = interpolate!(vec3.texture.x, vec4.texture.x, gradientY34);
    let startYTex    = interpolate!(vec1.texture.y, vec2.texture.y, gradientY12);
    let endYTex      = interpolate!(vec3.texture.y, vec4.texture.y, gradientY34);

    for x in startX..endX {
      let gradientX: Double = (x as f32 - startX) / (endX - startX);

      let z = interpolate!(startZ, endZ, gradientX);
      let xTex = interpolate!(startXTex, endXTex, gradientX);
      let yTex = interpolate!(startYTex, endYTex, gradientX);
      let ( r, g, b, a ) = self.pixelShader( xTex, yTex, obj );
    
      self.scene.dot( x, y, z, r, g, b, a )
    }
  }
  fn is_not_back_faced( vert1: &Vertex, vert2: &Vertex, vert3: &Vertex ) -> bool {
    dotProduct(Vec3::(0, 0, 1), vert1.normal) > 0 ||
    dotProduct(Vec3::(0, 0, 1), vert2.normal) > 0 ||
    dotProduct(Vec3::(0, 0, 1), vert3.normal) > 0
  }

  fn triangle(&self, scene: &Scene, obj: &Obj)(vec1: &Vertex, vec2: &Vertex, vec3: &Vertex){
    if(isNotBackface( vec1, vec2, vec3))
        self.rasterize(
            scene, 
            obj, 
            self.vertexShader(vec1, scene), 
            self.vertexShader(vec2, scene), 
            self.vertexShader(vec3, scene)
        )
  }

  fn rasterize(&mut self, vert1: &Vertex, vert2: &Vertex, vert3: &Vertex) {
    if (vert1.vertex.y > vert2.vertex.y)
      self.rasterize(scene, obj, vert2, vert1, vert3);
    else if(vert2.vertex.y > vert3.vertex.y)
      self.rasterize(scene, obj, vert1, vert3, vert2);
    else {
      let d1 =
        if (vert2.vertex.y - vert1.vertex.y > 0)
          (vert2.vertex.x - vert1.vertex.x) / (vert2.vertex.y - vert1.vertex.y);
        else 0
      let d2 =
        if (vert3.vertex.y - vert1.vertex.y > 0)
          (vert3.vertex.x - vert1.vertex.x) / (vert3.vertex.y - vert1.vertex.y);
        else 0

      if (d1 > d2) {
        for y in (vert1.vertex.y as usize)..(1 + vert2.vertex.y as usize) {
          self.line(y, vert1, vert3, vert1, vert2);
        }
        for y in (vert2.vertex.y as usize)..(vert3.vertex.y as usize) {
          self.line(y, vert3, vert1, vert3, vert2);
        }
      } else {
        for y in (vert1.vertex.y as usize)..( 1 + vert2.vertex.y as usize) {
          self.line(y, vert1, vert2, vert1, vert3)
        }
        for y in (vert2.vertex.y as usize)..(vert3.vertex.y as usize)
          self.line(y, vert3, vert2, vert3, vert1)
      }
    }
  }
}

pub struct Renderer {
    scene: &mut Scene,
    obj: &Obj
}

impl Pipeline for Renderer {
    fn scene(&self) -> &mut Scene {
        self.scene()
    }
    fn obj(&self) -> &Obj {
        self.obj()
    }
    fn vertexShader(&self, vert: &Vertex) -> Vertex {
        let Vertex( vertex: Vec3, normal: Vec3, texture: Vec2 ) = vert;
        Vertex::new(
            self.scene().scale(vertex),
            normal,
            texture
        )
    }
    fn pixelShader(x: f32, y: f32 ) {
        let light = Vec3::new(0.65, 0.65, -0.15);
        let normal = normalize(obj.normalsTex.getVec3(x, y));
        // let specularPow = obj.specular.getColor(x, y)
        // let rPlusL = crossProduct( normal, crossProduct( normal, Vec3(-light.x*2, -light.y*2, light.z*2) ))
        // let r = normalize(Vec3(rPlusL.x - light.x, rPlusL.y - light.y, rPlusL.z - light.z))
        // let color = obj.deffuse.getColor( x, y)
        // let spec = dotProduct(r, Vec3(0, 0, 1))
        let defuse_intensity = dotProduct( light, normal )
      (
        color.r * defuse_intensity/* + 0.3*abs(pow(spec, specularPow.r)*/.min(1),
        color.g * defuse_intensity/* + 0.3*abs(pow(spec, specularPow.g)*/.min(1),
        color.b * defuse_intensity/* + 0.3*abs(pow(spec, specularPow.b)*/.min(1),
        color.a
      )
    } 
  }