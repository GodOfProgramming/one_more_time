use crate::util::ptr::prelude::*;
use std::{
  ffi::{OsStr, OsString},
  fmt::Display,
  ops::Deref,
  path::PathBuf,
  str::FromStr,
  string::ToString,
};

#[derive(Debug, Clone)]
pub struct ConfigDir {
  pub animations: PathBuf,
  pub entities: PathBuf,
  pub models: PathBuf,
  pub shaders: PathBuf,
  pub textures: PathBuf,
}

impl ConfigDir {
  fn new(dir: PathBuf) -> Self {
    Self {
      animations: dir.join("animations"),
      entities: dir.join("entities"),
      models: dir.join("models"),
      shaders: dir.join("shaders"),
      textures: dir.join("textures"),
    }
  }
}

#[derive(Debug, Clone)]
pub struct AssetsDir {
  pub cfg: ConfigDir,
  pub maps: PathBuf,
  pub shaders: PathBuf,
  pub textures: PathBuf,
  pub ui: PathBuf,
  pub scripts: PathBuf,
}

impl AssetsDir {
  fn new(dir: PathBuf) -> Self {
    Self {
      cfg: ConfigDir::new(dir.join("cfg")),
      maps: dir.join("maps"),
      shaders: dir.join("shaders"),
      textures: dir.join("textures"),
      ui: dir.join("ui"),
      scripts: dir.join("scripts"),
    }
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

impl AsPtr for Dirs {}

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
