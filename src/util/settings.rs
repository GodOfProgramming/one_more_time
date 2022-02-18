mod display;
mod graphics;

pub use display::DisplaySettings;
pub use graphics::GraphicsSettings;
use std::{fs, path::Path};
use toml::{value::Table, Value};

mod keys {
  pub const DISPLAY: &str = "display";
  pub const GRAPHICS: &str = "graphics";
}

#[derive(Default, Debug)]
pub struct Settings {
  pub display: DisplaySettings,
  pub graphics: GraphicsSettings,
}

impl Settings {
  pub fn load(p: &Path) -> Result<Settings, String> {
    match fs::read_to_string(p) {
      Ok(data) => match data.parse::<Value>() {
        Ok(root) => {
          let mut settings = Settings::default();

          if let Some(Value::Table(display)) = root.get("display") {
            settings.display = DisplaySettings::from(display);
          }

          if let Some(Value::Table(graphics)) = root.get("graphics") {
            settings.graphics = GraphicsSettings::from(graphics);
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

    match toml::to_string_pretty(&root) {
      Ok(data) => {
        fs::write(p, data).map_err(|e| format!("could not write settings to '{:?}': {}", p, e))
      }
      Err(e) => Err(format!("could not read settings at '{:?}': {}", p, e)),
    }
  }
}
