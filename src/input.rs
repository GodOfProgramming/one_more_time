pub mod keyboard;
pub mod mouse;

use keyboard::{Key, KeyAction, KeyEvent, Keyboard};
use mouse::{Button, ButtonAction, Mouse, MouseButtonEvent};

pub trait InputProcessor<T> {
  fn process(&mut self, kind: T);
}

pub trait InputCheck<T, A> {
  fn check(&self, kind: T) -> A;
}

#[derive(Default)]
pub struct InputDevices {
  keyboard: Keyboard,
  mouse: Mouse,
}

impl InputDevices {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn new_frame(&mut self) {
    self.keyboard.new_frame();
    self.mouse.new_frame();
  }
}

impl InputProcessor<KeyEvent> for InputDevices {
  fn process(&mut self, event: KeyEvent) {
    self.keyboard.process(event);
  }
}

impl InputCheck<Key, KeyAction> for InputDevices {
  fn check(&self, key: Key) -> KeyAction {
    self.keyboard.check(key)
  }
}

impl InputProcessor<MouseButtonEvent> for InputDevices {
  fn process(&mut self, event: MouseButtonEvent) {
    self.mouse.process(event);
  }
}

impl InputCheck<Button, ButtonAction> for InputDevices {
  fn check(&self, button: Button) -> ButtonAction {
    self.mouse.check(button)
  }
}
