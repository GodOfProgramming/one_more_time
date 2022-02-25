use omt::{
  core::*,
  toml::Value,
  ui::{UiModel, UiModelError, UiModelInstance},
  util::*,
  Plugin, PluginResult,
};

use directory_iter::RecursiveDirIteratorWithID;

mod directory_iter;

pub fn exports(plugin: &mut dyn Plugin) -> PluginResult {
  // textures
  {
    let iter = RecursiveDirIteratorWithID::from(plugin.path().join("tex"));
    load_textures(plugin);
  }

  // shaders
  {
    load_shaders(plugin);
  }

  Ok(())
}

struct DebugMainMenu;

impl UiModel for DebugMainMenu {
  fn tag_name(&self) -> &'static str {
    "debug-main-menu"
  }

  fn new_instance(&self) -> Result<Box<dyn UiModelInstance>, UiModelError> {
    Ok(Box::new(DebugMainMenu))
  }
}

impl UiModelInstance for DebugMainMenu {
  fn call_handler(&self, name: &str, game: &mut dyn Game) {
    match name {
      "show_or_hide_profiler" => {
        game
          .settings()
          .modify("game", &|game_settings: &mut Value| {
            if let Value::Table(game_settings) = game_settings {
              if let Some(Value::Boolean(show_or_hide_profiler)) =
                game_settings.get_mut("show_or_hide_profiler")
              {
                *show_or_hide_profiler = !*show_or_hide_profiler;
              }
            }
          });
      }
      "show_or_hide_demo_window" => {
        game
          .settings()
          .modify("game", &|game_settings: &mut Value| {
            if let Value::Table(game_settings) = game_settings {
              if let Some(Value::Boolean(show_or_hide_demo_window)) =
                game_settings.get_mut("show_or_hide_demo_window")
              {
                *show_or_hide_demo_window = !*show_or_hide_demo_window;
              }
            }
          });
      }
      _ => (),
    }
  }
}

pub fn load_textures(plugin: &mut dyn Plugin, iter: RecursiveDirIteratorWithID) -> PluginResult {
  plugin.logger().info("loading textures".to_string());
  for (path, id) in iter {
    plugin.logger().info(format!("loading {}", id));
    match Reader::open(&path) {
      Ok(reader) => match reader.decode() {
        Ok(image) => {
          plugin.textures().register(id.to_string(), image);
        }
        Err(err) => plugin
          .logger()
          .error(format!("error decoding '{:?}': {}", path, err)),
      },
      Err(err) => plugin
        .logger()
        .error(format!("error reading '{:?}': {}", path, err)),
    }
  }
  Ok(())
}

pub fn load_shaders(plugin: &mut dyn Plugin) -> PluginResult {
  Ok(())
}
