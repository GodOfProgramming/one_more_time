use crate::util::prelude::*;
use fern::InitError;
use log::LevelFilter;
use log::{debug, error, info, trace, warn};
use mlua::{UserData, UserDataMethods};
use std::{
  ffi::OsString,
  fs::{self, OpenOptions},
  path::{Path, PathBuf},
  thread::JoinHandle,
  time::SystemTime,
};

const LOG_DIR: &str = "logs";
const BASE_LOG_FILENAME: &str = "game";

pub trait Logger {
  fn trace(&self, msg: String);
  fn debug(&self, msg: String);
  fn info(&self, msg: String);
  fn warn(&self, msg: String);
  fn error(&self, msg: String);
}

pub trait SpawnableLogger<C: Logger>: Logger {
  fn spawn(&self) -> C;
}

enum LogMessage {
  Trace(String),
  Debug(String),
  Info(String),
  Warn(String),
  Error(String),
  Stop,
}

pub struct MainLogger {
  logging_thread: JoinHandle<()>,
  sender: std::sync::mpsc::Sender<LogMessage>,
}

impl MainLogger {
  pub fn new(limit: usize) -> Self {
    let (sender, receiver) = std::sync::mpsc::channel::<LogMessage>();
    let logging_thread = std::thread::spawn(move || {
      for message in receiver {
        match message {
          LogMessage::Trace(msg) => trace!("{}", msg),
          LogMessage::Debug(msg) => debug!("{}", msg),
          LogMessage::Info(msg) => info!("{}", msg),
          LogMessage::Warn(msg) => warn!("{}", msg),
          LogMessage::Error(msg) => error!("{}", msg),
          LogMessage::Stop => break,
        }
      }
    });

    Self::setup_logger(limit);

    Self {
      logging_thread,
      sender,
    }
  }

  pub fn setup_logger(rotation_limit: usize) -> Result<(), InitError> {
    let logs = MainLogger::read_log_dir();
    let filename = MainLogger::next_log_rotation(logs, rotation_limit);

    fs::create_dir_all(LOG_DIR)?;

    fern::Dispatch::new()
      .format(|out, msg, record| {
        out.finish(format_args!(
          "{}[{}] {}",
          // "{}[{}][{}] {}",
          chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
          // record.target(),
          record.level(),
          msg
        ))
      })
      .level(LevelFilter::Debug)
      .chain(std::io::stdout())
      .chain(
        OpenOptions::new()
          .create(true)
          .truncate(true)
          .write(true)
          .open(filename)?,
      )
      .apply()
      .map_err(InitError::SetLoggerError)
  }

  fn read_log_dir() -> Vec<(OsString, SystemTime)> {
    let mut vec = Vec::new();

    let p = Path::new(LOG_DIR);

    if let Ok(entries) = fs::read_dir(p) {
      for entry in entries.flatten() {
        if let Ok(metadata) = entry.metadata() {
          if let Ok(modified) = metadata.modified() {
            vec.push((entry.file_name(), modified));
          }
        }
      }
    }

    vec
  }

  fn next_log_rotation(mut existing_logs: Vec<(OsString, SystemTime)>, limit: usize) -> PathBuf {
    let mut next = PathBuf::default();
    next.push(LOG_DIR);

    let mut should_overwrite = true;

    let find_fn = |filename: &OsString| {
      let mut found = false;

      for entry in &existing_logs {
        if entry.0 == *filename {
          found = true;
          break;
        }
      }

      found
    };

    for x in 0..limit {
      let filename = format!("{}_{}.log", BASE_LOG_FILENAME, x);
      let os_fn: OsString = filename.clone().into();
      if !find_fn(&os_fn) {
        should_overwrite = false;
        next.push(filename);
        break;
      }
    }

    if should_overwrite {
      existing_logs.sort_by(
        |left: &(OsString, SystemTime), right: &(OsString, SystemTime)| left.1.cmp(&right.1),
      );
      if let Some(log) = existing_logs.first() {
        next.push(log.0.clone());
      }
    }

    next
  }
}

impl Drop for MainLogger {
  fn drop(&mut self) {
    self.sender.send(LogMessage::Stop).unwrap();
  }
}

impl Logger for MainLogger {
  fn trace(&self, msg: String) {
    self.sender.send(LogMessage::Trace(msg)).ok();
  }

  fn debug(&self, msg: String) {
    self.sender.send(LogMessage::Debug(msg)).ok();
  }

  fn info(&self, msg: String) {
    self.sender.send(LogMessage::Info(msg)).ok();
  }

  fn warn(&self, msg: String) {
    self.sender.send(LogMessage::Warn(msg)).ok();
  }

  fn error(&self, msg: String) {
    self.sender.send(LogMessage::Error(msg)).ok();
  }
}

impl AsPtr for MainLogger {}

impl UserData for ConstPtr<MainLogger> {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method_mut("trace", |_, this, msg: String| {
      this.trace(msg);
      Ok(())
    });

    methods.add_method_mut("debug", |_, this, msg: String| {
      this.debug(msg);
      Ok(())
    });

    methods.add_method_mut("info", |_, this, msg: String| {
      this.info(msg);
      Ok(())
    });

    methods.add_method_mut("warn", |_, this, msg: String| {
      this.warn(msg);
      Ok(())
    });

    methods.add_method_mut("error", |_, this, msg: String| {
      this.error(msg);
      Ok(())
    });
  }
}

impl SpawnableLogger<ChildLogger> for MainLogger {
  fn spawn(&self) -> ChildLogger {
    ChildLogger {
      sender: self.sender.clone(),
    }
  }
}

pub struct ChildLogger {
  sender: std::sync::mpsc::Sender<LogMessage>,
}

impl Logger for ChildLogger {
  fn trace(&self, msg: String) {
    self.sender.send(LogMessage::Trace(msg)).ok();
  }

  fn debug(&self, msg: String) {
    self.sender.send(LogMessage::Debug(msg)).ok();
  }

  fn info(&self, msg: String) {
    self.sender.send(LogMessage::Info(msg)).ok();
  }

  fn warn(&self, msg: String) {
    self.sender.send(LogMessage::Warn(msg)).ok();
  }

  fn error(&self, msg: String) {
    self.sender.send(LogMessage::Error(msg)).ok();
  }
}

impl SpawnableLogger<ChildLogger> for ChildLogger {
  fn spawn(&self) -> Self {
    Self {
      sender: self.sender.clone(),
    }
  }
}

#[cfg(test)]
pub mod tests {
  use super::Logger;

  pub struct MockLogger;

  impl Logger for MockLogger {
    fn trace(&self, _: std::string::String) {}
    fn debug(&self, _: std::string::String) {}
    fn info(&self, _: std::string::String) {}
    fn warn(&self, _: std::string::String) {}
    fn error(&self, _: std::string::String) {}
  }
}
