use crate::{
  util::{ChildLogger, Dirs, MainLogger, Settings, SpawnableLogger},
  view::window::Window,
};
use game::App;
use glium::{
  backend::Context,
  debug::DebugCallbackBehavior,
  debug::{MessageType, Severity, Source},
};
use imgui_glium_renderer::{glium, imgui};
use omt::util::Logger;
use std::{env, path::Path};

mod game;
mod gfx;
mod input;
mod math;
mod ui;
mod util;
mod view;

static SETTINGS_FILE: &str = "settings.toml";
const LOG_LIMIT: usize = 5;

fn main() {
  puffin::set_scopes_on(cfg!(debug_assertions));

  let logger = MainLogger::new(LOG_LIMIT);

  let cwd = env::current_dir().unwrap(); // unwrap because there's bigger problems if this doesn't work
  let dirs = Dirs::new(cwd);
  let settings_file = Path::new(SETTINGS_FILE);
  let mut settings = Settings::load(settings_file).unwrap();

  logger.info("creating window".to_string());
  let (window, draw_interface) = Window::new(&mut settings);

  logger.info("initializing opengl".to_string());
  let behavior = create_opengl_debug_behavior(logger.spawn());
  let gl_context = unsafe { Context::new(draw_interface, false, behavior).unwrap() };

  logger.info("creating app".to_string());
  let app = App::new(dirs, logger, settings, window);

  app.run(gl_context);
}

fn create_opengl_debug_behavior(child_logger: ChildLogger) -> DebugCallbackBehavior {
  DebugCallbackBehavior::Custom {
    synchronous: true,
    callback: Box::new(
      move |source: Source,
            message_type: MessageType,
            severity: Severity,
            _ident: u32,
            handled: bool,
            message: &str| {
        match severity {
          glium::debug::Severity::Notification => {
            child_logger.trace(format!(
              "OpenGL Notification: source = {:?}, message type = {:?}, handled = {:?} -> {}",
              source, message_type, handled, message
            ));
          }
          glium::debug::Severity::Low => {
            child_logger.info(format!(
              "OpenGL Warning: source = {:?}, message type = {:?}, handled = {:?} -> {}",
              source, message_type, handled, message
            ));
          }
          glium::debug::Severity::Medium => {
            child_logger.warn(format!(
              "OpenGL Warning: source = {:?}, message type = {:?}, handled = {:?} -> {}",
              source, message_type, handled, message
            ));
          }
          glium::debug::Severity::High => {
            child_logger.error(format!(
              "OpenGL Error: source = {:?}, message type = {:?}, handled = {:?} -> {}",
              source, message_type, handled, message
            ));
            std::process::exit(1);
          }
        }
      },
    ),
  }
}
