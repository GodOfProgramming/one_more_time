mod cam;
mod world;

use std::{fs, path::PathBuf};

use crate::{
  gfx::*,
  input::{
    keyboard::{Key, KeyAction},
    InputCheck, InputDevices,
  },
  ui::UiManager,
  util::prelude::*,
  view::window::Window,
};

use omt::{
  core::Game,
  glium::{
    self,
    backend::Context,
    debug::DebugCallbackBehavior,
    debug::{MessageType, Severity, Source},
    Surface,
  },
  imgui,
  imgui_glium_renderer::Renderer,
  puffin, puffin_imgui,
  regex::Regex,
};

pub use cam::Camera;
use world::{EntityArchive, Map, MapData};

#[derive(PartialEq)]
enum State {
  Starting,
  Exiting,
}

pub struct App {
  logger: MainLogger,
  settings: Settings,
  state: State,
}

impl App {
  pub fn new(logger: MainLogger, settings: Settings) -> Self {
    Self {
      logger,
      settings,
      state: State::Starting,
    }
  }

  pub fn run(&mut self, dirs: &Dirs, input_devices: &mut InputDevices) {
    self.logger.info("creating window".to_string());
    let (window, draw_interface) = Window::new(&mut self.settings);

    self.logger.info("initializing opengl".to_string());
    let behavior = App::create_opengl_debug_behavior(self.logger.spawn());
    let gl_context = unsafe { Context::new(draw_interface, false, behavior).unwrap() };

    self.logger.info("initializing shaders".to_string());
    let mut shader_archive = ShaderProgramArchive::default();

    self.logger.info("initializing models".to_string());
    let model_repository = ModelRepository::new(&gl_context);

    self.logger.info("initializing textures".to_string());
    let mut texture_sources = TextureSources::default();
    texture_sources.load_all(
      &self.logger,
      RecursiveDirIteratorWithID::from(&dirs.assets.textures),
    );
    let texture_repository = texture_sources.load_repository(&self.logger, &gl_context);

    self.logger.info("initializing ui".to_string());
    let mut imgui_ctx = imgui::Context::create();
    imgui_ctx.set_log_filename(None);
    let mut imgui_render = Renderer::init(&mut imgui_ctx, &gl_context.clone()).unwrap();
    window.setup_imgui(&mut imgui_ctx);

    let mut ui_manager = UiManager::new(self.logger.spawn());

    self.logger.info("initializing entities".to_string());
    let entity_repository = EntityArchive::new(self.logger.spawn());

    let mut fps_manager = FpsManager::new(self.settings.graphics.fps.into());

    self.logger.info("loading ui".to_string());
    ui_manager.load_ui(RecursiveDirIteratorWithID::from(&dirs.assets.ui));

    if cfg!(debug_assertions) {
      self.logger.info("opening debug menu".to_string());
      ui_manager.open("core.main_menu_bar", "debug_main_menu_bar");
    }

    let mut puffin_ui = puffin_imgui::ProfilerUi::default();

    self.logger.info("creating map".to_string());
    let map_data = MapData {
      width: 0,
      height: 0,
    };
    let mut map = Map::new(
      map_data,
      self.logger.spawn(),
      entity_repository.as_ptr(),
      shader_archive.as_ptr(),
      model_repository.as_ptr(),
      texture_repository.as_ptr(),
    );

    self.logger.info("spawning test character".to_string());
    map.spawn("characters.test.square");

    let mut camera = Camera::default();

    self.logger.info("showing window".to_string());
    window.show();

    let mut i: f32 = 0.0;
    'main: loop {
      puffin::GlobalProfiler::lock().new_frame();

      if self.state == State::Exiting {
        break 'main;
      }

      // frame setup

      fps_manager.begin();

      window.poll_events(input_devices, &mut imgui_ctx);

      // pre prossess game logic

      if input_devices.check(Key::Escape) == KeyAction::Press {
        break;
      }

      // game logic

      map.update();

      camera.update(&self.settings);

      i += 0.1;

      // post process game logic

      input_devices.new_frame();

      imgui_ctx.io_mut().display_size = [
        self.settings.display.window.x as f32,
        self.settings.display.window.y as f32,
      ];

      // render logic

      let mut frame = glium::Frame::new(
        gl_context.clone(),
        (
          self.settings.display.window.x,
          self.settings.display.window.y,
        ),
      );

      frame.clear_color(i.sin(), 0.30, 1.0 - i.sin(), 1.0);

      // draw objects

      map.draw_to(&mut frame, &camera);

      // draw ui

      let ui: imgui::Ui<'_> = imgui_ctx.frame();

      ui_manager.update(&ui, self);

      if self.settings.game.show_profiler {
        puffin_ui.window(&ui);
      }

      if self.settings.game.show_demo_window {
        ui.show_demo_window(&mut true);
      }

      if let Err(err) = imgui_render.render(&mut frame, ui.render()) {
        self.logger.warn(err.to_string());
      }

      // finalize

      frame.finish().unwrap();

      // calculate frame stats

      fps_manager.end();
    }

    self.settings.save().unwrap();
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

  fn load_plugins(&self) {
    let version: &str = env!("CARGO_PKG_VERSION");

    let plugin_dir = PathBuf::from("plugins");

    if let Ok(entries) = fs::read_dir(&plugin_dir) {
      for entry in entries.flatten() {
        if let Ok(meta) = entry.metadata() {
          if meta.is_dir() {
            let plugin_dir = plugin_dir.join(entry.path()).join(version);
            if let Ok(entries) = fs::read_dir(&plugin_dir) {
              for entry in entries.flatten() {
                if let Err(err) = Lib::load_lib(&entry.path(), |_| Ok(())) {
                  self
                    .logger
                    .error(format!("error loading {:?}: {}", entry, err));
                }
              }
            }
          }
        }
      }
    }
  }
}

impl Game for App {
  fn settings(&mut self) -> &mut dyn omt::util::Settings {
    &mut self.settings
  }

  fn logger(&self) -> &dyn omt::util::Logger {
    &self.logger
  }
}

impl AsPtr for App {}
