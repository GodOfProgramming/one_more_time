use crate::gfx::{ShaderRepository, ShaderSources};
use crate::input::{
  keyboard::{Key, KeyAction},
  InputCheck, InputDevices,
};
use crate::ui::{UiElement, UiManager, UiRoot};
use crate::util::{
  self, ChildLogger, Dirs, FpsManager, Logger, MainLogger, RecursiveDirectoryIterator, Settings,
  SpawnableLogger, XmlNode,
};
use crate::view::window::{Window, WindowSettings};
use glium::debug::DebugCallbackBehavior;
use glium::debug::{MessageType, Severity, Source};
use glium::Surface;
use imgui_glium_renderer::imgui;
use log::info;
use std::{env, ops::Deref, path::Path};

static SETTINGS_FILE: &str = "config/settings.toml";
const LOG_LIMIT: usize = 5;

pub struct App {
  logger: MainLogger,
}

impl App {
  pub fn new() -> Self {
    let logger = MainLogger::new();

    Self { logger }
  }

  pub fn run(&mut self) {
    /////////////////////////
    let test_window_xml = std::fs::read_to_string("assets/ui/test_window.xml").unwrap();
    let test_bar_xml = std::fs::read_to_string("assets/ui/test_bar.xml").unwrap();

    let test_window_nodes = XmlNode::parse(&test_window_xml).unwrap();
    let test_bar_nodes = XmlNode::parse(&test_bar_xml).unwrap();

    let mut elements = Vec::new();

    for node in test_window_nodes {
      elements.push(UiRoot::from(node));
    }

    for node in test_bar_nodes {
      elements.push(UiRoot::from(node));
    }

    /////////////////////////

    let cwd = env::current_dir().unwrap(); // unwrap because there's bigger problems if this doesn't work

    let dirs = Dirs::new(cwd);

    let settings_file = Path::new(SETTINGS_FILE);

    let settings = Settings::load(settings_file).unwrap();

    let window_settings = WindowSettings::new(&settings);

    let (window, draw_interface) = Window::new(window_settings);

    let child_logger = self.logger.spawn();
    let behavior = App::create_opengl_debug_behavior(child_logger);

    let gl_context =
      unsafe { glium::backend::Context::new(draw_interface, true, behavior).unwrap() };

    let mut shaders = ShaderSources::new();
    shaders.load_all();

    let shader_repository = shaders.load_repository(&gl_context);

    let mut imgui_ctx = imgui_glium_renderer::imgui::Context::create();
    imgui_ctx.set_ini_filename(None);
    imgui_ctx.set_log_filename(None);

    let mut imgui_render =
      imgui_glium_renderer::Renderer::init(&mut imgui_ctx, &gl_context.clone()).unwrap();

    let mut input_devices = InputDevices::default();

    window.setup_imgui(&mut imgui_ctx);

    let mut fps_manager = FpsManager::new(settings.graphics.fps.into());

    let mut ui_manager = UiManager::new(RecursiveDirectoryIterator::from(&dirs.assets.ui));

    self
      .logger
      .info(format!("target fps = {}", fps_manager.target()));

    window.show();

    let mut i: f32 = 0.0;
    'main: loop {
      // frame setup

      fps_manager.begin();

      window.poll_events(&mut input_devices, &mut imgui_ctx);

      // pre prossess game logic

      if input_devices.check(Key::Escape) == KeyAction::Press {
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

      imgui_ctx.io_mut().display_size = [
        settings.display.width as f32,
        settings.display.height as f32,
      ];

      let ui: imgui::Ui<'_> = imgui_ctx.frame();

      imgui::Window::new("Hello world")
        .size([300.0, 100.0], imgui::Condition::FirstUseEver)
        .build(&ui, || {
          ui.text("Hello world!");
          ui.text("こんにちは世界！");
          ui.text("This...is...imgui-rs!");
          ui.separator();
          let mouse_pos = ui.io().mouse_pos;
          ui.text(format!(
            "Mouse Position: ({:.1},{:.1})",
            mouse_pos[0], mouse_pos[1]
          ));
        });

      ui_manager.update(&ui, &settings);

      ui.show_demo_window(&mut true);

      // finalize

      let draw_data = ui.render();

      imgui_render.render(&mut frame, draw_data).unwrap();

      frame.finish().unwrap();

      // calculate frame stats

      fps_manager.end();
    }

    settings.save(settings_file).unwrap();
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
