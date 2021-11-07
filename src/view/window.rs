use crate::input::{
  keyboard::{Key, KeyAction, KeyEvent},
  InputDevices, InputProcessor,
};
use crate::math::*;
use crate::util::Settings;
use glfw::{Context, Glfw, OpenGlProfileHint, Window as GlfwWindow, WindowEvent, WindowHint};
use glium::backend::Backend;
use glm::U32Vec2;
use std::cell::RefCell;
use std::fmt::{Display, Error, Formatter};
use std::rc::Rc;
use std::sync::mpsc::Receiver;

pub struct WindowSettings {
  title: String,
  dimentions: U32Vec2,
}

impl WindowSettings {
  pub fn new(settings: &Settings) -> Self {
    Self {
      title: settings.display.title.clone(),
      dimentions: glm::vec2(settings.display.width, settings.display.height),
    }
  }
}

pub enum WindowMode {
  Fullscreen,
  Windowed,
  Borderless,
}

impl Default for WindowMode {
  fn default() -> Self {
    WindowMode::Borderless
  }
}

impl Display for WindowMode {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    match self {
      WindowMode::Fullscreen => write!(f, "fullscreen"),
      WindowMode::Windowed => write!(f, "windowed"),
      WindowMode::Borderless => write!(f, "borderless"),
    }
  }
}

impl From<&String> for WindowMode {
  fn from(string: &String) -> Self {
    match string.as_str() {
      "windowed" => WindowMode::Windowed,
      "fullscreen" => WindowMode::Fullscreen,
      "borderless" => WindowMode::Borderless,
      _ => WindowMode::Windowed,
    }
  }
}

type GlfwHandle = Rc<RefCell<Glfw>>;
type WindowHandle = Rc<RefCell<GlfwWindow>>;

pub struct Window {
  glfw_handle: GlfwHandle,
  window_handle: WindowHandle,
  event_stream: Receiver<(f64, WindowEvent)>,
}

impl Window {
  pub fn new(settings: WindowSettings) -> (Self, WindowDrawInterface) {
    let mut glfw_handle = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw_handle.default_window_hints();
    glfw_handle.window_hint(WindowHint::ContextVersionMajor(3));
    glfw_handle.window_hint(WindowHint::ContextVersionMinor(3));
    glfw_handle.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw_handle.window_hint(WindowHint::Resizable(false));
    glfw_handle.window_hint(WindowHint::Visible(false));
    glfw_handle.window_hint(WindowHint::DoubleBuffer(true));
    glfw_handle.window_hint(WindowHint::ContextNoError(true));

    let (mut window_handle, event_stream) = glfw_handle
      .create_window(
        settings.dimentions.x,
        settings.dimentions.y,
        &settings.title,
        glfw::WindowMode::Windowed,
      )
      .unwrap();

    window_handle.set_all_polling(true);

    let glfw_handle = Rc::new(RefCell::new(glfw_handle));
    let window_handle = Rc::new(RefCell::new(window_handle));

    (
      Self {
        glfw_handle: glfw_handle.clone(),
        window_handle: window_handle.clone(),
        event_stream,
      },
      WindowDrawInterface::new(glfw_handle, window_handle),
    )
  }

  pub fn show(&self) {
    self.window_handle.borrow_mut().show();
  }

  pub fn poll_events(&self, input_devices: &mut InputDevices) {
    self.glfw_handle.borrow_mut().poll_events();
    for (_, event) in glfw::flush_messages(&self.event_stream) {
      match event {
        WindowEvent::Key(key, _scancode, action, _modifiers) => {
          input_devices.process(Self::convert_key_event(key, action))
        }
        WindowEvent::MouseButton(_mouse_button, _action, _modifiers) => {}
        _ => (),
      }
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

pub struct WindowDrawInterface {
  glfw_handle: GlfwHandle,
  window_handle: WindowHandle,
}

impl WindowDrawInterface {
  fn new(glfw_handle: GlfwHandle, window_handle: WindowHandle) -> Self {
    Self {
      glfw_handle,
      window_handle,
    }
  }
}

unsafe impl Backend for WindowDrawInterface {
  fn swap_buffers(&self) -> std::result::Result<(), glium::SwapBuffersError> {
    let ptr = self.window_handle.borrow().window_ptr();
    unsafe { glfw::ffi::glfwSwapBuffers(ptr) };
    Ok(())
  }

  unsafe fn get_proc_address(&self, proc_name: &str) -> *const chlorine::c_void {
    self.glfw_handle.borrow().get_proc_address_raw(proc_name)
  }

  fn get_framebuffer_dimensions(&self) -> (u32, u32) {
    let fb = self.window_handle.borrow().get_framebuffer_size();
    (fb.0.try_into().unwrap(), fb.1.try_into().unwrap())
  }

  fn is_current(&self) -> bool {
    self.window_handle.borrow().is_current()
  }

  unsafe fn make_current(&self) {
    let ptr = self.window_handle.borrow().window_ptr();
    glfw::ffi::glfwMakeContextCurrent(ptr);
  }
}
