mod fps;
mod logging;
mod settings;

pub use fps::FpsManager;
pub use logging::{gl_error_handler, setup_logger};
pub use settings::Settings;
use std::{
  ffi::{OsStr, OsString},
  path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct DirID {
  id: OsString,
}

impl DirID {
  pub fn id(&self) -> &OsString {
    &self.id
  }

  pub fn extend<T: AsRef<OsStr>>(&self, s: T) -> Self {
    let mut copy = self.clone();
    copy.id.push(".");
    copy.id.push(s);
    copy
  }
}

impl From<PathBuf> for DirID {
  fn from(path: PathBuf) -> Self {
    let mut v = Vec::default();

    for part in path.iter() {
      v.push(part);
    }

    let mut id = std::ffi::OsString::default();

    for i in 0..v.len() {
      id.push(v[i]);
      if i != v.len() - 1 {
        id.push(".");
      }
    }

    Self { id }
  }
}

pub fn iterate_dir_with_id<F: FnMut(&Path, DirID)>(dir: &Path, mut f: F) {
  for result in WalkDir::new(dir) {
    let entry: DirEntry = result.unwrap();
    if entry.file_type().is_file() {
      let entry_suffix = entry.path().strip_prefix(dir).unwrap();
      let mut entry_cpy = entry_suffix.to_path_buf();
      entry_cpy.pop();
      let last = entry.path().file_stem().unwrap();
      let id = entry_cpy.join(last);

      let id = DirID::from(id);
      f(entry.path(), id);
    }
  }
}
