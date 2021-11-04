mod input;
mod util;
mod view;

use input::keyboard::{Key, KeyAction, Keyboard};
use std::{path::Path, process, thread, time::Duration};
use util::Settings;
use view::window::{GlfwWindow, Sdl2Window, Window, WindowHandle, WindowSettings};

static SETTINGS_FILE: &str = "config/settings.toml";

fn main() {
  type WindowApi = Sdl2Window;

  let settings_file = Path::new(SETTINGS_FILE);

  let mut keyboard = Keyboard::new();

  let settings = Settings::load(settings_file).unwrap();

  let window_settings = WindowSettings::new(&settings);

  let handle = WindowApi::new(window_settings);

  let mut window = Window::new(handle);

  let mut i = 0;
  'main: loop {
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

    thread::sleep(Duration::from_nanos(16666666));
  }

  settings.save(settings_file).unwrap();
}
