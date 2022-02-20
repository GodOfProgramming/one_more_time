use super::common::*;

mod keys {
  pub const FPS: &str = "fps";
}

#[derive(Debug)]
pub struct Settings {
  pub fps: u8,
}

impl Settings {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Default for Settings {
  fn default() -> Self {
    Self { fps: 60 }
  }
}

impl From<&Table> for Settings {
  fn from(table: &Table) -> Self {
    let mut settings = Self::new();

    if let Some(Value::Integer(fps)) = table.get(keys::FPS) {
      settings.fps = (*fps).try_into().unwrap_or(60);
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
      String::from(keys::FPS),
      Value::Integer(self.fps.try_into().unwrap_or(60)),
    );

    table
  }
}

impl AsPtr for Settings {}

impl UserData for MutPtr<Settings> {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("fps", |_, this, _: ()| Ok(this.fps));
    methods.add_method_mut("set_fps", |_, this, fps: u8| {
      this.fps = fps;
      Ok(())
    });
  }
}
