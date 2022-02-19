use crate::util::{DirID, Logger};
use mlua::prelude::*;
use std::{cell::RefCell, collections::BTreeMap, fs, mem, path::PathBuf, rc::Rc};

pub mod prelude {
  pub use super::{LuaType, LuaTypeTrait};
  pub use mlua::{prelude::*, Lua, UserData, UserDataFields, UserDataMethods};
}

pub struct LuaType<T>(*mut T);

impl<T> LuaType<T> {
  pub fn from_type(t: *mut T) -> Self {
    Self(t)
  }

  pub fn obj(&self) -> &T {
    unsafe { &*self.0 }
  }

  pub fn obj_mut(&mut self) -> &mut T {
    unsafe { &mut *self.0 }
  }
}

impl<T> Clone for LuaType<T> {
  fn clone(&self) -> Self {
    Self(self.0)
  }
}

impl<T> Copy for LuaType<T> {}

pub trait LuaTypeTrait {
  fn create_lua_type(&mut self) -> LuaType<Self>
  where
    Self: Sized,
  {
    LuaType::from_type(self)
  }
}

#[derive(Default)]
pub struct ScriptRepository {
  init_fns: Vec<Box<dyn Fn(&Lua)>>,
  scripts: BTreeMap<DirID, &'static Lua>,
  sources: BTreeMap<DirID, String>,
}

impl ScriptRepository {
  pub fn new<L, I>(logger: &L, iter: I) -> Self
  where
    L: Logger,
    I: Iterator<Item = (PathBuf, DirID)>,
  {
    let mut ret = Self {
      init_fns: Default::default(),
      scripts: Default::default(),
      sources: Default::default(),
    };

    for (path, id) in iter {
      logger.info(format!("loading {:?} as id {:?}", path, id));
      if let Ok(src) = fs::read_to_string(&path) {
        let lua = Lua::new().into_static();
        ret.scripts.insert(id.clone(), lua);
        ret.sources.insert(id, src);
      } else {
        logger.error(format!("could not read {:?}", path));
      }
    }

    ret
  }

  pub fn register_init_fn(&mut self, f: Box<dyn Fn(&Lua)>) {
    self.init_fns.push(f);
  }

  pub fn load_scripts<L: Logger>(&mut self, logger: &L) {
    logger.debug("loading scripts".to_string());
    let keys = self.sources.keys();
    for key in keys {
      let lua = self.scripts.get(key).unwrap();
      let src = self.sources.get(key).unwrap();
      for f in &self.init_fns {
        f(&lua);
      }
      if let Err(e) = lua.load(&src).exec() {
        logger.error(format!("could not load {:?}: {}", key, e));
      }
    }
  }

  fn unload_scripts(mut self) {
    let scripts = mem::take(&mut self.scripts);
    for (_, script) in scripts {
      unsafe {
        Lua::from_static(script);
      }
    }
  }

  pub fn get(&self, id: &str) -> Option<&'static Lua> {
    self.scripts.get(&DirID::from(id)).cloned()
  }
}

impl Drop for ScriptRepository {
  fn drop(&mut self) {
    let orig = mem::take(self);
    orig.unload_scripts();
  }
}
