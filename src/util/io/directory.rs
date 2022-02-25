use crate::util::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ConfigDir {
  pub animations: PathBuf,
  pub entities: PathBuf,
  pub models: PathBuf,
  pub shaders: PathBuf,
  pub textures: PathBuf,
}

impl ConfigDir {
  fn new(dir: PathBuf) -> Self {
    Self {
      animations: dir.join("animations"),
      entities: dir.join("entities"),
      models: dir.join("models"),
      shaders: dir.join("shaders"),
      textures: dir.join("textures"),
    }
  }
}

#[derive(Debug, Clone)]
pub struct AssetsDir {
  pub cfg: ConfigDir,
  pub maps: PathBuf,
  pub shaders: PathBuf,
  pub textures: PathBuf,
  pub ui: PathBuf,
  pub scripts: PathBuf,
}

impl AssetsDir {
  fn new(dir: PathBuf) -> Self {
    Self {
      cfg: ConfigDir::new(dir.join("cfg")),
      maps: dir.join("maps"),
      shaders: dir.join("shaders"),
      textures: dir.join("textures"),
      ui: dir.join("ui"),
      scripts: dir.join("scripts"),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Dirs {
  pub root: PathBuf,
  pub assets: AssetsDir,
  pub config: PathBuf,
}

impl Dirs {
  pub fn new(root: PathBuf) -> Self {
    Self {
      assets: AssetsDir::new(root.join("assets")),
      config: root.join("config"),
      root,
    }
  }
}

impl AsPtr for Dirs {}
