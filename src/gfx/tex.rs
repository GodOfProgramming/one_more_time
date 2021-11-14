use crate::util::{self, DirID};
use glium::texture::{
  self, RawImage2d, ResidentTexture, Texture2d, Texture2dDataSource, TextureAny, TextureHandle,
  TextureKind,
};
use image::{io::Reader, DynamicImage, ImageBuffer, RgbaImage};
use log::error;
use std::{
  collections::BTreeMap,
  fs::{self, File},
  path::PathBuf,
};
use toml::{value::Table, Value};

pub struct TextureSources {
  images: BTreeMap<DirID, DynamicImage>,
}

impl TextureSources {
  pub fn load_all(&mut self) {
    let config = PathBuf::new().join("assets").join("cfg").join("textures");
    util::iterate_dir_with_id(&config, |path, id| {
      let data = fs::read_to_string(path)
        .map_err(|e| format!("cannot find {}, err = {}", path.display(), e))
        .unwrap();
      let table = data.parse::<Value>().unwrap();
      let table = table.as_table().unwrap();

      for (local_shader_id, filename) in table {
        let new_id = id.extend(&local_shader_id);
        let filename = filename.as_str().unwrap();
        let img = Reader::open(filename).unwrap().decode().unwrap();
        self.images.insert(new_id, img);
      }
    });
  }

  pub fn load_repository(self) {
    for (id, image) in self.images {}
  }
}

struct ImageConverter {}

impl Texture2dDataSource<'_> for ImageConverter {
  type Data = u8;
  fn into_raw(self) -> RawImage2d<'a, <Self as Texture2dDataSource<'a>>::Data> {
    RawImage2d::from_raw_rgb(data: Vec<T>, dimensions: (u32, u32))
  }
}

pub struct TextureRepository {
  textures: BTreeMap<DirID, TextureAny>,
}

impl TextureRepository {}
