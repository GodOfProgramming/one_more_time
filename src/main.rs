mod gfx;
mod input;
mod math;
mod ui;
mod util;
mod view;

use gfx::{ShaderRepository, ShaderSources};
use glium::Surface;
use imgui_glium_renderer::imgui;
use input::{
  keyboard::{Key, KeyAction},
  InputCheck, InputDevices,
};
use log::info;
use std::{ops::Deref, path::Path};
use ui::{UiElement, UiRoot};
use util::{FpsManager, Settings, XmlNode};
use view::window::{Window, WindowSettings};

static SETTINGS_FILE: &str = "config/settings.toml";
const LOG_LIMIT: usize = 5;

fn main() {
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

  util::setup_logger(LOG_LIMIT).unwrap();

  let settings_file = Path::new(SETTINGS_FILE);

  let settings = Settings::load(settings_file).unwrap();

  let window_settings = WindowSettings::new(&settings);

  let (window, draw_interface) = Window::new(window_settings);

  let behavior = glium::debug::DebugCallbackBehavior::Custom {
    callback: Box::new(util::gl_error_handler),
    synchronous: true,
  };

  let gl_context = unsafe { glium::backend::Context::new(draw_interface, true, behavior).unwrap() };

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

  info!("target fps = {}", fps_manager.target());

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

    for element in &mut elements {
      element.update(&ui, &settings);
    }

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
