use toml::{value::Table, Value};

mod keys {
  pub const FPS: &str = "fps";
}

#[derive(Debug)]
pub struct GraphicsSettings {
  pub fps: u8,
}

impl GraphicsSettings {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Default for GraphicsSettings {
  fn default() -> Self {
    Self { fps: 60 }
  }
}

impl From<&Table> for GraphicsSettings {
  fn from(table: &Table) -> Self {
    let mut settings = Self::new();

    if let Some(Value::Integer(fps)) = table.get(keys::FPS) {
      settings.fps = (*fps).try_into().unwrap_or(60);
    }

    settings
  }
}

impl From<Table> for GraphicsSettings {
  fn from(table: Table) -> Self {
    Self::from(&table)
  }
}

impl Into<Table> for GraphicsSettings {
  fn into(self) -> Table {
    let mut table = Table::new();

    table.insert(
      String::from(keys::FPS),
      Value::Integer(self.fps.try_into().unwrap_or(60)),
    );

    table
  }
}
