use common::*;
use omt::toml::{value::Table, Value};
use std::{
  fs,
  path::{Path, PathBuf},
};

pub mod display;
pub mod game;
pub mod graphics;
pub mod scripts;

mod common {
  pub use crate::util::prelude::*;
  pub use omt::toml::{self, value::Table, Value};
  pub use std::cell::Cell;
}

mod keys {
  pub const DISPLAY: &str = "display";
  pub const GRAPHICS: &str = "graphics";
  pub const GAME: &str = "game";
  pub const SCRIPTS: &str = "scripts";
}

#[derive(Default, Debug)]
pub struct Settings {
  file: PathBuf,
  root: Table,
  pub display: display::Settings,
  pub graphics: graphics::Settings,
  pub game: game::Settings,
  pub scripts: scripts::Settings,
}

impl Settings {
  fn new(file: PathBuf) -> Self {
    Self {
      file,
      ..Default::default()
    }
  }

  pub fn load(p: &Path) -> Result<Settings, String> {
    match fs::read_to_string(p) {
      Ok(data) => match data.parse::<Value>() {
        Ok(root) => {
          let mut settings = Settings::new(p.to_path_buf());

          if let Some(Value::Table(display)) = root.get(keys::DISPLAY) {
            settings.display = display::Settings::from(display);
          }

          if let Some(Value::Table(graphics)) = root.get(keys::GRAPHICS) {
            settings.graphics = graphics::Settings::from(graphics);
          }

          if let Some(Value::Table(game)) = root.get(keys::GAME) {
            settings.game = game::Settings::from(game);
          }

          if let Some(Value::Table(scripts)) = root.get(keys::SCRIPTS) {
            settings.scripts = scripts::Settings::from(scripts);
          }

          Ok(settings)
        }
        Err(e) => Err(format!("could not parse settings '{}'", e)),
      },
      Err(e) => Err(format!("could not read settings at '{:?}': {}", p, e)),
    }
  }

  pub fn save(&self) -> Result<(), String> {
    let mut root = Table::new();

    root.insert(
      String::from(keys::DISPLAY),
      Value::Table((&self.display).into()),
    );

    root.insert(
      String::from(keys::GRAPHICS),
      Value::Table((&self.graphics).into()),
    );

    root.insert(String::from(keys::GAME), Value::Table((&self.game).into()));

    root.insert(
      String::from(keys::SCRIPTS),
      Value::Table((&self.scripts).into()),
    );

    match toml::to_string_pretty(&root) {
      Ok(data) => fs::write(&self.file, data)
        .map_err(|e| format!("could not write settings to '{:?}': {}", self.file, e)),
      Err(e) => Err(format!(
        "could not read settings at '{:?}': {}",
        self.file, e
      )),
    }
  }
}

impl omt::util::Settings for Settings {
  fn modify(&mut self, name: &str, f: Box<dyn for<'r> FnOnce(&'r mut omt::toml::Value)>) {
    if let Some(child) = self.root.get_mut(name) {
      f(child);
    }
  }
}

impl AsPtr for Settings {}
