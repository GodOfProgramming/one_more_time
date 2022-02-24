fn initialize_lua(
  &mut self,
  dirs: &Dirs,
  settings: &mut Settings,
  ui_manager: &mut UiManager,
) -> &'static Lua {
  let lua = Lua::new();

  {
    let globals = lua.globals();

    let dirs_ptr = dirs.as_ptr();

    let package: LuaTable = globals.get("package").unwrap();
    let path: String = package.get("path").unwrap();
    let path = format!(
      "{}/?.lua;{}",
      dirs_ptr.assets.scripts.to_str().unwrap(),
      path
    );
    package.set("path", path).unwrap();

    let logger_ptr = self.logger.as_ptr();
    let app_ptr = self.as_ptr_mut();
    let settings_ptr = settings.as_ptr_mut();
    let ui_manager_ptr = ui_manager.as_ptr_mut();
    let _ = globals.set("App", app_ptr);
    let _ = globals.set("Logger", logger_ptr);
    let _ = globals.set("Settings", settings_ptr);
    let _ = globals.set("UiManager", ui_manager_ptr);

    for (file, id) in RecursiveDirIteratorWithID::from(&dirs.assets.scripts) {
      if settings
        .scripts
        .exclude
        .iter()
        .any(|reg: &Regex| reg.is_match(&id))
      {
        continue;
      }

      if let Ok(src) = fs::read_to_string(file) {
        lua.load(&src).exec().unwrap();
      }
    }
  }
  lua.into_static()
}
