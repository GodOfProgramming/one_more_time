use super::{WindowHandle, WindowSettings};
use crate::input::keyboard::{Key, KeyAction, KeyEvent, Keyboard};
use glfw::{Glfw, OpenGlProfileHint, Window, WindowEvent, WindowHint};
use std::sync::mpsc::Receiver;

pub struct GlfwWindow {
  _glfw_handle: Glfw,
  window_handle: Window,
  event_stream: Receiver<(f64, WindowEvent)>,
  _width: u32,
  _height: u32,
}

impl GlfwWindow {
  pub fn new(settings: WindowSettings) -> Self {
    let mut glfw_handle = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw_handle.window_hint(WindowHint::ContextVersionMajor(3));
    glfw_handle.window_hint(WindowHint::ContextVersionMinor(3));
    glfw_handle.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw_handle.window_hint(WindowHint::Resizable(false));
    glfw_handle.window_hint(WindowHint::Visible(false));
    glfw_handle.window_hint(WindowHint::DoubleBuffer(true));
    glfw_handle.window_hint(WindowHint::ContextNoError(true));

    let (window_handle, event_stream) = glfw_handle
      .create_window(
        settings.dimentions.x,
        settings.dimentions.y,
        &settings.title,
        glfw::WindowMode::Windowed,
      )
      .unwrap();

    Self {
      _glfw_handle: glfw_handle,
      window_handle,
      event_stream,
      _width: settings.dimentions.x,
      _height: settings.dimentions.y,
    }
  }

  fn convert_key_event(key: glfw::Key, action: glfw::Action) -> KeyEvent {
    let key = match key {
      glfw::Key::A => Key::A,
      glfw::Key::B => Key::B,
      glfw::Key::C => Key::C,
      glfw::Key::D => Key::D,
      glfw::Key::E => Key::E,
      glfw::Key::F => Key::F,
      glfw::Key::G => Key::G,
      glfw::Key::H => Key::H,
      glfw::Key::I => Key::I,
      glfw::Key::J => Key::J,
      glfw::Key::K => Key::K,
      glfw::Key::L => Key::L,
      glfw::Key::M => Key::M,
      glfw::Key::N => Key::N,
      glfw::Key::O => Key::O,
      glfw::Key::P => Key::P,
      glfw::Key::Q => Key::Q,
      glfw::Key::R => Key::R,
      glfw::Key::S => Key::S,
      glfw::Key::T => Key::T,
      glfw::Key::U => Key::U,
      glfw::Key::V => Key::V,
      glfw::Key::W => Key::W,
      glfw::Key::X => Key::X,
      glfw::Key::Y => Key::Y,
      glfw::Key::Z => Key::Z,

      glfw::Key::Escape => Key::Esc,
      glfw::Key::Tab => Key::Tab,

      _ => Key::Unsupported,
    };

    let action = match action {
      glfw::Action::Press => KeyAction::Press,
      glfw::Action::Release => KeyAction::Release,
      _ => KeyAction::None,
    };

    KeyEvent { key, action }
  }
}

impl WindowHandle for GlfwWindow {
  fn process_input(&mut self, keyboard: &mut Keyboard) {
    for (_, event) in &self.event_stream {
      match event {
        WindowEvent::Key(key, _scancode, action, _modifiers) => {
          keyboard.process(GlfwWindow::convert_key_event(key, action))
        }
        WindowEvent::MouseButton(_mouse_button, _action, _modifiers) => {}
        _ => (),
      }
    }
  }

  fn present(&mut self) {
    todo!()
  }

  fn close(&mut self) {
    self.window_handle.set_should_close(true);
  }

  fn close_requested(&self) -> bool {
    self.window_handle.should_close()
  }

  fn bg_color(&mut self, _: (u8, u8, u8)) {
    todo!()
  }
  fn clear_color(&mut self) {
    todo!()
  }
}
