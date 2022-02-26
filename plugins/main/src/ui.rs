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
          &["game", "show_profiler"],
          Box::new(|value: &mut Value| {
            if let Value::Boolean(show_or_hide_profiler) = value {
              *show_or_hide_profiler = !*show_or_hide_profiler;
            }
          }),
        );
      }
      "show_or_hide_demo_window" => {
        game.settings().modify(
          &["game", "show_demo_window"],
          Box::new(|value: &mut Value| {
            if let Value::Boolean(show_or_hide_demo_window) = value {
              *show_or_hide_demo_window = !*show_or_hide_demo_window;
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
