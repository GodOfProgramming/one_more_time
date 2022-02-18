use crate::scripting::{LuaType, LuaTypeTrait};
use mlua::{UserData, UserDataFields};
pub use nalgebra_glm as glm;

#[derive(Debug, Clone, Copy)]
pub struct Dim<T: Sized + std::fmt::Debug + Clone + Copy> {
  pub x: T,
  pub y: T,
}

impl<T> Dim<T>
where
  T: Sized + std::fmt::Debug + Clone + Copy,
{
  pub fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

impl<T> LuaType<Dim<T>>
where
  T: Sized + std::fmt::Debug + Clone + Copy,
{
  pub fn x(&self) -> T {
    self.obj().x
  }

  pub fn set_x(&mut self, x: T) {
    self.obj_mut().x = x;
  }

  pub fn y(&self) -> T {
    self.obj().y
  }

  pub fn set_y(&mut self, y: T) {
    self.obj_mut().y = y;
  }
}

impl LuaTypeTrait for Dim<u32> {}

impl UserData for LuaType<Dim<u32>> {
  fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
    fields.add_field_method_get("x", |_, this| Ok(this.x()));
    fields.add_field_method_set("x", |_, this, v: u32| {
      this.obj_mut().x = v;
      Ok(())
    });

    fields.add_field_method_get("y", |_, this| Ok(this.y()));
    fields.add_field_method_set("y", |_, this, v: u32| {
      this.obj_mut().y = v;
      Ok(())
    });
  }
}
