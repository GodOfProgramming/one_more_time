mod gfx;
mod input;
mod math;
mod util;
mod view;

use glium::Surface;
use input::{
  keyboard::{Key, KeyAction},
  InputCheck, InputDevices,
};
use log::{debug, info};
use std::path::Path;
use util::{FpsManager, Settings};
use view::window::{Window, WindowSettings};

static SETTINGS_FILE: &str = "config/settings.toml";
const LOG_LIMIT: usize = 5;

fn main() {
  let logs = util::read_log_dir();
  let log_file = util::next_log_rotation(logs, LOG_LIMIT);

  println!("logging to {:?}", log_file);
  util::setup_logger(&log_file).unwrap();

  let settings_file = Path::new(SETTINGS_FILE);

  let settings = Settings::load(settings_file).unwrap();

  let window_settings = WindowSettings::new(&settings);

  let (window, draw_interface) = Window::new(window_settings);

  let behavior = glium::debug::DebugCallbackBehavior::Custom {
    callback: Box::new(util::gl_error_handler),
    synchronous: true,
  };

  let gl_context = unsafe { glium::backend::Context::new(draw_interface, true, behavior).unwrap() };

  let mut fps_manager = FpsManager::new(settings.graphics.fps.into());

  info!("target fps = {}", fps_manager.target());

  let mut input_devices = InputDevices::default();

  window.show();

  let mut i: f32 = 0.0;
  'main: loop {
    // frame setup
    fps_manager.begin();

    window.poll_events(&mut input_devices);

    // pre prossess game logic

    if input_devices.check(Key::Esc) == KeyAction::Press {
      break 'main;
    }

    // game logic
    i += 0.1;

    // post process game logic

    input_devices.new_frame();

    // render logic

    let mut frame = glium::Frame::new(
      gl_context.clone(),
      (settings.display.width, settings.display.height),
    );

    // draw

    frame.clear_color(i.sin(), 0.30, 1.0 - i.sin(), 1.0);

    // finalize

    frame.finish().unwrap();

    // calculate frame stats

    fps_manager.end();
  }

  settings.save(settings_file).unwrap();
}
