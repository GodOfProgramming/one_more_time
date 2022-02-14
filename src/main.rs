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
  scripting::{LuaType, ScriptRepository},
  util::{ChildLogger, Dirs, MainLogger, RecursiveDirIteratorWithID, Settings, SpawnableLogger},
};
use game::App;
use mlua::Lua;
use std::{env, path::Path, sync::mpsc};

static SETTINGS_FILE: &str = "config/settings.toml";
const LOG_LIMIT: usize = 5;

fn main() {
  let mut logger = MainLogger::new(LOG_LIMIT);
  let lua_logger = LuaType::<MainLogger>::from_type(&mut logger);

  let (sender, receiver) = mpsc::channel();
  let mut app = App::new(logger.spawn(), sender, receiver);
  let lua_app = LuaType::<App>::from_type(&mut app);

  let cwd = env::current_dir().unwrap(); // unwrap because there's bigger problems if this doesn't work
  let dirs = Dirs::new(cwd);
  let settings_file = Path::new(SETTINGS_FILE);
  let settings = Settings::load(settings_file).unwrap();

  let mut input_devices = InputDevices::default();

  let mut script_repo = ScriptRepository::new(
    &logger,
    RecursiveDirIteratorWithID::from(&dirs.assets.scripts),
  );

  // set up some top level lua functions

  script_repo.register_init_fn(Box::new(move |lua: &mut Lua| {
    let globals = lua.globals();
    let _ = globals.set("App", lua_app.clone());
    let _ = globals.set("Logger", lua_logger.clone());
  }));

  app.run(&settings, &dirs, &mut input_devices, &mut script_repo);

  settings.save(settings_file).unwrap();
}
