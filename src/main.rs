mod game;
mod gfx;
mod input;
mod math;
mod scripting;
mod ui;
mod util;
mod view;

use crate::{
  input::InputDevices,
  scripting::{LuaTypeTrait, ScriptRepository},
  util::{Dirs, Logger, MainLogger, RecursiveDirIteratorWithID, Settings, SpawnableLogger},
};
use game::App;
use std::{env, path::Path};

static SETTINGS_FILE: &str = "settings.toml";
const LOG_LIMIT: usize = 5;

fn main() {
  puffin::set_scopes_on(cfg!(debug_assertions));

  let mut logger = MainLogger::new(LOG_LIMIT);
  let lua_logger = logger.create_lua_type();

  let mut app = App::new(logger.spawn());
  let lua_app = app.create_lua_type();

  let cwd = env::current_dir().unwrap(); // unwrap because there's bigger problems if this doesn't work
  let dirs = Dirs::new(cwd);
  let settings_file = Path::new(SETTINGS_FILE);
  let mut settings = Settings::load(settings_file).unwrap();
  let lua_settings = settings.create_lua_type();

  logger.info(format!("Settings: {:#?}", settings));

  let mut input_devices = InputDevices::default();

  let mut script_repo = ScriptRepository::new(
    &logger,
    RecursiveDirIteratorWithID::from(&dirs.assets.scripts),
  );

  // set up some top level lua functions

  script_repo.register_init_fn(Box::new(move |lua| {
    let globals = lua.globals();
    let _ = globals.set("App", lua_app);
    let _ = globals.set("Logger", lua_logger);
    let _ = globals.set("Settings", lua_settings);
  }));

  app.run(&mut settings, &dirs, &mut input_devices, &mut script_repo);

  settings.save(settings_file).unwrap();
}
