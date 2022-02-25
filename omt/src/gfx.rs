use image::RgbaImage;
use std::path::PathBuf;

pub struct ShaderSource {
  pub vertex: PathBuf,
  pub fragment: PathBuf,
}

pub trait ShaderLoader {
  fn register(&mut self, id: &str, src: ShaderSource);
}

pub trait TextureLoader {
  fn register(&mut self, name: String, image: RgbaImage);
}
