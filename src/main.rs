use crate::{
  input::InputDevices,
  scripting::prelude::*,
  util::{Dirs, MainLogger, RecursiveDirIteratorWithID, Settings, SpawnableLogger},
};
use game::App;
use std::{env, path::Path};

mod game;
mod gfx;
mod input;
mod math;
mod scripting;
mod ui;
mod util;
mod view;

static SETTINGS_FILE: &str = "settings.toml";
const LOG_LIMIT: usize = 5;

fn main() {
  puffin::set_scopes_on(cfg!(debug_assertions));

  let logger = MainLogger::new(LOG_LIMIT);

  let mut app = App::new(logger.spawn());

  let cwd = env::current_dir().unwrap(); // unwrap because there's bigger problems if this doesn't work
  let dirs = Dirs::new(cwd);
  let settings_file = Path::new(SETTINGS_FILE);
  let mut settings = Settings::load(settings_file).unwrap();

  let mut input_devices = InputDevices::default();

  let mut script_repo = ScriptRepository::new(
    &logger,
    RecursiveDirIteratorWithID::from(&dirs.assets.scripts),
  );

  // set up some top level lua functions

  {
    let logger_ptr = logger.as_ptr();
    let app_ptr = app.as_ptr_mut();
    let settings_ptr = settings.as_ptr_mut();
    script_repo.register_init_fn(Box::new(move |lua| {
      let globals = lua.globals();
      let _ = globals.set("App", app_ptr);
      let _ = globals.set("Logger", logger_ptr);
      let _ = globals.set("Settings", settings_ptr);
    }));
  }

  app.run(&mut settings, &dirs, &mut input_devices, &mut script_repo);

  settings.save(settings_file).unwrap();
}
