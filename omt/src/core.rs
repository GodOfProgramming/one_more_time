use crate::util::*;

pub trait Game {
  fn settings(&mut self) -> &mut dyn Settings;
  fn logger(&self) -> &dyn Logger;
}

pub trait EntityModelLoader {
  fn register(&mut self, name: String, model: Box<dyn EntityModel>);
}

pub trait EntityModel {
  fn new_instance(&self) -> Box<dyn EntityInstance>;
  fn shader(&self) -> Option<&'static str>;
  fn sprite(&self) -> Option<&'static str>;
  fn shape(&self) -> Option<&'static str>;
}

pub trait EntityInstance {
  fn update(&mut self, handle: &mut dyn EntityHandle);
  fn should_update(&self) -> bool;
}

pub trait EntityHandle {
  fn dispose(&mut self);
}

pub struct InvisibleEntity;

impl EntityModel for InvisibleEntity {
  fn new_instance(&self) -> Box<dyn EntityInstance> {
    Box::new(InvisibleEntity)
  }

  fn shader(&self) -> Option<&'static str> {
    None
  }

  fn sprite(&self) -> Option<&'static str> {
    None
  }

  fn shape(&self) -> Option<&'static str> {
    None
  }
}

impl EntityInstance for InvisibleEntity {
  fn update(&mut self, _handle: &mut dyn EntityHandle) {}

  fn should_update(&self) -> bool {
    false
  }
}
