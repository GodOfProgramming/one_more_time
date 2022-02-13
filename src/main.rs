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
  let logger = MainLogger::new(LOG_LIMIT);

  let (sender, receiver) = mpsc::channel();

  let mut app = LuaType::<App>::new(logger.spawn(), sender.clone(), receiver);

  let cwd = env::current_dir().unwrap(); // unwrap because there's bigger problems if this doesn't work
  let dirs = Dirs::new(cwd);
  let settings_file = Path::new(SETTINGS_FILE);
  let settings = Settings::load(settings_file).unwrap();

  let mut input_devices = InputDevices::default();

  let mut script_repo =
    ScriptRepository::new(&logger, RecursiveDirIteratorWithID::from(&dirs.assets.scripts));

  // set up some top level lua functions
  {
    let moved_sender = sender.clone();
    script_repo.register_init_fn(Box::new(move |lua: &mut Lua| {
      let globals = lua.globals();
      let core_table: mlua::Table = lua.create_table().unwrap();

      let msg_sender = moved_sender.clone();
      let send_message = lua
        .create_function(move |_lua, msg: String| {
          let _ = msg_sender.send(msg);
          Ok(())
        })
        .unwrap();
      let _ = core_table.set("send_message", send_message);

      let _ = globals.set("App", core_table);
      let _ = globals.set("Logger", LuaType::<ChildLogger>::from_type(logger.spawn()));
    }));
  }

  app.run(&settings, &dirs, &mut input_devices, &mut script_repo);

  settings.save(settings_file).unwrap();
}
