use image::RgbaImage;
use std::path::{ PathBuf, Path };

pub struct ShaderSource {
  pub vertex: PathBuf,
  pub fragment: PathBuf,
}

impl ShaderSource {
  pub fn new(vertex: &Path, fragment: &Path) -> Self {
    Self {
      vertex: vertex.to_path_buf(),
      fragment: fragment.to_path_buf(),
    }
  }
}

pub trait ShaderLoader {
  fn register(&mut self, id: &str, src: ShaderSource);
}

pub trait TextureLoader {
  fn register(&mut self, name: String, image: RgbaImage);
}
