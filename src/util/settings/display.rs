use crate::{math::Dim, util::prelude::*};
use omt::toml::{value::Table, Value};

mod keys {
  pub const TITLE: &str = "title";
  pub const WIDTH: &str = "width";
  pub const HEIGHT: &str = "height";
  pub const MODE: &str = "video_mode";
  pub const WINDOW: &str = "window";
}

#[derive(Debug, Clone)]
pub struct Settings {
  pub title: String,
  pub window: Dim<u32>,
  pub mode: String,
}

impl Settings {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      title: String::from("game"),
      window: Dim::new(1280, 720),
      mode: String::from("windowed"),
    }
  }
}

impl From<&Table> for Settings {
  fn from(table: &Table) -> Self {
    let mut settings = Self::new();

    if let Some(Value::String(title)) = table.get(keys::TITLE) {
      settings.title = title.clone();
    }

    if let Some(Value::Table(window)) = table.get(keys::WINDOW) {
      if let Some(Value::Integer(width)) = window.get(keys::WIDTH) {
        settings.window.x = (*width).try_into().unwrap_or(1280);
      }

      if let Some(Value::Integer(height)) = window.get(keys::HEIGHT) {
        settings.window.y = (*height).try_into().unwrap_or(720);
      }
    }

    if let Some(Value::String(mode)) = table.get(keys::MODE) {
      settings.mode = mode.clone();
    }

    settings
  }
}

impl From<Table> for Settings {
  fn from(table: Table) -> Self {
    Self::from(&table)
  }
}

impl Into<Table> for Settings {
  fn into(self) -> Table {
    let mut table = Table::new();

    table.insert(String::from(keys::TITLE), Value::String(self.title.clone()));

    {
      let mut window = Table::new();

      window.insert(
        String::from(keys::WIDTH),
        Value::Integer(self.window.x.try_into().unwrap_or(1080)),
      );

      window.insert(
        String::from(keys::HEIGHT),
        Value::Integer(self.window.y.try_into().unwrap_or(1920)),
      );

      table.insert(String::from(keys::WINDOW), Value::Table(window));
    }

    table.insert(
      String::from(keys::MODE),
      Value::String(self.mode.to_string()),
    );

    table
  }
}

impl AsPtr for Settings {}
