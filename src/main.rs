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
  util::{ChildLogger, Dirs, MainLogger, RecursiveDirIDIterator, Settings, SpawnableLogger},
};
use game::App;
use mlua::{prelude::*, UserData, UserDataFields};
use std::{env, path::Path};

static SETTINGS_FILE: &str = "config/settings.toml";
const LOG_LIMIT: usize = 5;

fn main() {
  let logger = MainLogger::new(LOG_LIMIT);

  let mut app = LuaType::<App>::new(logger.spawn());

  let cwd = env::current_dir().unwrap(); // unwrap because there's bigger problems if this doesn't work
  let dirs = Dirs::new(cwd);
  let settings_file = Path::new(SETTINGS_FILE);
  let settings = Settings::load(settings_file).unwrap();

  let mut input_devices = InputDevices::default();

  let script_repo = ScriptRepository::new(
    &logger,
    RecursiveDirIDIterator::from(&dirs.assets.scripts),
    |lua: &mut Lua| {
      let _ = lua.globals().set("App", app.clone());
      let _ = lua
        .globals()
        .set("Logger", LuaType::<ChildLogger>::from_type(logger.spawn()));
    },
  );

  app.run(&settings, &dirs, &mut input_devices, &script_repo);

  settings.save(settings_file).unwrap();
}
