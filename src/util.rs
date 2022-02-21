pub use self::{
  convert::*,
  fps::FpsManager,
  io::{DirID, Dirs, RecursiveDirIterator, RecursiveDirIteratorWithID},
  logging::{ChildLogger, Logger, MainLogger, SpawnableLogger},
  ptr::{AsPtr, ConstPtr, MutPtr},
  script::*,
  settings::Settings,
  xml::XmlNode,
};
use std::fmt::{Display, Error, Formatter};

pub mod convert;
mod fps;
mod io;
mod logging;
pub mod ptr;
pub mod script;
mod settings;
mod xml;

pub mod prelude {
  pub use super::*;
}

#[derive(Debug)]
pub enum GameError {
  Simple(String),
  Lua(mlua::Error),
}

impl Display for GameError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    match self {
      GameError::Simple(msg) => write!(f, "{}", msg),
      GameError::Lua(err) => write!(f, "{}", err),
    }
  }
}
