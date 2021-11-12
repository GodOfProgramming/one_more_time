use super::ImageLoader;
use sdl2::image::{self, InitFlag, Sdl2ImageContext};
use sdl2::render::TextureCreator;

pub struct Sdl2Loader {
  context: Sdl2ImageContext,
}

impl Sdl2Loader {
  pub fn new() -> Self {
    let context = image::init(InitFlag::JPG | InitFlag::PNG).unwrap();
    Self { context }
  }
}

impl ImageLoader for Sdl2Loader {}
