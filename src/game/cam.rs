use crate::{math::*, util::Settings};

#[derive(Default)]
pub struct Camera {
  view: glm::Mat4,
  projection: glm::Mat4,
}

impl Camera {
  pub fn update(&mut self, settings: &Settings) {
    self.view = glm::look_at(
      &glm::vec3(0.0, 0.0, -1.0),
      &glm::vec3(0.0, 0.0, 0.0),
      &glm::vec3(0.0, 0.0, 1.0),
    );

    self.projection = glm::perspective(16.0 / 9.0, 45_f32.to_radians(), 0.1, 100.0);
  }

  pub fn view(&self) -> [[f32; 4]; 4] {
    self.view.into()
  }

  pub fn projection(&self) -> [[f32; 4]; 4] {
    self.projection.into()
  }
}
