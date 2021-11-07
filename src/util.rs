mod fps;
mod settings;

use fern::InitError;
pub use fps::FpsManager;
use glium::debug::{MessageType, Severity, Source};
use log::LevelFilter;
use log::{error, info, warn};
pub use settings::Settings;
use std::{
  ffi::OsString,
  fs::{self, OpenOptions},
  path::{Path, PathBuf},
  time::SystemTime,
};

const LOG_DIR: &str = "logs";
const BASE_LOG_FILENAME: &str = "game";

pub fn read_log_dir() -> Vec<(OsString, SystemTime)> {
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

pub fn next_log_rotation(mut existing_logs: Vec<(OsString, SystemTime)>, limit: usize) -> PathBuf {
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

pub fn setup_logger(filename: &Path) -> Result<(), InitError> {
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
    .level(LevelFilter::Trace)
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

pub fn gl_error_handler(
  source: Source,
  message_type: MessageType,
  severity: Severity,
  ident: u32,
  handled: bool,
  message: &str,
) {
  match severity {
    glium::debug::Severity::Notification => {
      info!("{:?} {:?} handled={:?}", source, message_type, handled);
    }
    glium::debug::Severity::Low => {
      warn!("{:?} {:?} handled={:?}", source, message_type, handled);
    }
    glium::debug::Severity::Medium => {
      error!("{:?} {:?} handled={:?}", source, message_type, handled);
    }
    glium::debug::Severity::High => {
      error!(
        "FATAL {:?} {:?} handled={:?}",
        source, message_type, handled
      );
      std::process::exit(1);
    }
  }
}
