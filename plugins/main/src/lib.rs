use directory_iter::RecursiveDirIteratorWithID;
use game::TestModel;
use omt::{image::io::Reader, Plugin, PluginResult};
use std::{fs, rc::Rc};
use ui::DebugMainMenu;

mod directory_iter;
mod game;
mod ui;

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn exports(plugin: *mut dyn Plugin) -> PluginResult {
  let mut plugin = Box::from_raw(plugin);

  // entities
  {
    load_entities(&mut plugin);
  }

  // textures
  {
    let iter = RecursiveDirIteratorWithID::new(&plugin.path().join("tex"));
    load_textures(&mut plugin, iter);
  }

  // shaders
  {
    let iter = RecursiveDirIteratorWithID::new_with_ext(&plugin.path().join("shaders"));
    load_shaders(&mut plugin, iter);
  }

  // ui
  {
    let iter = RecursiveDirIteratorWithID::new(&plugin.path().join("ui"));
    load_ui(&mut plugin, iter);
  }

  Box::into_raw(plugin);

  Ok(())
}

pub fn load_entities(plugin: &mut Box<dyn Plugin>) {
  plugin.entity_models().register("test", Box::new(TestModel));
}

pub fn load_textures(plugin: &mut Box<dyn Plugin>, iter: RecursiveDirIteratorWithID) {
  plugin.logger().info("loading textures".to_string());
  for (id, path) in iter {
    plugin.logger().info(format!("loading {}", id));
    match Reader::open(&path) {
      Ok(reader) => match reader.decode() {
        Ok(image) => {
          plugin.textures().register(id, image.to_rgba8());
        }
        Err(err) => plugin
          .logger()
          .error(format!("error decoding '{:?}': {}", path, err)),
      },
      Err(err) => plugin
        .logger()
        .error(format!("error reading '{:?}': {}", path, err)),
    }
  }
}

pub fn load_shaders(plugin: &mut Box<dyn Plugin>, iter: RecursiveDirIteratorWithID) {
  plugin
    .shaders()
    .register_shader("basic", "main.basic.vs", "main.basic.fs");

  for (id, file) in iter {
    if let Ok(source) = fs::read_to_string(file) {
      plugin.shaders().register(&id, &source);
    }
  }
}

pub fn load_ui(plugin: &mut Box<dyn Plugin>, iter: RecursiveDirIteratorWithID) {
  plugin.logger().info("loading ui".to_string());

  plugin
    .ui_models()
    .register("debug-main-menu", Rc::new(DebugMainMenu));

  for (id, path) in iter {
    plugin.logger().info(format!("loading {}", id));
    if let Ok(data) = fs::read_to_string(path) {
      plugin.ui_sources().register(id, data);
    }
  }
}
