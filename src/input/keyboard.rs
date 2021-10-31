use enum_map::{Enum, EnumMap};
use std::collections::HashMap;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};

#[derive(PartialEq, PartialOrd, Enum, EnumCount, EnumIter)]
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

  Esc,
  Tab,

  Unsupported,
}

#[derive(Clone, Copy, PartialEq)]
pub enum KeyAction {
  None,
  Press,
  Release,
}

impl Default for KeyAction {
  fn default() -> Self {
    KeyAction::None
  }
}

pub struct KeyEvent {
  pub key: Key,
  pub action: KeyAction,
}

pub struct Keyboard {
  key_states: EnumMap<Key, KeyAction>,
}

impl Keyboard {
  pub fn new() -> Self {
    Self {
      key_states: EnumMap::default(),
    }
  }

  pub fn new_frame(&mut self) {
    for i in Key::iter() {
      self.key_states[i] = KeyAction::None;
    }
  }

  pub fn process(&mut self, event: KeyEvent) {}

  pub fn check(&self, key: Key) -> KeyAction {
    self.key_states[key]
  }
}
