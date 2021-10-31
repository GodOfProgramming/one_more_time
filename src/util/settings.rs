use crate::view::window::WindowMode;
use std::{fs, path::Path};
use toml::Value;

#[derive(Default)]
pub struct DisplaySettings {
  pub title: String,
  pub width: u32,
  pub height: u32,
  pub mode: WindowMode,
}

#[derive(Default)]
pub struct Settings {
  pub display: DisplaySettings,
}

impl Settings {
  pub fn load(p: &Path) -> Result<Settings, String> {
    if let Ok(data) = fs::read_to_string(p) {
      match data.parse::<Value>() {
        Ok(root) => {
          let settings = Settings::default();

          assert!(root.is_table());

          Ok(settings)
        }
        Err(msg) => Err(format!("could not parse settings '{}'", msg)),
      }
    } else {
      Err(format!("could not read settings '{:?}'", p))
    }
  }

  pub fn save(&self, p: &Path) -> Result<(), String> {
    Ok(())
  }
}
