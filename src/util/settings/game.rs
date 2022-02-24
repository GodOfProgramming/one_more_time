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

impl AsPtr for Settings {}

impl MutPtr<Settings> {
  fn invert_profiler_display(&mut self) {
    self.show_profiler = !self.show_profiler;
  }

  fn invert_demo_window_display(&mut self) {
    self.show_demo_window = !self.show_demo_window;
  }
}
