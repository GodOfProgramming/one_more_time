mod cam;
mod world;

use std::fs;

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
  glium::{
    self,
    backend::Context,
    debug::DebugCallbackBehavior,
    debug::{MessageType, Severity, Source},
    Surface,
  },
  imgui,
  imgui_glium_renderer::Renderer,
  mlua::{prelude::*, UserData, UserDataMethods},
  puffin, puffin_imgui,
  regex::Regex,
};

pub use cam::Camera;
use world::{EntityRepository, Map, MapData};

#[derive(PartialEq)]
enum State {
  Starting,
  Exiting,
}

pub struct App {
  logger: MainLogger,
  state: State,
}

impl App {
  pub fn new(logger: MainLogger) -> Self {
    Self {
      logger,
      state: State::Starting,
    }
  }

  pub fn run(&mut self, settings: &mut Settings, dirs: &Dirs, input_devices: &mut InputDevices) {
    self.logger.info("creating window".to_string());
    let (window, draw_interface) = Window::new(settings);

    self.logger.info("initializing opengl".to_string());
    let behavior = App::create_opengl_debug_behavior(self.logger.spawn());
    let gl_context = unsafe { Context::new(draw_interface, false, behavior).unwrap() };

    self.logger.info("initializing shaders".to_string());
    let mut shaders = ShaderSources::new();
    shaders.load_all(
      &self.logger,
      RecursiveDirIteratorWithID::from(&dirs.assets.cfg.shaders),
    );
    let shader_repository = shaders.load_repository(&gl_context, &self.logger);

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
    let entity_repository = EntityRepository::new(
      self.logger.spawn(),
      RecursiveDirIteratorWithID::from(&dirs.assets.cfg.entities),
    );

    let mut fps_manager = FpsManager::new(settings.graphics.fps.into());

    self.logger.info("initializing lua".to_string());
    let lua = self.initialize_lua(dirs, settings, &mut ui_manager);

    self.logger.info("loading ui".to_string());
    ui_manager.load_ui(RecursiveDirIteratorWithID::from(&dirs.assets.ui), lua);

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
      lua,
      entity_repository.as_ptr(),
      shader_repository.as_ptr(),
      model_repository.as_ptr(),
      texture_repository.as_ptr(),
    );

    map.register_to_lua(lua);

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

      map.update(&self.logger);

      camera.update(settings);

      i += 0.1;

      // post process game logic

      input_devices.new_frame();

      imgui_ctx.io_mut().display_size = [
        settings.display.window.x as f32,
        settings.display.window.y as f32,
      ];

      // render logic

      let mut frame = glium::Frame::new(
        gl_context.clone(),
        (settings.display.window.x, settings.display.window.y),
      );

      frame.clear_color(i.sin(), 0.30, 1.0 - i.sin(), 1.0);

      // draw objects

      map.draw_to(&mut frame, &camera);

      // draw ui

      let ui: imgui::Ui<'_> = imgui_ctx.frame();

      ui_manager.update(&self.logger, &ui, lua, settings);

      if settings.game.show_profiler {
        puffin_ui.window(&ui);
      }

      if settings.game.show_demo_window {
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

  fn initialize_lua(
    &mut self,
    dirs: &Dirs,
    settings: &mut Settings,
    ui_manager: &mut UiManager,
  ) -> &'static Lua {
    let lua = Lua::new();

    {
      let globals = lua.globals();

      let dirs_ptr = dirs.as_ptr();

      let package: LuaTable = globals.get("package").unwrap();
      let path: String = package.get("path").unwrap();
      let path = format!(
        "{}/?.lua;{}",
        dirs_ptr.assets.scripts.to_str().unwrap(),
        path
      );
      package.set("path", path).unwrap();

      let logger_ptr = self.logger.as_ptr();
      let app_ptr = self.as_ptr_mut();
      let settings_ptr = settings.as_ptr_mut();
      let ui_manager_ptr = ui_manager.as_ptr_mut();
      let _ = globals.set("App", app_ptr);
      let _ = globals.set("Logger", logger_ptr);
      let _ = globals.set("Settings", settings_ptr);
      let _ = globals.set("UiManager", ui_manager_ptr);

      for (file, id) in RecursiveDirIteratorWithID::from(&dirs.assets.scripts) {
        if settings
          .scripts
          .exclude
          .iter()
          .any(|reg: &Regex| reg.is_match(&id))
        {
          continue;
        }

        if let Ok(src) = fs::read_to_string(file) {
          lua.load(&src).exec().unwrap();
        }
      }
    }

    lua.into_static()
  }
}

impl AsPtr for App {}

impl UserData for MutPtr<App> {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method_mut("request_exit", |_, this, _: ()| {
      this.state = State::Exiting;
      Ok(())
    });
  }
}
