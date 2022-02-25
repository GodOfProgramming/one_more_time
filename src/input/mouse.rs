use crate::imgui;
use enum_map::{Enum, EnumMap};
use strum::IntoEnumIterator;
use strum_macros::{EnumCount, EnumIter};

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Enum, EnumCount, EnumIter)]
pub enum Button {
  Left,
  Middle,
  Right,
  Extra1,
  Extra2,
  Extra3,
  Extra4,
  Extra5,
}

impl From<glfw::MouseButton> for Button {
  fn from(button: glfw::MouseButton) -> Self {
    match button {
      glfw::MouseButton::Button1 => Self::Left,
      glfw::MouseButton::Button2 => Self::Right,
      glfw::MouseButton::Button3 => Self::Middle,
      glfw::MouseButton::Button4 => Self::Extra1,
      glfw::MouseButton::Button5 => Self::Extra2,
      glfw::MouseButton::Button6 => Self::Extra3,
      glfw::MouseButton::Button7 => Self::Extra4,
      glfw::MouseButton::Button8 => Self::Extra5,
    }
  }
}

impl From<imgui::MouseButton> for Button {
  fn from(button: imgui::MouseButton) -> Self {
    match button {
      imgui::MouseButton::Left => Button::Left,
      imgui::MouseButton::Right => Button::Right,
      imgui::MouseButton::Middle => Button::Middle,
      imgui::MouseButton::Extra1 => Button::Extra1,
      imgui::MouseButton::Extra2 => Button::Extra2,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonAction {
  None,
  Press,
  Release,
}

impl From<glfw::Action> for ButtonAction {
  fn from(action: glfw::Action) -> Self {
    match action {
      glfw::Action::Press => Self::Press,
      glfw::Action::Release => Self::Release,
      _ => Self::None,
    }
  }
}

impl Default for ButtonAction {
  fn default() -> Self {
    Self::None
  }
}

#[derive(Debug, Clone, Copy)]
pub struct MouseButtonEvent {
  pub button: Button,
  pub action: ButtonAction,
}

impl MouseButtonEvent {
  pub fn new(button: Button, action: ButtonAction) -> Self {
    Self { button, action }
  }
}

#[derive(Default)]
pub struct Mouse {
  button_states: EnumMap<Button, ButtonAction>,
}

impl Mouse {
  pub fn new_frame(&mut self) {
    for i in Button::iter() {
      self.button_states[i] = ButtonAction::None;
    }
  }

  pub fn process(&mut self, event: MouseButtonEvent) {
    self.button_states[event.button] = event.action;
  }

  pub fn check(&self, button: Button) -> ButtonAction {
    self.button_states[button]
  }
}
