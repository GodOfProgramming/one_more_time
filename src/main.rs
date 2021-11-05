mod input;
mod util;
mod view;

use input::keyboard::{Key, KeyAction, Keyboard};
use log::info;
use std::{
  path::Path,
  process, thread,
  time::{Duration, Instant},
};
use util::Settings;
use view::window::{GlfwWindow, Sdl2Window, Window, WindowHandle, WindowSettings};

static SETTINGS_FILE: &str = "config/settings.toml";
const LOG_LIMIT: usize = 5;

fn main() {
  const NANOS_IN_SECS: u64 = 1_000_000_000;
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

  let fps: u64 = settings.graphics.fps.into();
  let base_sleep_time = Duration::from_nanos(NANOS_IN_SECS / fps);

  info!("target fps = {}", fps);

  let mut i = 0;
  'main: loop {
    let start = Instant::now();
    i = (i + 1) % 255;

    window.process_input(&mut keyboard);

    if keyboard.check(Key::Esc) == KeyAction::Press {
      window.close();
    }

    if window.close_requested() {
      break 'main;
    }

    // game logic

    window.bg_color((1, 64, 255 - i));

    window.clear_color();

    keyboard.new_frame();

    // render logic

    window.present();

    // calculate frame stats

    thread::sleep(base_sleep_time.saturating_sub(Instant::now() - start));
  }

  settings.save(settings_file).unwrap();
}
