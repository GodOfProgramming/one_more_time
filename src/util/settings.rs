use crate::view::window::WindowMode;
use std::{fs, path::Path};
use toml::{value::Table, Value};

#[derive(Default)]
pub struct DisplaySettings {
  pub title: String,
  pub width: u32,
  pub height: u32,
  pub mode: WindowMode,
}

impl From<&Table> for DisplaySettings {
  fn from(table: &Table) -> Self {
    let mut settings = Self::default();

    if let Some(Value::String(title)) = table.get("title") {
      settings.title = title.clone();
    }

    if let Some(Value::Integer(width)) = table.get("width") {
      settings.width = (*width).try_into().unwrap();
    }

    if let Some(Value::Integer(height)) = table.get("height") {
      settings.height = (*height).try_into().unwrap();
    }

    if let Some(Value::String(video_mode)) = table.get("video_mode") {
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

    table.insert(String::from("title"), Value::String(self.title.clone()));

    table.insert(
      String::from("width"),
      Value::Integer(self.width.try_into().unwrap()),
    );

    table.insert(
      String::from("height"),
      Value::Integer(self.height.try_into().unwrap()),
    );

    table.insert(String::from("mode"), Value::String(self.mode.to_string()));

    table
  }
}

#[derive(Default)]
pub struct Settings {
  pub display: DisplaySettings,
}

impl Settings {
  pub fn load(p: &Path) -> Result<Settings, String> {
    match fs::read_to_string(p) {
      Ok(data) => match data.parse::<Value>() {
        Ok(root) => {
          let mut settings = Settings::default();

          assert!(root.is_table());

          if let Some(Value::Table(display)) = root.get("display") {
            settings.display = DisplaySettings::from(display);
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

    root.insert(String::from("display"), Value::Table(self.display.into()));

    match toml::to_string_pretty(&root) {
      Ok(data) => {
        fs::write(p, data).map_err(|e| format!("could not write settings to '{:?}': {}", p, e))
      }
      Err(e) => Err(format!("could not read settings at '{:?}': {}", p, e)),
    }
  }
}
