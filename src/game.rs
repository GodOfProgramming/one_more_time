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
use std::{fs, path::PathBuf, rc::Rc};

pub use cam::Camera;
use world::*;

#[derive(PartialEq)]
enum State {
  Starting,
  Exiting,
}

pub struct App {
  dirs: Dirs,
  logger: MainLogger,
  settings: Settings,
  window: Window,
  texture_archive: TextureArchive,
  shader_archive: ShaderProgramArchive,
  input_devices: InputDevices,
  state: State,
}

impl App {
  pub fn new(dirs: Dirs, logger: MainLogger, settings: Settings, window: Window) -> Self {
    Self {
      dirs,
      logger,
      settings,
      window,
      texture_archive: Default::default(),
      shader_archive: Default::default(),
      input_devices: Default::default(),
      state: State::Starting,
    }
  }

  pub fn run(mut self, context: Rc<Context>) {
    self.logger.info("initializing models".to_string());
    let model_repository = ModelRepository::new(&context);

    self.logger.info("initializing imgui".to_string());
    let mut imgui_ctx = imgui::Context::create();
    imgui_ctx.set_log_filename(None);
    let mut imgui_render = Renderer::init(&mut imgui_ctx, &context.clone()).unwrap();
    self.window.setup_imgui(&mut imgui_ctx);

    self.logger.info("initializing ui manager".to_string());
    let mut ui_manager = UiManager::new(self.logger.spawn());

    self.logger.info("initializing entities".to_string());
    let mut entity_archive = EntityArchive::new(self.logger.spawn());

    let mut fps_manager = FpsManager::new(self.settings.graphics.fps.into());

    let mut puffin_ui = ProfilerUi::default();

    let mut camera = Camera::default();

    self.load_plugins(&mut ui_manager, context.clone(), &mut entity_archive);

    self.logger.info("opening debug menu".to_string());
    ui_manager.open("core.main_menu_bar", "debug_main_menu_bar");

    self.logger.info("creating map".to_string());
    let map_data = MapData {
      width: 0,
      height: 0,
    };

    let mut map = Map::new(
      map_data,
      self.logger.spawn(),
      entity_archive.as_ptr(),
      self.shader_archive.as_ptr(),
      model_repository.as_ptr(),
      self.texture_archive.as_ptr(),
    );

    map.spawn("test");

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

      // pre process game logic

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

  fn load_plugins(
    &mut self,
    ui_manager: &mut UiManager,
    ctx: Rc<Context>,
    entity_archive: &mut EntityArchive,
  ) {
    let version: String = format!("v{}", env!("CARGO_PKG_VERSION"));

    if let Ok(entries) = fs::read_dir(&self.dirs.plugins) {
      for entry in entries.flatten() {
        self.logger.debug(format!("checking plugin {:?}", entry));
        if let Ok(meta) = entry.metadata() {
          if meta.is_dir() {
            let plugin_dir = entry.path().join(&version);
            self
              .logger
              .debug(format!("checking version dir {:?}", plugin_dir));
            if let Ok(entries) = fs::read_dir(&plugin_dir) {
              for entry in entries.flatten() {
                self.logger.debug(format!("checking file {:?}", entry));
                if let Some(extension) = entry.path().extension().and_then(std::ffi::OsStr::to_str)
                {
                  // todo get string based on os
                  if extension != "dll" {
                    continue;
                  }

                  self.logger.debug("dll found, loading".to_string());

                  let result = Lib::load_lib(&entry.path(), |lib: &mut Library| {
                    self.load_plugin(
                      plugin_dir.clone(),
                      lib,
                      ui_manager,
                      ctx.clone(),
                      entity_archive,
                    )
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

  fn load_plugin(
    &mut self,
    path: PathBuf,
    lib: &mut Library,
    ui_manager: &mut UiManager,
    ctx: Rc<Context>,
    entity_archive: &mut EntityArchive,
  ) -> Result<(), libloading::Error> {
    unsafe {
      let loader = lib.get::<PluginLoadFn>(b"exports")?;
      let mut module = Mod::new(path, self.logger.spawn());
      if loader(&mut module).is_ok() {
        self.add_mod(module, ui_manager, ctx, entity_archive);
      }
    }
    Ok(())
  }

  fn add_mod(
    &mut self,
    module: Mod,
    ui_manager: &mut UiManager,
    ctx: Rc<Context>,
    entity_archive: &mut EntityArchive,
  ) {
    ui_manager.add_model_archive(module.ui_models);
    ui_manager.add_template_archive(module.ui_sources);
    // todo check results
    let _ = self.texture_archive.add_image_archive(module.images, &ctx);
    self
      .shader_archive
      .add_source_archive(&self.logger, module.shaders, ctx);
    entity_archive.add_model_archive(module.entity_models);
  }
}

impl Game for App {
  fn settings(&mut self) -> &mut dyn omt::util::Settings {
    &mut self.settings
  }

  fn logger(&self) -> &dyn omt::util::Logger {
    &self.logger
  }

  fn exit(&mut self) {
    self.state = State::Exiting;
  }
}

impl AsPtr for App {}

struct Mod {
  path: PathBuf,
  logger: ChildLogger,
  images: ImageArchive,
  shaders: ShaderSourceArchive,
  ui_models: UiModelArchive,
  ui_sources: UiTemplateSourceArchive,
  entity_models: EntityModelArchive,
}

impl Mod {
  fn new(path: PathBuf, logger: ChildLogger) -> Self {
    Self {
      path,
      logger,
      images: Default::default(),
      shaders: Default::default(),
      ui_models: Default::default(),
      ui_sources: Default::default(),
      entity_models: Default::default(),
    }
  }
}

impl Plugin for Mod {
  fn path(&self) -> std::path::PathBuf {
    self.path.clone()
  }

  fn logger(&self) -> &dyn Logger {
    &self.logger
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
