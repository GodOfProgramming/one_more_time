use crate::math::glm;
use crate::view::window::WindowMode;
use toml::{value::Table, Value};

mod keys {
  pub const TITLE: &str = "title";
  pub const WIDTH: &str = "width";
  pub const HEIGHT: &str = "height";
  pub const MODE: &str = "video_mode";
  pub const WINDOW: &str = "window";
}

#[derive(Debug)]
pub struct DisplaySettings {
  pub title: String,
  pub window: glm::U32Vec2,
  pub mode: WindowMode,
  pub dimensions: glm::U32Vec2,
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
      window: glm::U32Vec2::new(1280, 720),
      mode: WindowMode::Windowed,
      dimensions: glm::U32Vec2::new(1280, 720),
    }
  }
}

impl From<&Table> for DisplaySettings {
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
