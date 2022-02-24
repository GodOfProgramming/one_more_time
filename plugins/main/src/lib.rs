use omt::{
  ui::{UiModel, UiModelInstance},
  util::settings::{Settings, Table},
  Plugin, PluginResult,
};

pub fn exports() -> PluginResult {
  Ok(Plugin)
}

struct TestUi;

impl TestUi {}

impl UiModel for TestUi {
  fn new_instance() -> Result<Box<dyn UiModelInstance>, ()> {
    Ok(TestUiInstance);
  }
}

struct TestUiInstance;

impl UiModelInstance for TestUiInstance {
  fn call_handler(&self, game: Game, name: &str) {
    match name {
      "show_or_hide_profiler" => {
        game.settings().modify("game", |game_settings: &mut Table| {
          game_settings.modify("show_or_hide_profiler", |value: &bool| {
            *value = !value;
          });
        });
      }
      "show_or_hide_demo_window" => {
        game.settings().modify("game", |game_settings: &mut Table| {
          game_settings.modify("show_or_hide_demo_window", |value: &bool| {
            *value = !value;
          });
        });
      }
    }
  }
}
