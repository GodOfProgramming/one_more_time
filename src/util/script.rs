use super::prelude::*;
pub use mlua::{
  prelude::*, Table as LuaTable, UserData, UserDataFields, UserDataMetatable, UserDataMethods,
  Value as LuaValue,
};

pub fn resolve<'lua>(lua: &'lua Lua, path: &str) -> Result<LuaValue<'lua>, GameError> {
  let mut value = LuaValue::Table(lua.globals());
  for part in path.split('.') {
    match value {
      LuaValue::Table(table) => value = table.get(part).map_err(GameError::Lua)?,
      _ => {
        return Err(GameError::Simple(format!(
          "part '{}' in value '{}' not table",
          part, path
        )));
      }
    }
  }

  Ok(value)
}

#[cfg(test)]
mod tests {
  use super::logging::tests::MockLogger;
  use super::*;

  #[test]
  fn resolve__resolves_to_correct_object() {
    let lua = Lua::new();
    lua
      .load(r#"foo = { bar = { baz = 'foobarbaz' } };"#)
      .exec()
      .unwrap();

    let resolved = resolve(&lua, "foo.bar.baz").unwrap();

    if let LuaValue::String(string) = resolved {
      assert_eq!(string, String::from("foobarbaz"));
    } else {
      panic!("failed to resolve to correct value: {:?}", resolved);
    }
  }
}
