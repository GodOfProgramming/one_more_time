use crate::{
  core::EntityModelLoader,
  gfx::{ShaderLoader, TextureLoader},
  ui::{UiModelLoader, UiSourceLoader},
  util::Logger,
};
use std::path::PathBuf;

pub use image;
pub use nalgebra_glm as glm;
pub use ncollide2d;
pub use toml;

pub mod core;
pub mod gfx;
pub mod ui;
pub mod util;

pub trait Plugin {
  fn path(&self) -> PathBuf;
  fn logger(&self) -> &dyn Logger;
  fn textures(&mut self) -> &mut dyn TextureLoader;
  fn shaders(&mut self) -> &mut dyn ShaderLoader;
  fn ui_models(&mut self) -> &mut dyn UiModelLoader;
  fn ui_sources(&mut self) -> &mut dyn UiSourceLoader;
  fn entity_models(&mut self) -> &mut dyn EntityModelLoader;
}

pub enum PluginLoadError {
  GeneralFailure(String),
}

pub type PluginResult = Result<(), PluginLoadError>;

pub type PluginLoadFn = unsafe extern "C" fn(*mut dyn Plugin) -> PluginResult;
