pub mod glfw_window;

use crate::input::keyboard::Keyboard;
pub use glfw_window::GlfwWindow;

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

pub trait WindowHandle {
  fn open(&self) -> bool;
  fn process_input(&self, keyboard: &mut Keyboard);
  fn next_buffer(&mut self);
  fn close(&mut self);
  fn close_requested(&self) -> bool;
}

pub struct Window<W>
where
  W: WindowHandle,
{
  handle: W,
}

impl<W> Window<W>
where
  W: WindowHandle,
{
  pub fn new(handle: W) -> Self {
    Self { handle }
  }
}

impl<W> WindowHandle for Window<W>
where
  W: WindowHandle,
{
  fn open(&self) -> bool {
    self.handle.open()
  }

  fn process_input(&self, keyboard: &mut Keyboard) {
    self.handle.process_input(keyboard);
  }

  fn next_buffer(&mut self) {
    self.handle.next_buffer();
  }

  fn close(&mut self) {
    self.handle.close()
  }

  fn close_requested(&self) -> bool {
    self.handle.close_requested()
  }
}
