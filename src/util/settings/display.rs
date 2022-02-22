use crate::{math::Dim, util::prelude::*};
use omt::toml::{value::Table, Value};

mod keys {
  pub const TITLE: &str = "title";
  pub const WIDTH: &str = "width";
  pub const HEIGHT: &str = "height";
  pub const MODE: &str = "video_mode";
  pub const WINDOW: &str = "window";
}

#[derive(Debug, Clone)]
pub struct Settings {
  pub title: String,
  pub window: Dim<u32>,
  pub mode: String,
}

impl Settings {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      title: String::from("game"),
      window: Dim::new(1280, 720),
      mode: String::from("windowed"),
    }
  }
}

impl From<&Table> for Settings {
  fn from(table: &Table) -> Self {
    let mut settings = Self::new();

    if let Some(Value::String(title)) = table.get(keys::TITLE) {
      settings.title = title.clone();
    }

    if let Some(Value::Table(window)) = table.get(keys::WINDOW) {
      if let Some(Value::Integer(width)) = window.get(keys::WIDTH) {
        settings.window.x = (*width).try_into().unwrap_or(1280);
      }

      if let Some(Value::Integer(height)) = window.get(keys::HEIGHT) {
        settings.window.y = (*height).try_into().unwrap_or(720);
      }
    }

    if let Some(Value::String(mode)) = table.get(keys::MODE) {
      settings.mode = mode.clone();
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

    table.insert(String::from(keys::TITLE), Value::String(self.title.clone()));

    {
      let mut window = Table::new();

      window.insert(
        String::from(keys::WIDTH),
        Value::Integer(self.window.x.try_into().unwrap_or(1080)),
      );

      window.insert(
        String::from(keys::HEIGHT),
        Value::Integer(self.window.y.try_into().unwrap_or(1920)),
      );

      table.insert(String::from(keys::WINDOW), Value::Table(window));
    }

    table.insert(
      String::from(keys::MODE),
      Value::String(self.mode.to_string()),
    );

    table
  }
}

impl AsPtr for Settings {}

impl UserData for MutPtr<Settings> {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method_mut("title", |_, this, _: ()| Ok(this.title.clone()));
    methods.add_method_mut("set_title", |_, this, title: String| {
      this.title = title;
      Ok(())
    });

    methods.add_method_mut("window", |_, this, _: ()| Ok(this.window.as_ptr_mut()));
    methods.add_method_mut("set_window", |_, this, v: mlua::Value| match v {
      mlua::Value::Table(tbl) => {
        if let Ok(x) = tbl.get("x") {
          this.window.x = x;
        }

        if let Ok(y) = tbl.get("y") {
          this.window.y = y;
        }
        Ok(())
      }

      mlua::Value::UserData(ud) => {
        if ud.is::<MutPtr<Dim<u32>>>() {
          let dim = ud.get_user_value::<MutPtr<Dim<u32>>>()?;
          this.window = *dim;
        }
        Ok(())
      }

      _ => Err(mlua::Error::RuntimeError(String::from(
        "invalid parameter type",
      ))),
    });

    methods.add_method_mut("mode", |_, this, _: ()| Ok(this.mode.clone()));
    methods.add_method_mut("set_mode", |_, this, mode: String| {
      this.mode = mode;
      Ok(())
    });
  }
}
