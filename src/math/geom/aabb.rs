use super::prelude::*;

pub struct AABB {
  left: f32,
  bottom: f32,
  right: f32,
  top: f32,
}

impl AABB {
  pub fn contains(&self, x: f32, y: f32) -> bool {
    x >= self.left && x <= self.right && y >= self.bottom && y <= self.top
  }
}
