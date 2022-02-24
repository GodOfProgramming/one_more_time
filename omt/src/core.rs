use crate::util::*;
use std::path::PathBuf;

pub trait Game {
  fn settings(&mut self) -> &mut dyn Settings;
  fn logger(&self) -> &dyn Logger;
}

pub struct ShaderSource {
  pub vertex: PathBuf,
  pub fragment: PathBuf,
}

pub trait ShaderLoader {
  fn register(&mut self, id: &str, src: ShaderSource);
}

pub trait EntityModel {
  fn new_instance(&self) -> Box<dyn EntityInstance>;
}

pub trait EntityInstance {
  fn update(&mut self, handle: &mut dyn EntityHandle);
  fn should_update(&self) -> bool;
}

pub trait EntityHandle {
  fn dispose(&mut self);
}

pub struct StaticEntity;

impl EntityModel for StaticEntity {
  fn new_instance(&self) -> Box<dyn EntityInstance> {
    Box::new(StaticEntity)
  }
}

impl EntityInstance for StaticEntity {
  fn update(&mut self, _handle: &mut dyn EntityHandle) {}

  fn should_update(&self) -> bool {
    false
  }
}
