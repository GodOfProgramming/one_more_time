mod world;

use crate::{
  gfx::*,
  input::{
    keyboard::{Key, KeyAction},
    InputCheck, InputDevices,
  },
  scripting::{LuaType, LuaTypeTrait, ScriptRepository},
  ui::UiManager,
  util::{
    ChildLogger, Dirs, FpsManager, Logger, RecursiveDirIteratorWithID, Settings, SpawnableLogger,
  },
  view::window::Window,
};
use imgui_glium_renderer::glium::{
  self,
  backend::Context,
  debug::DebugCallbackBehavior,
  debug::{MessageType, Severity, Source},
  uniform, Surface,
};
use imgui_glium_renderer::imgui;
use mlua::{Lua, UserData, UserDataMethods, Value};

#[derive(PartialEq)]
enum State {
  Starting,
  InGame,
  Paused,
  Exiting,
}

pub struct App {
  logger: ChildLogger,
  state: State,
}

impl App {
  pub fn new(logger: ChildLogger) -> Self {
    Self {
      logger,
      state: State::Starting,
    }
  }

  pub fn run(
    &mut self,
    settings: &mut Settings,
    dirs: &Dirs,
    input_devices: &mut InputDevices,
    scripts: &mut ScriptRepository,
  ) {
    // window
    let (window, draw_interface) = Window::new(settings);

    // opengl
    let behavior = App::create_opengl_debug_behavior(self.logger.spawn());
    let gl_context = unsafe { Context::new(draw_interface, false, behavior).unwrap() };

    // shaders
    let mut shaders = ShaderSources::new();
    shaders.load_all(
      &self.logger,
      RecursiveDirIteratorWithID::from(&dirs.assets.cfg.shaders),
    );
    let shader_repository = shaders.load_repository(&gl_context, &self.logger);

    // textures
    let mut texture_sources = TextureSources::default();
    texture_sources.load_all(
      &self.logger,
      RecursiveDirIteratorWithID::from(&dirs.assets.textures),
    );
    let texture_repository = texture_sources.load_repository(&self.logger, &gl_context);

    // ui
    let mut imgui_ctx = imgui_glium_renderer::imgui::Context::create();
    imgui_ctx.set_log_filename(None);
    let mut imgui_render =
      imgui_glium_renderer::Renderer::init(&mut imgui_ctx, &gl_context.clone()).unwrap();
    window.setup_imgui(&mut imgui_ctx);

    let mut ui_manager = UiManager::new(
      &self.logger,
      RecursiveDirIteratorWithID::from(&dirs.assets.ui),
      scripts,
    );
    let lua_ui_manager = ui_manager.create_lua_type();

    scripts.register_init_fn(Box::new(move |lua: &mut Lua| {
      let globals = lua.globals();
      let _ = globals.set("UiManager", lua_ui_manager);
    }));

    // game
    let mut fps_manager = FpsManager::new(settings.graphics.fps.into());

    scripts.load_scripts(&self.logger);

    ui_manager.open("core.main_menu_bar", "debug_main_menu_bar", Value::Nil);

    let mut puffin_ui = puffin_imgui::ProfilerUi::default();

    /* =============================================================================================== */

    let test_obj = Square::new();
    let test_obj_vbuff = glium::VertexBuffer::new(&gl_context, &test_obj.vertices).unwrap();
    let test_obj_ibuff = glium::index::IndexBuffer::new(
      &gl_context,
      glium::index::PrimitiveType::TrianglesList,
      &test_obj.indices,
    )
    .unwrap();
    let test_obj_shader = shader_repository.get("test.basic").unwrap();

    let grass_tex = texture_repository.get("grass").unwrap();

    let test_obj_uniforms = uniform! {
      tex: grass_tex,
    };

    let test_obj_params = glium::DrawParameters::default();

    /* =============================================================================================== */

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

      frame
        .draw(
          &test_obj_vbuff,
          &test_obj_ibuff,
          test_obj_shader,
          &test_obj_uniforms,
          &test_obj_params,
        )
        .unwrap();

      // draw ui

      let ui: imgui::Ui<'_> = imgui_ctx.frame();

      ui_manager.update(&ui, settings);

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
}

impl LuaType<App> {
  fn request_exit(&mut self) {
    self.obj_mut().state = State::Exiting;
  }
}

impl LuaTypeTrait for App {}

impl UserData for LuaType<App> {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method_mut("request_exit", |_, this, _: ()| {
      this.request_exit();
      Ok(())
    });
  }
}
