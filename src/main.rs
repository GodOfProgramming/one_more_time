mod input;
mod util;
mod view;

use input::keyboard::{Key, KeyAction, Keyboard};
use std::{path::Path, process};
use util::Settings;
use view::window::{glfw_window, GlfwWindow, Window, WindowHandle};

static SETTINGS_FILE: &str = "settings.toml";

fn main() {
  type WindowApi = GlfwWindow;
  type WindowSettings = glfw_window::WindowSettings;

  let settings_file = Path::new(SETTINGS_FILE);

  let mut keyboard = Keyboard::new();

  let settings = Settings::load(settings_file).unwrap();

  let window_settings = WindowSettings::new(&settings);

  let handle = WindowApi::new(window_settings);

  let mut window = Window::new(handle);

  if !window.open() {
    println!("failed to open window");
    process::exit(1);
  }

  while !window.close_requested() {
    window.process_input(&mut keyboard);

    if keyboard.check(Key::Esc) == KeyAction::Press {
      window.close();
    }
  }

  settings.save(settings_file).unwrap();
}
