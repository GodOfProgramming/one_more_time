use omt::walkdir::{DirEntry, WalkDir};
use std::{
  ffi::{OsStr, OsString},
  fmt::Display,
  ops::Deref,
  path::{Path, PathBuf},
};

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

impl Deref for DirID {
  type Target = str;
  fn deref(&self) -> &Self::Target {
    self.id.to_str().unwrap_or_default()
  }
}

impl Display for DirID {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{:?}", self.id)
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

impl From<&str> for DirID {
  fn from(id: &str) -> Self {
    Self {
      id: OsString::from(id),
    }
  }
}

impl From<String> for DirID {
  fn from(id: String) -> Self {
    Self {
      id: OsString::from(id),
    }
  }
}

impl Into<String> for DirID {
  fn into(self) -> String {
    String::from(self.id.to_str().unwrap_or_default())
  }
}

pub struct RecursiveDirIterator {
  dirs: Vec<PathBuf>,
  idx: usize,
}

impl RecursiveDirIterator {
  pub fn iterate_with_prefix(path: &Path) -> Self {
    let mut dirs = Vec::new();

    for result in WalkDir::new(path) {
      let entry: DirEntry = result.unwrap();
      if entry.file_type().is_file() {
        dirs.push(entry.path().to_path_buf());
      }
    }

    let dirs = dirs.into_iter().rev().collect();

    RecursiveDirIterator { dirs, idx: 0 }
  }
}

impl Iterator for RecursiveDirIterator {
  type Item = PathBuf;
  fn next(&mut self) -> std::option::Option<<Self as Iterator>::Item> {
    let dir = self.dirs.get(self.idx);
    self.idx += 1;
    dir.cloned()
  }
}

impl From<&Path> for RecursiveDirIterator {
  fn from(path: &Path) -> Self {
    Self::from(&path.to_path_buf())
  }
}

impl From<&PathBuf> for RecursiveDirIterator {
  fn from(path: &PathBuf) -> Self {
    let mut dirs = Vec::new();

    for result in WalkDir::new(path) {
      let entry: DirEntry = result.unwrap();
      if entry.file_type().is_file() {
        dirs.push(entry.path().strip_prefix(path).unwrap().to_path_buf());
      }
    }

    let dirs = dirs.into_iter().rev().collect();

    RecursiveDirIterator { dirs, idx: 0 }
  }
}

pub struct RecursiveDirIteratorWithID {
  dirs: Vec<(PathBuf, DirID)>,
  idx: usize,
}

impl From<&PathBuf> for RecursiveDirIteratorWithID {
  fn from(path: &PathBuf) -> Self {
    let mut dirs = Vec::new();

    for result in WalkDir::new(path) {
      let entry: DirEntry = result.unwrap();
      if entry.file_type().is_file() {
        let entry_suffix = entry.path().strip_prefix(path).unwrap();
        let mut entry_cpy = entry_suffix.to_path_buf();
        entry_cpy.pop();
        let last = entry.path().file_stem().unwrap();
        let id = entry_cpy.join(last);
        dirs.push((entry.path().to_path_buf(), DirID::from(id)));
      }
    }

    let dirs = dirs.into_iter().rev().collect();

    Self { dirs, idx: 0 }
  }
}

impl Iterator for RecursiveDirIteratorWithID {
  type Item = (PathBuf, DirID);
  fn next(&mut self) -> std::option::Option<<Self as Iterator>::Item> {
    let dir = self.dirs.get(self.idx);
    self.idx += 1;
    dir.cloned()
  }
}
