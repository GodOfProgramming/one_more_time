mod input;
mod util;
mod view;

use input::keyboard::{Key, KeyAction, Keyboard};
use log::info;
use std::{
  path::Path,
  thread,
  time::{Duration, Instant},
};
use util::{FpsManager, Settings};
use view::window::{GlfwWindow, Sdl2Window, Window, WindowHandle, WindowSettings};

static SETTINGS_FILE: &str = "config/settings.toml";
const LOG_LIMIT: usize = 5;

fn main() {
  type WindowApi = Sdl2Window;

  let logs = util::read_log_dir();
  let log_file = util::next_log_rotation(logs, LOG_LIMIT);
  println!("logging to {:?}", log_file);
  util::setup_logger(&log_file).unwrap();

  let settings_file = Path::new(SETTINGS_FILE);

  let mut keyboard = Keyboard::new();

  let settings = Settings::load(settings_file).unwrap();

  let window_settings = WindowSettings::new(&settings);

  let handle = WindowApi::new(window_settings);

  let mut window = Window::new(handle);

  let mut fps_manager = FpsManager::new(settings.graphics.fps.into());

  info!("target fps = {}", fps_manager.target());

  let mut i = 0;
  'main: loop {
    fps_manager.begin();

    i = (i + 1) % 255;

    window.process_input(&mut keyboard);

    if keyboard.check(Key::Esc) == KeyAction::Press {
      window.close();
    }

    if window.close_requested() {
      break 'main;
    }

    // game logic

    window.bg_color((i, 64, 255 - i));

    window.clear_color();

    keyboard.new_frame();

    // render logic

    window.present();

    // calculate frame stats

    fps_manager.end();
  }

  settings.save(settings_file).unwrap();
}
