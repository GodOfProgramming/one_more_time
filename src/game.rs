use crate::{
  gfx::ShaderSources,
  input::{
    keyboard::{Key, KeyAction},
    InputCheck, InputDevices,
  },
  scripting::{LuaType, LuaTypeTrait, ScriptRepository},
  ui::UiManager,
  util::{
    ChildLogger, Dirs, FpsManager, Logger, MainLogger, RecursiveDirIterator,
    RecursiveDirIteratorWithID, Settings, SpawnableLogger,
  },
  view::window::{Window, WindowSettings},
};
use glium::backend::Context;
use glium::debug::DebugCallbackBehavior;
use glium::debug::{MessageType, Severity, Source};
use glium::Surface;
use imgui_glium_renderer::imgui;
use mlua::{LightUserData, UserData, UserDataFields, UserDataMethods, Value};
use std::{
  env,
  path::Path,
  sync::mpsc::{Receiver, Sender},
};

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
  message_sender: Sender<String>,
  message_receiver: Receiver<String>,
}

impl App {
  pub fn new(
    logger: ChildLogger,
    message_sender: Sender<String>,
    message_receiver: Receiver<String>,
  ) -> Self {
    Self {
      logger,
      state: State::Starting,
      message_sender,
      message_receiver,
    }
  }

  pub fn run(
    &mut self,
    settings: &Settings,
    dirs: &Dirs,
    input_devices: &mut InputDevices,
    scripts: &mut ScriptRepository,
  ) {
    // window
    let window_settings = WindowSettings::new(settings);
    let (window, draw_interface) = Window::new(window_settings);

    // opengl
    let behavior = App::create_opengl_debug_behavior(self.logger.spawn());
    let gl_context = unsafe { Context::new(draw_interface, true, behavior).unwrap() };

    // shaders
    let mut shaders = ShaderSources::new();
    shaders.load_all(dirs);
    let shader_repository = shaders.load_repository(&gl_context);

    // textures

    // ui
    let mut imgui_ctx = imgui_glium_renderer::imgui::Context::create();
    imgui_ctx.set_ini_filename(None);
    imgui_ctx.set_log_filename(None);
    let mut imgui_render =
      imgui_glium_renderer::Renderer::init(&mut imgui_ctx, &gl_context.clone()).unwrap();
    window.setup_imgui(&mut imgui_ctx);

    let mut ui_manager = UiManager::new(
      &self.logger,
      RecursiveDirIteratorWithID::from(&dirs.assets.ui),
      scripts,
    );

    // game
    let mut fps_manager = FpsManager::new(settings.graphics.fps.into());

    self
      .logger
      .info(format!("target fps = {}", fps_manager.target()));

    scripts.load_scripts(&self.logger);

    ui_manager.open("test_bar", Value::Nil);

    window.show();

    let mut i: f32 = 0.0;
    'main: loop {
      if self.state == State::Exiting {
        break 'main;
      }

      // frame setup

      fps_manager.begin();

      while let Ok(msg) = self.message_receiver.try_recv() {
        if msg == "quit" {
          self.state = State::Exiting;
        }
      }

      window.poll_events(input_devices, &mut imgui_ctx);

      // pre prossess game logic

      if input_devices.check(Key::Escape) == KeyAction::Press {
        break;
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

      imgui_ctx.io_mut().display_size = [
        settings.display.width as f32,
        settings.display.height as f32,
      ];

      let ui: imgui::Ui<'_> = imgui_ctx.frame();

      ui_manager.update(&ui, settings);

      ui.show_demo_window(&mut true);

      // finalize

      let draw_data = ui.render();

      imgui_render.render(&mut frame, draw_data).unwrap();

      frame.finish().unwrap();

      // calculate frame stats

      fps_manager.end();
    }
  }

  pub fn get_msg_sender(&self) -> Sender<String> {
    self.message_sender.clone()
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

impl LuaTypeTrait for App {}

impl LuaType<App> {
  fn request_exit(&mut self) {
    self.obj_mut().state = State::Exiting;
  }
}

impl UserData for LuaType<App> {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method_mut("request_exit", |_, this, _: ()| {
      this.request_exit();
      Ok(())
    });
  }
}

impl UserData for App {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method_mut("request_exit", |_, this, _: ()| {
      this.state = State::Exiting;
      Ok(())
    });
  }
}
