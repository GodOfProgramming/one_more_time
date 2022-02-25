use omt::{core::*, glm};

pub struct TestModel;

impl EntityModel for TestModel {
  fn new_instance(&self) -> Box<(dyn EntityInstance)> {
    Box::new(TestInstance::new())
  }

  fn shader(&self) -> std::option::Option<&'static str> {
    Some("basic")
  }

  fn sprite(&self) -> std::option::Option<&'static str> {
    Some("grass")
  }

  fn shape(&self) -> std::option::Option<&'static str> {
    Some("sprite")
  }
}

pub struct TestInstance {
  i: f32,
  transform: glm::Mat4,
}

impl TestInstance {
  fn new() -> Self {
    let transform = glm::scale(&glm::Mat4::identity(), &glm::vec3(5.0, 5.0, 1.0));
    Self { i: 0.0, transform }
  }
}

impl EntityInstance for TestInstance {
  fn update(&mut self, _handle: &mut dyn EntityHandle) {
    let transform = glm::rotate(
      &self.transform,
      self.i.to_radians(),
      &glm::vec3(0.0, 0.0, 1.0),
    );
    self.transform = transform;
  }

  fn should_update(&self) -> bool {
    todo!()
  }

  fn transform(&self) -> glm::Mat4 {
    self.transform.clone()
  }
}
