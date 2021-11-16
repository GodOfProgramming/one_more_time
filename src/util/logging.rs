use fern::InitError;
use glium::debug::{MessageType, Severity, Source};
use log::LevelFilter;
use log::{debug, error, info, warn};
use std::{
  ffi::OsString,
  fs::{self, OpenOptions},
  path::{Path, PathBuf},
  time::SystemTime,
};

const LOG_DIR: &str = "logs";
const BASE_LOG_FILENAME: &str = "game";

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

pub fn setup_logger(rotation_limit: usize) -> Result<(), InitError> {
  let logs = read_log_dir();
  let filename = next_log_rotation(logs, rotation_limit);

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
    .level(LevelFilter::Info)
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
  _ident: u32,
  handled: bool,
  message: &str,
) {
  match severity {
    glium::debug::Severity::Notification => {
      debug!(
        "OpenGL Notification: source = {:?}, message type = {:?}, handled = {:?} -> {}",
        source, message_type, handled, message
      );
    }
    glium::debug::Severity::Low => {
      info!(
        "OpenGL Warning: source = {:?}, message type = {:?}, handled = {:?} -> {}",
        source, message_type, handled, message
      );
    }
    glium::debug::Severity::Medium => {
      warn!(
        "OpenGL Warning: source = {:?}, message type = {:?}, handled = {:?} -> {}",
        source, message_type, handled, message
      );
    }
    glium::debug::Severity::High => {
      error!(
        "OpenGL Error: source = {:?}, message type = {:?}, handled = {:?} -> {}",
        source, message_type, handled, message
      );
      std::process::exit(1);
    }
    _ => (),
  }
}
