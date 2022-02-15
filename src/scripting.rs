use crate::util::{DirID, Logger};
use mlua::prelude::*;
use std::{cell::RefCell, collections::BTreeMap, fs, path::PathBuf, rc::Rc};

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

pub struct ScriptRepository {
  init_fns: Vec<Box<dyn Fn(&mut Lua)>>,
  scripts: BTreeMap<DirID, Rc<RefCell<Lua>>>,
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
        ret
          .scripts
          .insert(id.clone(), Rc::new(RefCell::new(Lua::new())));
        ret.sources.insert(id, src);
      } else {
        logger.error(format!("could not read {:?}", path));
      }
    }

    ret
  }

  pub fn register_init_fn(&mut self, f: Box<dyn Fn(&mut Lua)>) {
    self.init_fns.push(f);
  }

  pub fn load_scripts<L: Logger>(&mut self, logger: &L) {
    let keys = self.sources.keys();
    for key in keys {
      let lua = self.scripts.get(key).unwrap();
      let src = self.sources.get(key).unwrap();
      for f in &self.init_fns {
        f(&mut lua.borrow_mut());
      }
      if let Err(e) = lua.borrow().load(&src).exec() {
        logger.error(format!("could not load {:?}: {}", key, e));
      }
    }
  }

  pub fn get(&self, id: &str) -> Option<Rc<RefCell<Lua>>> {
    self.scripts.get(&DirID::from(id)).cloned()
  }
}
