use crate::{
  glium::{
    backend::Facade,
    texture::{RawImage2d, SrgbTexture2d},
  },
  util::prelude::*,
};
use omt::{gfx::TextureLoader, image::RgbaImage};
use std::{collections::BTreeMap, rc::Rc};

pub enum TextureLoadError {
  Standard(String),
}

#[derive(Default)]
pub struct ImageArchive {
  images: BTreeMap<String, RgbaImage>,
}

impl TextureLoader for ImageArchive {
  fn register(&mut self, name: String, image: RgbaImage) {
    self.images.insert(name, image);
  }
}

#[derive(Default)]
pub struct TextureArchive {
  textures: BTreeMap<String, Rc<SrgbTexture2d>>,
}

impl TextureArchive {
  pub fn add_image_archive<F: Facade>(
    &mut self,
    archive: ImageArchive,
    facade: &F,
  ) -> Result<(), Vec<TextureLoadError>> {
    let mut errors = Vec::default();

    for (id, img) in archive.images {
      let dim = img.dimensions();
      let raw_img: RawImage2d<u8> = RawImage2d::from_raw_rgba_reversed(&img.into_raw(), dim);

      let result = SrgbTexture2d::new(facade, raw_img).map_err(|err| {
        TextureLoadError::Standard(format!("could not convert {} to srgb tex: {:?}", id, err))
      });

      match result {
        Ok(tex) => {
          self.textures.insert(id.into(), Rc::new(tex));
        }
        Err(err) => errors.push(err),
      }
    }

    if errors.is_empty() {
      Ok(())
    } else {
      Err(errors)
    }
  }

  pub fn get(&self, id: &str) -> Option<Rc<SrgbTexture2d>> {
    self.textures.get(id).cloned()
  }
}

impl AsPtr for TextureArchive {}
