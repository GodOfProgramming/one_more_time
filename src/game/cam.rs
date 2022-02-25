use crate::{math::*, util::Settings};

#[derive(Default)]
pub struct Camera {
  location: glm::Vec3,
  target: glm::Vec3,
  projection: glm::Mat4,
}

impl Camera {
  pub fn new() -> Self {
    Self {
      location: glm::vec3(0.0, 0.0, 10.0),
      target: glm::vec3(0.0, 0.0, 0.0),
      ..Default::default()
    }
  }

  pub fn update(&mut self, settings: &Settings) {
    let width_2 = (settings.display.window.x / 2) as f32;
    let height_2 = (settings.display.window.y / 2) as f32;

    // self.projection = glm::ortho(-width_2, width_2, -height_2, height_2, 0.1, 100.0);
    self.projection = glm::perspective(16.0 / 9.0, 45_f32.to_radians(), 0.1, 100.0);
  }

  pub fn view(&self) -> [[f32; 4]; 4] {
    glm::look_at::<f32>(&self.location, &self.target, &glm::vec3(0.0, 1.0, 0.0)).into()
  }

  pub fn projection(&self) -> [[f32; 4]; 4] {
    self.projection.into()
  }

  pub fn move_up(&mut self) {
    self.location.y += 1.0;
  }

  pub fn move_down(&mut self) {
    self.location.y -= 1.0;
  }

  pub fn move_left(&mut self) {
    self.location.x -= 1.0;
  }

  pub fn move_right(&mut self) {
    self.location.x += 1.0;
  }
}
