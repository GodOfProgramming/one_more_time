use crate::util::{DirID, Logger};
use image::{io::Reader, RgbaImage};
use imgui_glium_renderer::glium::{
  backend::Facade,
  texture::{RawImage2d, SrgbTexture2d},
};
use std::{collections::BTreeMap, path::PathBuf};

#[derive(Default)]
pub struct TextureSources {
  images: BTreeMap<DirID, RgbaImage>,
}

impl TextureSources {
  pub fn load_all<L, I>(&mut self, logger: &L, iter: I)
  where
    L: Logger,
    I: Iterator<Item = (PathBuf, DirID)>,
  {
    logger.info("loading textures".to_string());
    for (path, id) in iter {
      logger.info(format!("loading {}", id));
      match Reader::open(&path) {
        Ok(reader) => match reader.decode() {
          Ok(image) => {
            self.images.insert(id, image.to_rgba8());
          }
          Err(err) => logger.error(format!("error decoding '{:?}': {}", path, err)),
        },
        Err(err) => logger.error(format!("error reading '{:?}': {}", path, err)),
      }
    }
  }

  pub fn load_repository<L, F>(self, logger: &L, facade: &F) -> TextureRepository
  where
    L: Logger,
    F: Facade,
  {
    let mut tex_repo = TextureRepository::default();

    for (id, img) in self.images {
      let dim = img.dimensions();
      let raw_img: RawImage2d<u8> = RawImage2d::from_raw_rgba_reversed(&img.into_raw(), dim);

      match SrgbTexture2d::new(facade, raw_img) {
        Ok(tex) => {
          tex_repo.textures.insert(id.into(), tex);
        }
        Err(err) => logger.error(format!("could not convert {} to srgb tex: {:?}", id, err)),
      }
    }

    tex_repo
  }
}

#[derive(Default)]
pub struct TextureRepository {
  textures: BTreeMap<String, SrgbTexture2d>,
}

impl TextureRepository {
  pub fn get(&self, id: &str) -> Option<&SrgbTexture2d> {
    self.textures.get(id)
  }
}
