use common::*;
use omt::toml::Value;
use std::{
  fs,
  path::{Path, PathBuf},
};

mod common {
  pub use crate::util::prelude::*;
  pub use omt::toml::{self, value::Table, Value};
  pub use std::cell::Cell;
}

pub struct Settings {
  file: PathBuf,
  root: Value,
}

impl Settings {
  fn new(file: PathBuf, root: Value) -> Self {
    Self { file, root }
  }

  pub fn load(p: &Path) -> Result<Settings, String> {
    match fs::read_to_string(p) {
      Ok(data) => match data.parse::<Value>() {
        Ok(root) => Ok(Settings::new(p.to_path_buf(), root)),
        Err(e) => Err(format!("could not parse settings '{}'", e)),
      },
      Err(e) => Err(format!("could not read settings at '{:?}': {}", p, e)),
    }
  }

  pub fn save(&self) -> Result<(), String> {
    match toml::to_string_pretty(&self.root) {
      Ok(data) => fs::write(&self.file, data)
        .map_err(|e| format!("could not write settings to '{:?}': {}", self.file, e)),
      Err(e) => Err(format!(
        "could not read settings at '{:?}': {}",
        self.file, e
      )),
    }
  }

  pub fn get_window_size(&self) -> (i64, i64) {
    let mut size = (1280, 720);

    if let Value::Table(table) = Self::traverse(&self.root, &["display", "window"]) {
      size.0 = match table.get("width") {
        Some(Value::Float(w)) => *w as i64,
        Some(Value::Integer(w)) => *w,
        _ => 1280,
      };

      size.1 = match table.get("height") {
        Some(Value::Float(h)) => *h as i64,
        Some(Value::Integer(h)) => *h,
        _ => 720,
      };
    }

    size
  }

  pub fn get_display_mode(&self) -> String {
    if let Value::String(vid_mode) = Self::traverse(&self.root, &["display", "video_mode"]) {
      vid_mode.clone()
    } else {
      String::from("fullscreen")
    }
  }

  pub fn get_title(&self) -> String {
    if let Value::String(title) = Self::traverse(&self.root, &["display", "title"]) {
      title.clone()
    } else {
      String::from("game")
    }
  }

  pub fn get_fps(&self) -> u64 {
    if let Value::Integer(fps) = Self::traverse(&self.root, &["graphics", "fps"]) {
      std::cmp::max(*fps, 144) as u64
    } else {
      60
    }
  }

  pub fn get_show_profiler(&self) -> bool {
    if let Value::Boolean(show_profiler) = Self::traverse(&self.root, &["game", "show_profiler"]) {
      *show_profiler
    } else {
      false
    }
  }

  pub fn get_show_demo_window(&self) -> bool {
    if let Value::Boolean(show_demo_window) =
      Self::traverse(&self.root, &["game", "show_demo_window"])
    {
      *show_demo_window
    } else {
      false
    }
  }

  fn traverse<'v>(value: &'v Value, path: &[&str]) -> &'v Value {
    if !path.is_empty() {
      if let Value::Table(table) = value {
        if let Some(inner_value) = table.get(&path.first().unwrap().to_string()) {
          return Self::traverse(inner_value, &path[1..]);
        }
      }
    }
    value
  }

  fn traverse_mut<F: FnOnce(&mut Value)>(f: F, value: &mut Value, path: &[&str]) {
    if !path.is_empty() {
      if let Value::Table(table) = value {
        if let Some(inner_value) = table.get_mut(&path.first().unwrap().to_string()) {
          Self::traverse_mut(f, inner_value, &path[1..]);
          return;
        }
      }
    }
    f(value);
  }
}

impl omt::util::Settings for Settings {
  fn lookup(&self, path: &[&str]) -> &Value {
    Self::traverse(&self.root, path)
  }

  fn modify(&mut self, path: &[&str], f: Box<dyn FnOnce(&mut Value)>) {
    Self::traverse_mut(f, &mut self.root, path);
  }
}

impl AsPtr for Settings {}
