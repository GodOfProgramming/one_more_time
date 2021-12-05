use dlopen::{
  symbor::{Library, Symbol},
  Error,
};
use log::warn;
use std::{collections::BTreeMap, sync::Mutex};

#[derive(Default)]
pub struct LibStorage {
  libs: BTreeMap<String, Library>,
}

impl LibStorage {
  pub fn new_lib(lib: &str) -> Result<Library, Error> {
    Library::open(lib)
  }

  pub fn add(&mut self, id: String, lib: Library) {
    self.libs.insert(id, lib);
  }

  pub fn utilize<S, F: FnOnce(Result<Symbol<S>, Error>)>(&self, id: &str, symbol: &str, f: F) {
    if let Some(lib) = self.libs.get(id) {
      let sym = unsafe { lib.symbol::<S>(symbol) };
      f(sym);
    } else {
      warn!("could not find lib '{}'", id);
    }
  }
}
