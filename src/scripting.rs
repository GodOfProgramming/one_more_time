use crate::util::{DirID, Logger};
use mlua::{prelude::*, UserData, UserDataMethods};
use std::{cell::RefCell, collections::BTreeMap, fs, ops::DerefMut, path::PathBuf, rc::Rc};

pub struct LuaType<T>(Rc<RefCell<T>>);

impl<T> LuaType<T> {
  pub fn from_type(t: T) -> Self {
    Self(Rc::new(RefCell::new(t)))
  }

  pub fn obj(&self) -> &Rc<RefCell<T>> {
    &self.0
  }
}

impl<T> Clone for LuaType<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

pub struct ScriptRepository {
  scripts: BTreeMap<DirID, Rc<Lua>>,
}

impl ScriptRepository {
  pub fn new<L, I, F>(logger: &L, iter: I, init_fn: F) -> Self
  where
    L: Logger,
    I: Iterator<Item = (PathBuf, DirID)>,
    F: Fn(&mut Lua),
  {
    let mut scripts = BTreeMap::new();

    for (path, id) in iter {
      logger.info(format!("loading {:?} as id {:?}", path, id));
      if let Ok(src) = fs::read_to_string(&path) {
        let mut lua = Lua::new();
        init_fn(&mut lua);
        if lua.load(&src).exec().is_ok() {
          scripts.insert(id, Rc::new(lua));
        } else {
          logger.error(format!("could not load {:?}", path));
        }
      } else {
        logger.error(format!("could not read {:?}", path));
      }
    }

    Self { scripts }
  }

  pub fn get(&self, id: &str) -> Option<Rc<Lua>> {
    self.scripts.get(&DirID::from(id)).cloned()
  }
}
