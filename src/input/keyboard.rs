use enum_map::{Enum, EnumMap};
use std::collections::HashMap;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Enum, EnumCount, EnumIter)]
pub enum Key {
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
  V,
  W,
  X,
  Y,
  Z,

  LeftArrow,
  RightArrow,
  UpArrow,
  DownArrow,

  PageUp,
  PageDown,

  Home,
  End,

  Insert,
  Delete,

  Backspace,
  Space,

  Enter,
  Escape,
  KeyPadEnter,

  Tab,

  LeftShift,
  RightShift,

  LeftCtrl,
  RightCtrl,

  LeftAlt,
  RightAlt,

  LeftSuper,
  RightSuper,

  Unsupported,
}

impl From<glfw::Key> for Key {
  fn from(key: glfw::Key) -> Self {
    match key {
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

      glfw::Key::Left => Key::LeftArrow,
      glfw::Key::Right => Key::RightArrow,
      glfw::Key::Up => Key::UpArrow,
      glfw::Key::Down => Key::DownArrow,

      glfw::Key::PageUp => Key::PageUp,
      glfw::Key::PageDown => Key::PageDown,

      glfw::Key::Home => Key::Home,
      glfw::Key::End => Key::End,

      glfw::Key::Insert => Key::Insert,
      glfw::Key::Delete => Key::Delete,

      glfw::Key::Backspace => Key::Backspace,
      glfw::Key::Space => Key::Space,

      glfw::Key::Enter => Key::Enter,
      glfw::Key::Escape => Key::Escape,
      glfw::Key::KpEnter => Key::KeyPadEnter,

      glfw::Key::Tab => Key::Tab,

      glfw::Key::LeftShift => Key::LeftShift,
      glfw::Key::RightShift => Key::RightShift,

      glfw::Key::LeftControl => Key::LeftCtrl,
      glfw::Key::RightControl => Key::RightCtrl,

      glfw::Key::LeftAlt => Key::LeftAlt,
      glfw::Key::RightAlt => Key::RightAlt,

      glfw::Key::LeftSuper => Key::LeftSuper,
      glfw::Key::RightSuper => Key::RightSuper,

      _ => Key::Unsupported,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyAction {
  None,
  Press,
  Release,
}

impl From<glfw::Action> for KeyAction {
  fn from(action: glfw::Action) -> Self {
    match action {
      glfw::Action::Press => KeyAction::Press,
      glfw::Action::Release => KeyAction::Release,
      _ => KeyAction::None,
    }
  }
}

impl Default for KeyAction {
  fn default() -> Self {
    KeyAction::None
  }
}

#[derive(Debug, Clone, Copy)]
pub struct KeyEvent {
  pub key: Key,
  pub action: KeyAction,
}

impl KeyEvent {
  pub fn new(key: Key, action: KeyAction) -> Self {
    Self { key, action }
  }
}

#[derive(Default)]
pub struct Keyboard {
  key_states: EnumMap<Key, KeyAction>,
}

impl Keyboard {
  pub fn new_frame(&mut self) {
    for i in Key::iter() {
      self.key_states[i] = KeyAction::None;
    }
  }

  pub fn process(&mut self, event: KeyEvent) {
    self.key_states[event.key] = event.action;
  }

  pub fn check(&self, key: Key) -> KeyAction {
    self.key_states[key]
  }
}
