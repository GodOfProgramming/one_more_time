pub mod display;
pub mod game;
pub mod graphics;

use common::*;
use std::{fs, path::Path};
use toml::{value::Table, Value};

mod common {
  pub use crate::scripting::{LuaType, LuaTypeTrait};
  pub use mlua::{Lua, UserData, UserDataFields, UserDataMethods};
  pub use std::cell::Cell;
  pub use toml::{value::Table, Value};
}

mod keys {
  pub const DISPLAY: &str = "display";
  pub const GRAPHICS: &str = "graphics";
  pub const GAME: &str = "game";
}

#[derive(Default, Debug)]
pub struct Settings {
  pub display: display::Settings,
  pub graphics: graphics::Settings,
  pub game: game::Settings,
}

impl Settings {
  pub fn load(p: &Path) -> Result<Settings, String> {
    match fs::read_to_string(p) {
      Ok(data) => match data.parse::<Value>() {
        Ok(root) => {
          let mut settings = Settings::default();

          if let Some(Value::Table(display)) = root.get("display") {
            settings.display = display::Settings::from(display);
          }

          if let Some(Value::Table(graphics)) = root.get("graphics") {
            settings.graphics = graphics::Settings::from(graphics);
          }

          if let Some(Value::Table(game)) = root.get("game") {
            settings.game = game::Settings::from(game);
          }

          Ok(settings)
        }
        Err(e) => Err(format!("could not parse settings '{}'", e)),
      },
      Err(e) => Err(format!("could not read settings at '{:?}': {}", p, e)),
    }
  }

  pub fn save(self, p: &Path) -> Result<(), String> {
    let mut root = Table::new();

    root.insert(
      String::from(keys::DISPLAY),
      Value::Table(self.display.into()),
    );

    root.insert(
      String::from(keys::GRAPHICS),
      Value::Table(self.graphics.into()),
    );

    root.insert(String::from(keys::GAME), Value::Table(self.game.into()));

    match toml::to_string_pretty(&root) {
      Ok(data) => {
        fs::write(p, data).map_err(|e| format!("could not write settings to '{:?}': {}", p, e))
      }
      Err(e) => Err(format!("could not read settings at '{:?}': {}", p, e)),
    }
  }
}

impl LuaType<Settings> {
  fn display(&mut self) -> LuaType<display::Settings> {
    self.obj_mut().display.create_lua_type()
  }

  fn graphics(&mut self) -> LuaType<graphics::Settings> {
    self.obj_mut().graphics.create_lua_type()
  }

  fn game(&mut self) -> LuaType<game::Settings> {
    self.obj_mut().game.create_lua_type()
  }
}

impl LuaTypeTrait for Settings {}

impl UserData for LuaType<Settings> {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method_mut("display", |_, this, _: ()| Ok(this.display()));
    methods.add_method_mut("graphics", |_, this, _: ()| Ok(this.graphics()));
    methods.add_method_mut("game", |_, this, _: ()| Ok(this.game()));
  }
}
