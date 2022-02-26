use image::RgbaImage;
use std::path::{Path, PathBuf};

pub trait ShaderLoader {
  fn register(&mut self, id: &str, src: &str);
  fn register_shader(&mut self, id: &str, vertex: &str, fragment: &str);
}

pub trait TextureLoader {
  fn register(&mut self, name: String, image: RgbaImage);
}
