mod settings;

use fern::InitError;
use log::LevelFilter;
pub use settings::Settings;
use std::{
  cmp::Ordering,
  ffi::OsString,
  fs::{self, Metadata},
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
  fs::create_dir_all(LOG_DIR).map_err(InitError::Io)?;

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
    .chain(fern::log_file(filename)?)
    .apply()
    .map_err(InitError::SetLoggerError)
}
