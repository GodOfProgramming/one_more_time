use std::{
  ffi::{OsStr, OsString},
  fs,
  path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Clone)]
pub struct Dirs {
  root: PathBuf,
  assets: PathBuf,
  config: PathBuf,
}

impl Dirs {
  pub fn new(root: PathBuf) -> Self {
    Self {
      assets: root.clone().join("assets"),
      config: root.clone().join("config"),
      root,
    }
  }

  pub fn root(&self) -> PathBuf {
    self.root.clone()
  }

  pub fn assets(&self) -> PathBuf {
    self.assets.clone()
  }

  pub fn config(&self) -> PathBuf {
    self.config.clone()
  }
}

pub fn recursive<F: FnMut(&Path)>(start: &Path, f: &mut F) {
  if let Ok(entries) = fs::read_dir(&start) {
    for entry in entries.flatten() {
      let path = entry.path();
      if path.is_dir() {
        recursive(&path, f);
      } else {
        (*f)(&path);
      }
    }
  };
}

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
