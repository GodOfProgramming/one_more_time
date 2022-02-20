use crate::math::glm::{Vec2, Vec3};
use imgui_glium_renderer::glium::implement_vertex;

pub type Vertices = Vec<Vertex>;
pub type Indices = Vec<u32>;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vertex {
  pub pos: [f32; 3],
  pub norm: [f32; 3],
  pub uv: [f32; 2],
}

impl Vertex {
  pub fn new(pos: Vec3, norm: Vec3, uv: Vec2) -> Self {
    Self {
      pos: [pos.x, pos.y, pos.z],
      norm: [norm.x, norm.y, norm.z],
      uv: [uv.x, uv.y],
    }
  }

  pub fn new_with_pos(pos: Vec3) -> Self {
    Self::new_with_pos_norm(pos, Default::default())
  }

  pub fn new_with_pos_norm(pos: Vec3, norm: Vec3) -> Self {
    Self::new(pos, norm, Default::default())
  }
}

implement_vertex!(Vertex, pos, norm, uv);

pub struct Triangle {
  pub vertices: [Vertex; 3],
  pub indices: [u32; 3],
}

impl Triangle {
  pub fn new() -> Self {
    let vertices = [
      Vertex::new_with_pos_norm(Vec3::new(-0.5, -0.5, 0.0), Vec3::new(1.0, 0.0, 0.0)),
      Vertex::new_with_pos_norm(Vec3::new(0.0, 0.5, 0.0), Vec3::new(0.0, 1.0, 0.0)),
      Vertex::new_with_pos_norm(Vec3::new(0.5, -0.5, 0.0), Vec3::new(0.0, 0.0, 1.0)),
    ];

    let indices = [0, 1, 2];

    Self { vertices, indices }
  }
}

pub struct Square {
  pub vertices: [Vertex; 4],
  pub indices: [u32; 6],
}

impl Square {
  pub fn new() -> Self {
    let vertices = [
      Vertex::new(
        Vec3::new(-0.5, -0.5, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec2::new(0.0, 0.0),
      ),
      Vertex::new(
        Vec3::new(-0.5, 0.5, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec2::new(0.0, 1.0),
      ),
      Vertex::new(
        Vec3::new(0.5, 0.5, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec2::new(1.0, 1.0),
      ),
      Vertex::new(
        Vec3::new(0.5, -0.5, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec2::new(1.0, 0.0),
      ),
    ];

    let indices = [0, 1, 2, 2, 3, 0];

    Self { vertices, indices }
  }
}

pub struct Sprite {
  pub vertices: Vec<Vertex>,
  pub indices: Vec<u32>,
}

impl Sprite {
  pub fn new() -> Self {
    let vertices = vec![
      Vertex::new(
        Vec3::new(-1.0, -1.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec2::new(0.0, 0.0),
      ),
      Vertex::new(
        Vec3::new(-1.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec2::new(0.0, 1.0),
      ),
      Vertex::new(
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec2::new(1.0, 1.0),
      ),
      Vertex::new(
        Vec3::new(1.0, -1.0, 0.0),
        Vec3::new(0.5, 0.3, 0.1),
        Vec2::new(1.0, 0.0),
      ),
    ];

    let indices = vec![0, 1, 2, 2, 3, 0];

    Self { vertices, indices }
  }
}
