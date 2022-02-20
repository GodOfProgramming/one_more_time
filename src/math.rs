use crate::scripting::prelude::*;
pub use nalgebra_glm as glm;

pub mod geom;

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

impl AsPtr for Dim<u32> {}

impl UserData for MutPtr<Dim<u32>> {
  fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
    fields.add_field_method_get("x", |_, this| Ok(this.x));
    fields.add_field_method_set("x", |_, this, v: u32| {
      this.x = v;
      Ok(())
    });

    fields.add_field_method_get("y", |_, this| Ok(this.y));
    fields.add_field_method_set("y", |_, this, v: u32| {
      this.y = v;
      Ok(())
    });
  }
}
