use crate::util::{self, DirID, Dirs, RecursiveDirIteratorWithID};
use image::{io::Reader, DynamicImage, ImageBuffer, RgbaImage};
use imgui_glium_renderer::glium::texture::{
  self, RawImage2d, ResidentTexture, Texture2d, Texture2dDataSource, TextureAny, TextureHandle,
  TextureKind,
};
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
  pub fn load_all(&mut self, dirs: &Dirs) {
    for (path, id) in RecursiveDirIteratorWithID::from(&dirs.assets.cfg.textures) {
      let data = fs::read_to_string(&path)
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
    }
  }

  pub fn load_repository(self) {
    for (id, image) in self.images {}
  }
}

struct ImageConverter {}

pub struct TextureRepository {
  textures: BTreeMap<DirID, TextureAny>,
}

impl TextureRepository {}
