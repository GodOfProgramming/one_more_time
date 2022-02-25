use lazy_static::lazy_static;
use libloading::{Error, Library};
use std::{
  path::{Path, PathBuf},
  sync::Mutex,
};

lazy_static! {
  static ref LOADED_LIBS: Mutex<Vec<Library>> = Mutex::new(Vec::default());
}

pub struct Lib;

impl Lib {
  pub fn load_lib<F: FnOnce(PathBuf, &mut Library) -> Result<(), Error>>(
    p: &Path,
    f: F,
  ) -> Result<(), Error> {
    unsafe {
      let mut lib = Library::new(p)?;
      f(p.to_path_buf(), &mut lib)?;

      let mut libs = LOADED_LIBS.lock().unwrap();
      libs.push(lib);
    }
    Ok(())
  }
}
