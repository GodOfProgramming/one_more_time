use crate::view::window::WindowMode;
use toml::{value::Table, Value};

mod keys {
  pub const TITLE: &str = "title";
  pub const WIDTH: &str = "width";
  pub const HEIGHT: &str = "height";
  pub const MODE: &str = "video_mode";
}

pub struct DisplaySettings {
  pub title: String,
  pub width: u32,
  pub height: u32,
  pub mode: WindowMode,
}

impl DisplaySettings {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Default for DisplaySettings {
  fn default() -> Self {
    Self {
      title: String::from("game"),
      width: 720,
      height: 1280,
      mode: WindowMode::Windowed,
    }
  }
}

impl From<&Table> for DisplaySettings {
  fn from(table: &Table) -> Self {
    let mut settings = Self::new();

    if let Some(Value::String(title)) = table.get(keys::TITLE) {
      settings.title = title.clone();
    }

    if let Some(Value::Integer(width)) = table.get(keys::WIDTH) {
      settings.width = (*width).try_into().unwrap_or(720);
    }

    if let Some(Value::Integer(height)) = table.get(keys::HEIGHT) {
      settings.height = (*height).try_into().unwrap_or(1920);
    }

    if let Some(Value::String(video_mode)) = table.get(keys::MODE) {
      settings.mode = WindowMode::from(video_mode);
    }

    settings
  }
}

impl From<Table> for DisplaySettings {
  fn from(table: Table) -> Self {
    Self::from(&table)
  }
}

impl Into<Table> for DisplaySettings {
  fn into(self) -> Table {
    let mut table = Table::new();

    table.insert(String::from(keys::TITLE), Value::String(self.title.clone()));

    table.insert(
      String::from(keys::WIDTH),
      Value::Integer(self.width.try_into().unwrap_or(1080)),
    );

    table.insert(
      String::from(keys::HEIGHT),
      Value::Integer(self.height.try_into().unwrap_or(1920)),
    );

    table.insert(
      String::from(keys::MODE),
      Value::String(self.mode.to_string()),
    );

    table
  }
}
