pub use self::{
  convert::*,
  external::Lib,
  fps::FpsManager,
  io::Dirs,
  logging::{ChildLogger, Logger, MainLogger, SpawnableLogger},
  ptr::*,
  settings::Settings,
  xml::XmlNode,
};
use std::fmt::{Display, Error, Formatter};

pub mod convert;
mod external;
mod fps;
mod io;
mod logging;
mod ptr;
mod settings;
mod xml;

pub mod prelude {
  pub use super::*;
}

#[derive(Debug)]
pub enum GameError {
  Simple(String),
}

impl Display for GameError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    match self {
      GameError::Simple(msg) => write!(f, "{}", msg),
    }
  }
}
