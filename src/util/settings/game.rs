use super::common::*;

mod keys {
  pub const SHOW_DEMO_WINDOW: &str = "show_demo_window";
  pub const SHOW_PROFILER: &str = "show_profiler";
}

#[derive(Debug, Default)]
pub struct Settings {
  pub show_demo_window: bool,
  pub show_profiler: bool,
}

impl Settings {
  pub fn new() -> Self {
    Self::default()
  }
}

impl From<&Table> for Settings {
  fn from(table: &Table) -> Self {
    let mut settings = Self::new();

    if let Some(Value::Boolean(show_demo_window)) = table.get(keys::SHOW_DEMO_WINDOW).cloned() {
      settings.show_demo_window = show_demo_window;
    }

    if let Some(Value::Boolean(show_profiler)) = table.get(keys::SHOW_PROFILER).cloned() {
      settings.show_profiler = show_profiler;
    }

    settings
  }
}

impl From<Table> for Settings {
  fn from(table: Table) -> Self {
    Self::from(&table)
  }
}

impl Into<Table> for Settings {
  fn into(self) -> Table {
    let mut table = Table::new();

    table.insert(
      String::from(keys::SHOW_DEMO_WINDOW),
      Value::Boolean(self.show_demo_window),
    );

    table.insert(
      String::from(keys::SHOW_PROFILER),
      Value::Boolean(self.show_profiler),
    );

    table
  }
}

impl LuaType<Settings> {
  fn show_profiler(&self) -> bool {
    self.obj().show_profiler
  }

  fn set_show_profiler(&mut self, v: bool) {
    self.obj_mut().show_profiler = v;
  }

  fn invert_profiler_display(&mut self) {
    self.obj_mut().show_profiler = !self.obj().show_profiler;
  }

  fn show_demo_window(&self) -> bool {
    self.obj().show_demo_window
  }

  fn set_show_demo_window(&mut self, v: bool) {
    self.obj_mut().show_demo_window = v;
  }

  fn invert_demo_window_display(&mut self) {
    self.obj_mut().show_demo_window = !self.obj().show_demo_window;
  }
}

impl LuaTypeTrait for Settings {}

impl UserData for LuaType<Settings> {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method_mut("show_profiler", |_, this, _: ()| Ok(this.show_profiler()));
    methods.add_method_mut("set_show_profiler", |_, this, show_profiler: bool| {
      this.set_show_profiler(show_profiler);
      Ok(())
    });
    methods.add_method_mut("show_or_hide_profiler", |_, this, _: ()| {
      this.invert_profiler_display();
      Ok(())
    });

    methods.add_method_mut("show_demo_window", |_, this, _: ()| {
      Ok(this.show_demo_window())
    });
    methods.add_method_mut("set_show_demo_window", |_, this, show_demo_window: bool| {
      this.set_show_demo_window(show_demo_window);
      Ok(())
    });
    methods.add_method_mut("show_or_hide_demo_window", |_, this, _: ()| {
      this.invert_demo_window_display();
      Ok(())
    });
  }
}
