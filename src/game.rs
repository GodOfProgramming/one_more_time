mod cam;
mod world;

use crate::{
  gfx::*,
  glium::{self, backend::Context, Surface},
  imgui,
  input::{
    keyboard::{Key, KeyAction},
    InputCheck, InputDevices,
  },
  ui::*,
  util::prelude::*,
  view::window::Window,
};
use imgui_glium_renderer::Renderer;
use libloading::Library;
use omt::{core::Game, Plugin, PluginLoadFn};
use puffin_imgui::ProfilerUi;
use std::{
  fs,
  path::{Path, PathBuf},
  rc::Rc,
};

pub use cam::Camera;
use world::*;

#[derive(PartialEq)]
enum State {
  Starting,
  Exiting,
}

pub struct App {
  logger: MainLogger,
  settings: Settings,
  window: Window,
  shader_archive: ShaderProgramArchive,
  input_devices: InputDevices,
  state: State,
}

impl App {
  pub fn new(logger: MainLogger, settings: Settings, window: Window) -> Self {
    Self {
      logger,
      settings,
      window,
      shader_archive: Default::default(),
      input_devices: Default::default(),
      state: State::Starting,
    }
  }

  pub fn run(mut self, dirs: Dirs, context: Rc<Context>) {
    self.logger.info("initializing models".to_string());
    let model_repository = ModelRepository::new(&context);

    self.logger.info("initializing textures".to_string());
    let texture_archive = TextureArchive::default();

    self.logger.info("initializing imgui".to_string());
    let mut imgui_ctx = imgui::Context::create();
    imgui_ctx.set_log_filename(None);
    let mut imgui_render = Renderer::init(&mut imgui_ctx, &context.clone()).unwrap();
    self.window.setup_imgui(&mut imgui_ctx);

    self.logger.info("initializing ui manager".to_string());
    let mut ui_manager = UiManager::new(self.logger.spawn());

    self.logger.info("initializing entities".to_string());
    let entity_repository = EntityArchive::new(self.logger.spawn());

    let mut fps_manager = FpsManager::new(self.settings.graphics.fps.into());

    let mut puffin_ui = ProfilerUi::default();

    let mut camera = Camera::default();

    self.load_plugins();

    self.logger.info("opening debug menu".to_string());
    // ui_manager.open("core.main_menu_bar", "debug_main_menu_bar");

    self.logger.info("creating map".to_string());
    let map_data = MapData {
      width: 0,
      height: 0,
    };

    let mut map = Map::new(
      map_data,
      self.logger.spawn(),
      entity_repository.as_ptr(),
      self.shader_archive.as_ptr(),
      model_repository.as_ptr(),
      texture_archive.as_ptr(),
    );

    // self.logger.info("spawning test character".to_string());
    // map.spawn("main.characters.test.square");

    self.logger.info("showing window".to_string());
    self.window.show();

    let mut i: f32 = 0.0;
    'main: loop {
      puffin::GlobalProfiler::lock().new_frame();

      if self.state == State::Exiting {
        break 'main;
      }

      // frame setup

      fps_manager.begin();

      self
        .window
        .poll_events(&mut self.input_devices, &mut imgui_ctx);

      // pre prossess game logic

      if self.input_devices.check(Key::Escape) == KeyAction::Press {
        break;
      }

      // game logic

      map.update();

      camera.update(&self.settings);

      i += 0.1;

      // post process game logic

      self.input_devices.new_frame();

      imgui_ctx.io_mut().display_size = [
        self.settings.display.window.x as f32,
        self.settings.display.window.y as f32,
      ];

      // render logic

      let mut frame = glium::Frame::new(
        context.clone(),
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

      ui_manager.update(&ui, &mut self);

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

  fn load_plugins(&self) {
    let version: &str = env!("CARGO_PKG_VERSION");

    let plugin_dir = PathBuf::from("plugins");

    if let Ok(entries) = fs::read_dir(&plugin_dir) {
      for entry in entries.flatten() {
        if let Some(extension) = entry.path().extension().and_then(std::ffi::OsStr::to_str) {
          // todo get string based on os
          if extension != "dll" {
            continue;
          }

          if let Ok(meta) = entry.metadata() {
            if meta.is_dir() {
              let plugin_dir = plugin_dir.join(entry.path()).join(version);
              if let Ok(entries) = fs::read_dir(&plugin_dir) {
                for entry in entries.flatten() {
                  let result = Lib::load_lib(&entry.path(), |path: PathBuf, lib: &mut Library| {
                    self.load_plugin(path, lib)
                  });
                  if let Err(err) = result {
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

  fn load_plugin(&self, path: PathBuf, lib: &mut Library) -> Result<(), libloading::Error> {
    unsafe {
      let loader = lib.get::<PluginLoadFn>(b"exports")?;
      let mut module = Mod::new(path);
      if loader(&mut module).is_ok() {
        self.add_mod(module);
      }
    }
    Ok(())
  }

  fn add_mod(&self, module: Mod) {}
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

#[derive(Default)]
struct Mod {
  path: PathBuf,
  images: ImageArchive,
  shaders: ShaderSourceArchive,
  ui_models: UiModelArchive,
  ui_sources: UiTemplateSourceArchive,
  entity_models: EntityModelArchive,
}

impl Mod {
  fn new(path: PathBuf) -> Self {
    Self {
      path,
      ..Default::default()
    }
  }
}

impl Plugin for Mod {
  fn path(&self) -> std::path::PathBuf {
    self.path.clone()
  }

  fn textures(&mut self) -> &mut dyn omt::gfx::TextureLoader {
    &mut self.images
  }

  fn shaders(&mut self) -> &mut dyn omt::gfx::ShaderLoader {
    &mut self.shaders
  }

  fn ui_models(&mut self) -> &mut dyn omt::ui::UiModelLoader {
    &mut self.ui_models
  }

  fn ui_sources(&mut self) -> &mut dyn omt::ui::UiSourceLoader {
    &mut self.ui_sources
  }

  fn entity_models(&mut self) -> &mut dyn omt::core::EntityModelLoader {
    &mut self.entity_models
  }
}
