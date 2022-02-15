use super::DirID;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

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
