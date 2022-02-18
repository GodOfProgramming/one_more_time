use super::{WindowHandle, WindowSettings};
use crate::input::keyboard::{Key, KeyAction, KeyEvent, Keyboard};
use sdl2::{
  event::Event,
  keyboard::Keycode,
  pixels::Color,
  render::Canvas,
  video::{Window, WindowBuilder},
  EventPump, Sdl, VideoSubsystem,
};

pub struct Sdl2Window {
  _sdl_context: Sdl,
  _video_subsystem: VideoSubsystem,
  canvas: Canvas<Window>,
  event_pump: EventPump,

  should_close: bool,
}

impl Sdl2Window {
  pub fn new(settings: WindowSettings) -> Self {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut window_builder: WindowBuilder = video_subsystem.window(
      &settings.title,
      settings.dimensions.x,
      settings.dimensions.y,
    );

    window_builder.position_centered();
    window_builder.input_grabbed();
    window_builder.opengl();

    let window: Window = window_builder.build().unwrap();

    let canvas = window.into_canvas().build().unwrap();

    let event_pump = sdl_context.event_pump().unwrap();

    let tc = canvas.texture_creator();
    if cfg!(windows) {}

    Self {
      _sdl_context: sdl_context,
      _video_subsystem: video_subsystem,
      canvas,
      event_pump,
      should_close: false,
    }
  }
}

impl Sdl2Window {
  fn convert_keycode(key: Keycode) -> Key {
    match key {
      Keycode::A => Key::A,
      Keycode::B => Key::B,
      Keycode::C => Key::C,
      Keycode::D => Key::D,
      Keycode::E => Key::E,
      Keycode::F => Key::F,
      Keycode::G => Key::G,
      Keycode::H => Key::H,
      Keycode::I => Key::I,
      Keycode::J => Key::J,
      Keycode::K => Key::K,
      Keycode::L => Key::L,
      Keycode::M => Key::M,
      Keycode::N => Key::N,
      Keycode::O => Key::O,
      Keycode::P => Key::P,
      Keycode::Q => Key::Q,
      Keycode::R => Key::R,
      Keycode::S => Key::S,
      Keycode::T => Key::T,
      Keycode::U => Key::U,
      Keycode::V => Key::V,
      Keycode::W => Key::W,
      Keycode::X => Key::X,
      Keycode::Y => Key::Y,
      Keycode::Z => Key::Z,

      Keycode::Escape => Key::Esc,
      Keycode::Tab => Key::Tab,

      _ => Key::Unsupported,
    }
  }

  fn make_key_event(key: Keycode, action: KeyAction) -> KeyEvent {
    KeyEvent {
      key: Self::convert_keycode(key),
      action,
    }
  }
}

impl WindowHandle for Sdl2Window {
  fn process_input(&mut self, keyboard: &mut Keyboard) {
    for event in self.event_pump.poll_iter() {
      match event {
        Event::Quit { .. } => {
          self.should_close = true;
        }
        Event::KeyUp {
          keycode: Some(keycode),
          ..
        } => keyboard.process(Self::make_key_event(keycode, KeyAction::Release)),
        Event::KeyDown {
          keycode: Some(keycode),
          ..
        } => keyboard.process(Self::make_key_event(keycode, KeyAction::Press)),
        _ => {}
      }
    }
  }

  fn bg_color(&mut self, rgb: (u8, u8, u8)) {
    self.canvas.set_draw_color(Color::RGB(rgb.0, rgb.1, rgb.2));
  }

  fn present(&mut self) {
    self.canvas.present();
  }

  fn clear_color(&mut self) {
    self.canvas.clear();
  }

  fn close(&mut self) {
    self.should_close = true;
  }

  fn close_requested(&self) -> bool {
    self.should_close
  }
}
