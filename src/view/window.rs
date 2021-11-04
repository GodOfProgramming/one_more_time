pub mod glfw_window;
pub mod sdl2_window;

use crate::input::keyboard::Keyboard;
use crate::util::Settings;
pub use glfw_window::GlfwWindow;
use glm::U32Vec2;
use nalgebra_glm as glm;
pub use sdl2_window::Sdl2Window;
use std::fmt::{Display, Error, Formatter};

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

pub trait WindowHandle {
  fn process_input(&mut self, keyboard: &mut Keyboard);
  fn bg_color(&mut self, rgb: (u8, u8, u8));
  fn present(&mut self);
  fn clear_color(&mut self);
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
  fn process_input(&mut self, keyboard: &mut Keyboard) {
    self.handle.process_input(keyboard);
  }

  fn bg_color(&mut self, rgb: (u8, u8, u8)) {
    self.handle.bg_color(rgb);
  }

  fn present(&mut self) {
    self.handle.present();
  }

  fn clear_color(&mut self) {
    self.handle.clear_color();
  }

  fn close(&mut self) {
    self.handle.close()
  }

  fn close_requested(&self) -> bool {
    self.handle.close_requested()
  }
}
