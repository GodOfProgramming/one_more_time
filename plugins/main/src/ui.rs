use omt::{core::Game, toml::Value, ui::*};

pub struct DebugMainMenu;

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
        game.settings().modify(
          "game",
          Box::new(|game_settings: &mut Value| {
            if let Value::Table(game_settings) = game_settings {
              if let Some(Value::Boolean(show_or_hide_profiler)) =
                game_settings.get_mut("show_or_hide_profiler")
              {
                *show_or_hide_profiler = !*show_or_hide_profiler;
              }
            }
          }),
        );
      }
      "show_or_hide_demo_window" => {
        game.settings().modify(
          "game",
          Box::new(|game_settings: &mut Value| {
            if let Value::Table(game_settings) = game_settings {
              if let Some(Value::Boolean(show_or_hide_demo_window)) =
                game_settings.get_mut("show_or_hide_demo_window")
              {
                *show_or_hide_demo_window = !*show_or_hide_demo_window;
              }
            }
          }),
        );
      }
      "on_exit" => {
        game.exit();
      }
      _ => (),
    }
  }
}
