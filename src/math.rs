use crate::scripting::prelude::*;
use mlua::{MetaMethod, Value};
pub use nalgebra_glm as glm;

pub mod geom;

pub mod lua {
  pub use super::{LuaVec1, LuaVec2, LuaVec3, LuaVec4, LuaVecT};
}

// All vectors
pub struct LuaVecT;

impl ScriptInit for LuaVecT {
  fn callback(lua: &mlua::Lua) {
    let globals = lua.globals();
    // vectors
    {
      let vector = lua.create_table().unwrap();

      // vec1
      {
        let vec = lua
          .create_function(|_, _: ()| Ok(LuaVec1::default()))
          .unwrap();
        vector.set("new_vec1", vec).unwrap();
      }

      // vec2
      {
        let vec = lua
          .create_function(|_, _: ()| Ok(LuaVec2::default()))
          .unwrap();
        vector.set("new_vec2", vec).unwrap();
      }

      // vec3
      {
        let vec = lua
          .create_function(|_, _: ()| Ok(LuaVec3::default()))
          .unwrap();
        vector.set("new_vec3", vec).unwrap();
      }

      // vec4
      {
        let vec = lua
          .create_function(|_, _: ()| Ok(LuaVec4::default()))
          .unwrap();
        vector.set("new_vec4", vec).unwrap();
      }

      globals.set("vector", vector).unwrap();
    }
  }
}

#[derive(Default, Clone, Copy)]
pub struct LuaVec1(glm::Vec1);

impl UserData for LuaVec1 {
  fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
    fields.add_field_method_get("x", |_, this| Ok(this.0.x));
    fields.add_field_method_set("x", |_, this, x: f32| {
      this.0.x = x;
      Ok(())
    });
  }
}

#[derive(Default, Clone, Copy)]
pub struct LuaVec2(glm::Vec2);

impl UserData for LuaVec2 {
  fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
    fields.add_field_method_get("x", |_, this| Ok(this.0.x));
    fields.add_field_method_set("x", |_, this, x: f32| {
      this.0.x = x;
      Ok(())
    });

    fields.add_field_method_get("y", |_, this| Ok(this.0.y));
    fields.add_field_method_set("y", |_, this, y: f32| {
      this.0.y = y;
      Ok(())
    });
  }
}

#[derive(Default, Clone, Copy)]
pub struct LuaVec3(glm::Vec3);

impl UserData for LuaVec3 {
  fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
    fields.add_field_method_get("x", |_, this| Ok(this.0.x));
    fields.add_field_method_set("x", |_, this, x: f32| {
      this.0.x = x;
      Ok(())
    });

    fields.add_field_method_get("y", |_, this| Ok(this.0.y));
    fields.add_field_method_set("y", |_, this, y: f32| {
      this.0.y = y;
      Ok(())
    });

    fields.add_field_method_get("z", |_, this| Ok(this.0.z));
    fields.add_field_method_set("z", |_, this, z: f32| {
      this.0.z = z;
      Ok(())
    });
  }

  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_meta_method(MetaMethod::ToString, |_, this, _: ()| {
      Ok(format!("{:?}", this.0))
    });

    methods.add_meta_method(MetaMethod::Add, |_, this, other: Value| match other {
      Value::UserData(ud) => {
        if ud.is::<LuaVec1>() {
          let vec1 = ud.borrow::<LuaVec1>()?;
          Ok(LuaVec3(glm::Vec3::new(
            this.0.x + vec1.0.x,
            this.0.y,
            this.0.z,
          )))
        } else if ud.is::<LuaVec2>() {
          let vec = ud.borrow::<LuaVec2>()?;
          Ok(LuaVec3(glm::Vec3::new(
            this.0.x + vec.0.x,
            this.0.y + vec.0.y,
            this.0.z,
          )))
        } else if ud.is::<LuaVec3>() {
          let vec = ud.borrow::<LuaVec3>()?;
          Ok(LuaVec3(glm::Vec3::new(
            this.0.x + vec.0.x,
            this.0.y + vec.0.y,
            this.0.z + vec.0.z,
          )))
        } else {
          Err(mlua::Error::FromLuaConversionError {
            from: "Value",
            message: None,
            to: "VectorT",
          })
        }
      }
      Value::Number(n) => {
        let n = n as f32;
        Ok(LuaVec3(glm::Vec3::new(
          this.0.x + n,
          this.0.y + n,
          this.0.z + n,
        )))
      }
      _ => Err(mlua::Error::FromLuaConversionError {
        from: "Value",
        message: None,
        to: "VectorT",
      }),
    });

    methods.add_meta_method(MetaMethod::Sub, |_, this, other: Value| match other {
      Value::UserData(ud) => {
        if ud.is::<LuaVec1>() {
          let vec1 = ud.get_user_value::<LuaVec1>()?;
          Ok(LuaVec3(glm::Vec3::new(
            this.0.x - vec1.0.x,
            this.0.y,
            this.0.z,
          )))
        } else if ud.is::<LuaVec2>() {
          let vec = ud.get_user_value::<LuaVec2>()?;
          Ok(LuaVec3(glm::Vec3::new(
            this.0.x - vec.0.x,
            this.0.y - vec.0.y,
            this.0.z,
          )))
        } else if ud.is::<LuaVec3>() {
          let vec = ud.get_user_value::<LuaVec3>()?;
          Ok(LuaVec3(glm::Vec3::new(
            this.0.x - vec.0.x,
            this.0.y - vec.0.y,
            this.0.z - vec.0.z,
          )))
        } else {
          Err(mlua::Error::FromLuaConversionError {
            from: "Value",
            message: None,
            to: "VectorT",
          })
        }
      }
      Value::Number(n) => {
        let n = n as f32;
        Ok(LuaVec3(glm::Vec3::new(
          this.0.x - n,
          this.0.y - n,
          this.0.z - n,
        )))
      }
      _ => Err(mlua::Error::FromLuaConversionError {
        from: "Value",
        message: None,
        to: "VectorT",
      }),
    });

    methods.add_meta_method(MetaMethod::Mul, |_, this, other: Value| match other {
      Value::UserData(ud) => {
        if ud.is::<LuaVec1>() {
          let vec = ud.get_user_value::<LuaVec1>()?;
          Ok(LuaVec3(glm::Vec3::new(
            this.0.x * vec.0.x,
            this.0.y,
            this.0.z,
          )))
        } else if ud.is::<LuaVec2>() {
          let vec = ud.get_user_value::<LuaVec2>()?;
          Ok(LuaVec3(glm::Vec3::new(
            this.0.x * vec.0.x,
            this.0.y * vec.0.y,
            this.0.z,
          )))
        } else if ud.is::<LuaVec3>() {
          let vec = ud.get_user_value::<LuaVec3>()?;
          Ok(LuaVec3(glm::Vec3::new(
            this.0.x * vec.0.x,
            this.0.y * vec.0.y,
            this.0.z * vec.0.z,
          )))
        } else {
          Err(mlua::Error::FromLuaConversionError {
            from: "Value",
            message: None,
            to: "VectorT",
          })
        }
      }
      Value::Number(n) => {
        let n = n as f32;
        Ok(LuaVec3(glm::Vec3::new(
          this.0.x * n,
          this.0.y * n,
          this.0.z * n,
        )))
      }
      _ => Err(mlua::Error::FromLuaConversionError {
        from: "Value",
        message: None,
        to: "VectorT",
      }),
    });

    methods.add_meta_method(MetaMethod::Div, |_, this, other: Value| match other {
      Value::UserData(ud) => {
        if ud.is::<LuaVec1>() {
          let vec1 = ud.get_user_value::<LuaVec1>()?;
          Ok(LuaVec3(glm::Vec3::new(
            this.0.x / vec1.0.x,
            this.0.y,
            this.0.z,
          )))
        } else if ud.is::<LuaVec2>() {
          let vec = ud.get_user_value::<LuaVec2>()?;
          Ok(LuaVec3(glm::Vec3::new(
            this.0.x / vec.0.x,
            this.0.y / vec.0.y,
            this.0.z,
          )))
        } else if ud.is::<LuaVec3>() {
          let vec = ud.get_user_value::<LuaVec3>()?;
          Ok(LuaVec3(glm::Vec3::new(
            this.0.x / vec.0.x,
            this.0.y / vec.0.y,
            this.0.z / vec.0.z,
          )))
        } else {
          Err(mlua::Error::FromLuaConversionError {
            from: "Value",
            message: None,
            to: "VectorT",
          })
        }
      }
      Value::Number(n) => {
        if n != 0.0 {
          let n = n as f32;
          Ok(LuaVec3(glm::Vec3::new(
            this.0.x / n,
            this.0.y / n,
            this.0.z / n,
          )))
        } else {
          Err(mlua::Error::RuntimeError("div by 0".to_string()))
        }
      }
      _ => Err(mlua::Error::FromLuaConversionError {
        from: "Value",
        message: None,
        to: "VectorT",
      }),
    });
  }
}

#[derive(Default, Clone, Copy)]
pub struct LuaVec4(glm::Vec4);

impl UserData for LuaVec4 {
  fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
    fields.add_field_method_get("x", |_, this| Ok(this.0.x));
    fields.add_field_method_set("x", |_, this, x: f32| {
      this.0.x = x;
      Ok(())
    });

    fields.add_field_method_get("y", |_, this| Ok(this.0.y));
    fields.add_field_method_set("y", |_, this, y: f32| {
      this.0.y = y;
      Ok(())
    });

    fields.add_field_method_get("z", |_, this| Ok(this.0.z));
    fields.add_field_method_set("z", |_, this, z: f32| {
      this.0.z = z;
      Ok(())
    });

    fields.add_field_method_get("w", |_, this| Ok(this.0.z));
    fields.add_field_method_set("w", |_, this, w: f32| {
      this.0.w = w;
      Ok(())
    });
  }
}

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
