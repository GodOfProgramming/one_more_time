use std::{
  ffi::{OsStr, OsString},
  path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

pub struct RecursiveDirectoryIterator {
  dirs: Vec<PathBuf>,
  idx: usize,
}

impl RecursiveDirectoryIterator {
  pub fn iterate_with_prefix(path: &Path) -> Self {
    let mut dirs = Vec::new();

    for result in WalkDir::new(path) {
      let entry: DirEntry = result.unwrap();
      if entry.file_type().is_file() {
        dirs.push(entry.path().to_path_buf());
      }
    }

    Self { dirs, idx: 0 }
  }
}

impl From<&Path> for RecursiveDirectoryIterator {
  fn from(path: &Path) -> Self {
    Self::from(&path.to_path_buf())
  }
}

impl From<&PathBuf> for RecursiveDirectoryIterator {
  fn from(path: &PathBuf) -> Self {
    let mut dirs = Vec::new();

    for result in WalkDir::new(path) {
      let entry: DirEntry = result.unwrap();
      if entry.file_type().is_file() {
        dirs.push(entry.path().strip_prefix(path).unwrap().to_path_buf());
      }
    }

    Self { dirs, idx: 0 }
  }
}

impl Iterator for RecursiveDirectoryIterator {
  type Item = PathBuf;
  fn next(&mut self) -> std::option::Option<<Self as Iterator>::Item> {
    let dir = self.dirs.get(self.idx);
    self.idx += 1;
    dir.cloned()
  }
}

#[derive(Debug, Clone)]
pub struct Dirs {
  pub root: PathBuf,
  pub assets: AssetsDir,
  pub config: PathBuf,
}

impl Dirs {
  pub fn new(root: PathBuf) -> Self {
    Self {
      assets: AssetsDir::new(root.join("assets")),
      config: root.join("config"),
      root,
    }
  }
}

#[derive(Debug, Clone)]
pub struct AssetsDir {
  pub cfg: PathBuf,
  pub maps: PathBuf,
  pub shaders: PathBuf,
  pub textures: PathBuf,
  pub ui: PathBuf,
}

impl AssetsDir {
  fn new(dir: PathBuf) -> Self {
    Self {
      cfg: dir.join("cfg"),
      maps: dir.join("maps"),
      shaders: dir.join("shaders"),
      textures: dir.join("textures"),
      ui: dir.join("ui"),
    }
  }
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
