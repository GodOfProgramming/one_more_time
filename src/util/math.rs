pub fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
  x >= left && x <= right && y >= bottom && y <= top
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Vec2 {
  pub x: u32,
  pub y: u32,
}

impl Vec2 {
  pub fn new(x: u32, y: u32) -> Self {
    Self { x, y }
  }
}
