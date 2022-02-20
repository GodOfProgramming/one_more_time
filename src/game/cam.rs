use crate::{math::*, util::Settings};

#[derive(Default)]
pub struct Camera {
  view: glm::Mat4,
  projection: glm::Mat4,
}

impl Camera {
  pub fn update(&mut self, settings: &Settings) {
    self.view = glm::look_at(
      &glm::vec3(0.0, 0.0, 10.0),
      &glm::vec3(0.0, 0.0, 0.0),
      &glm::vec3(0.0, 1.0, 0.0),
    );

    let width_2 = (settings.display.window.x / 2) as f32;
    let height_2 = (settings.display.window.y / 2) as f32;

    // self.projection = glm::ortho(-width_2, width_2, -height_2, height_2, 0.1, 100.0);
    self.projection = glm::perspective(16.0 / 9.0, 45_f32.to_radians(), 0.1, 100.0);
  }

  pub fn view(&self) -> [[f32; 4]; 4] {
    self.view.into()
  }

  pub fn projection(&self) -> [[f32; 4]; 4] {
    self.projection.into()
  }
}
