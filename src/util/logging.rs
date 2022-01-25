use fern::InitError;
use glium::debug::{MessageType, Severity, Source};
use lazy_static::lazy_static;
use log::LevelFilter;
use log::{debug, error, info, trace, warn};
use std::{
  ffi::OsString,
  fs::{self, OpenOptions},
  path::{Path, PathBuf},
  thread::JoinHandle,
  time::SystemTime,
};

const LOG_DIR: &str = "logs";
const BASE_LOG_FILENAME: &str = "game";

enum LogMessage {
  Trace(String),
  Debug(String),
  Info(String),
  Warn(String),
  Error(String),
}

pub struct Logger {
  logging_thread: JoinHandle<()>,
  sender: std::sync::mpsc::Sender<LogMessage>,
}

impl Logger {
  pub fn new() -> Self {
    let (sender, receiver) = std::sync::mpsc::channel::<LogMessage>();
    let logging_thread = std::thread::spawn(move || {
      for message in receiver {
        match message {
          LogMessage::Trace(msg) => trace!("{}", msg),
          LogMessage::Debug(msg) => debug!("{}", msg),
          LogMessage::Info(msg) => info!("{}", msg),
          LogMessage::Warn(msg) => warn!("{}", msg),
          LogMessage::Error(msg) => error!("{}", msg),
        }
      }
    });

    Self {
      logging_thread,
      sender,
    }
  }

  pub fn spawn(&self) -> ChildLogger {
    ChildLogger {
      sender: self.sender.clone(),
    }
  }

  pub fn trace(&self, msg: String) {
    self.sender.send(LogMessage::Trace(msg)).ok();
  }

  pub fn debug(&self, msg: String) {
    self.sender.send(LogMessage::Debug(msg)).ok();
  }

  pub fn info(&self, msg: String) {
    self.sender.send(LogMessage::Info(msg)).ok();
  }

  pub fn warn(&self, msg: String) {
    self.sender.send(LogMessage::Warn(msg)).ok();
  }

  pub fn error(&self, msg: String) {
    self.sender.send(LogMessage::Error(msg)).ok();
  }

  pub fn setup_logger(rotation_limit: usize) -> Result<(), InitError> {
    let logs = Logger::read_log_dir();
    let filename = Logger::next_log_rotation(logs, rotation_limit);

    println!("logging to {:?}", filename);

    fs::create_dir_all(LOG_DIR)?;

    fern::Dispatch::new()
      .format(|out, msg, record| {
        out.finish(format_args!(
          "{}[{}][{}] {}",
          chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
          record.target(),
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

pub struct ChildLogger {
  sender: std::sync::mpsc::Sender<LogMessage>,
}

impl ChildLogger {
  pub fn spawn(&self) -> Self {
    Self {
      sender: self.sender.clone(),
    }
  }

  pub fn trace(&self, msg: String) {
    self.sender.send(LogMessage::Trace(msg)).ok();
  }

  pub fn debug(&self, msg: String) {
    self.sender.send(LogMessage::Debug(msg)).ok();
  }

  pub fn info(&self, msg: String) {
    self.sender.send(LogMessage::Info(msg)).ok();
  }

  pub fn warn(&self, msg: String) {
    self.sender.send(LogMessage::Warn(msg)).ok();
  }

  pub fn error(&self, msg: String) {
    self.sender.send(LogMessage::Error(msg)).ok();
  }
}

pub fn gl_error_handler(
  source: Source,
  message_type: MessageType,
  severity: Severity,
  _ident: u32,
  handled: bool,
  message: &str,
) {
}
