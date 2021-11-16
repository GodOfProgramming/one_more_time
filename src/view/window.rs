use crate::input::{
  keyboard::{Key, KeyAction, KeyEvent},
  mouse::{Button, ButtonAction, MouseButtonEvent},
  InputDevices, InputProcessor,
};
use crate::math::*;
use crate::util::Settings;
use glfw::{Context, Glfw, OpenGlProfileHint, Window as GlfwWindow, WindowEvent, WindowHint};
use glium::backend::Backend;
use glm::U32Vec2;
use imgui_glium_renderer::imgui;
use log::debug;
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

  pub fn setup_imgui(&self, imgui_ctx: &mut imgui::Context) {
    let mut io = imgui_ctx.io_mut();

    io[imgui::Key::Tab] = Key::Tab as _;
    io[imgui::Key::LeftArrow] = Key::LeftArrow as _;
    io[imgui::Key::RightArrow] = Key::RightArrow as _;
    io[imgui::Key::UpArrow] = Key::UpArrow as _;
    io[imgui::Key::DownArrow] = Key::DownArrow as _;
    io[imgui::Key::PageUp] = Key::PageUp as _;
    io[imgui::Key::PageDown] = Key::PageDown as _;
    io[imgui::Key::Home] = Key::Home as _;
    io[imgui::Key::End] = Key::End as _;
    io[imgui::Key::Insert] = Key::Insert as _;
    io[imgui::Key::Delete] = Key::Delete as _;
    io[imgui::Key::Backspace] = Key::Backspace as _;
    io[imgui::Key::Space] = Key::Space as _;
    io[imgui::Key::Enter] = Key::Enter as _;
    io[imgui::Key::Escape] = Key::Escape as _;
    io[imgui::Key::KeyPadEnter] = Key::KeyPadEnter as _;
    io[imgui::Key::A] = Key::A as _;
    io[imgui::Key::C] = Key::C as _;
    io[imgui::Key::V] = Key::V as _;
    io[imgui::Key::X] = Key::X as _;
    io[imgui::Key::Y] = Key::Y as _;
    io[imgui::Key::Z] = Key::Z as _;
  }

  pub fn show(&self) {
    self.window_handle.borrow_mut().show();
  }

  pub fn poll_events(&self, input_devices: &mut InputDevices, imgui_ctx: &mut imgui::Context) {
    let mut io = imgui_ctx.io_mut();
    self.glfw_handle.borrow_mut().poll_events();
    for (_, event) in glfw::flush_messages(&self.event_stream) {
      match event {
        WindowEvent::Key(key, _scancode, action, _modifiers) => {
          let key_event = KeyEvent::new(key.into(), action.into());
          input_devices.process(key_event);

          match key_event.action {
            KeyAction::Press => {
              io.keys_down[key_event.key as usize] = true;
            }
            KeyAction::Release => {
              io.keys_down[key_event.key as usize] = false;
            }
            _ => {}
          }

          match key_event.key {
            Key::LeftShift | Key::RightShift => io.key_shift = key_event.action == KeyAction::Press,
            Key::LeftCtrl | Key::RightCtrl => io.key_ctrl = key_event.action == KeyAction::Press,
            Key::LeftAlt | Key::RightAlt => io.key_alt = key_event.action == KeyAction::Press,
            Key::LeftSuper | Key::RightSuper => io.key_super = key_event.action == KeyAction::Press,
            _ => {}
          }
        }
        WindowEvent::Char(c) => {
          io.add_input_character(c);
        }
        WindowEvent::MouseButton(mouse_button, action, _modifiers) => {
          let mouse_event = MouseButtonEvent::new(mouse_button.into(), action.into());
          input_devices.process(mouse_event);

          match mouse_event.action {
            ButtonAction::Press => {
              io.mouse_down[mouse_event.button as usize] = true;
            }
            ButtonAction::Release => {
              io.mouse_down[mouse_event.button as usize] = false;
            }
            _ => {}
          }
        }
        WindowEvent::Scroll(x, y) => {
          io.mouse_wheel = y as f32;
          io.mouse_wheel_h = x as f32;
        }
        WindowEvent::CursorPos(x, y) => {
          io.mouse_pos = [x as f32, y as f32];
        }
        WindowEvent::FramebufferSize(x, y) => {
          io.display_size = [x as f32, y as f32];
        }
        _ => (),
      }
    }
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
