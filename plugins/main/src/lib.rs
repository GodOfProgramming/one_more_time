use directory_iter::RecursiveDirIteratorWithID;
use game::TestModel;
use omt::{gfx::ShaderSource, image::io::Reader, Plugin, PluginResult};
use std::{fs, rc::Rc};
use ui::DebugMainMenu;

mod directory_iter;
mod game;
mod ui;

#[no_mangle]
pub unsafe extern "C" fn exports(plugin: *mut dyn Plugin) -> PluginResult {
  let mut plugin = unsafe { Box::from_raw(plugin) };
  // entities
  {
    load_entities(&mut plugin);
  }

  // textures
  {
    let iter = RecursiveDirIteratorWithID::from(plugin.path().join("tex"));
    load_textures(&mut plugin, iter);
  }

  // shaders
  {
    load_shaders(&mut plugin);
  }

  // ui
  {
    let iter = RecursiveDirIteratorWithID::from(plugin.path().join("ui"));
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
  for (path, id) in iter {
    plugin.logger().info(format!("loading {}", id));
    match Reader::open(&path) {
      Ok(reader) => match reader.decode() {
        Ok(image) => {
          plugin.textures().register(id.to_string(), image.to_rgba8());
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

pub fn load_shaders(plugin: &mut Box<dyn Plugin>) {
  let shader_dir = plugin.path().join("shaders");
  let shader = ShaderSource::new(&shader_dir.join("basic.vs"), &shader_dir.join("basic.fs"));
  plugin.shaders().register("basic", shader);
}

pub fn load_ui(plugin: &mut Box<dyn Plugin>, iter: RecursiveDirIteratorWithID) {
  plugin.logger().info("loading ui".to_string());

  plugin
    .ui_models()
    .register("debug-main-menu", Rc::new(DebugMainMenu));

  for (path, id) in iter {
    plugin.logger().info(format!("loading {}", id));
    if let Ok(data) = fs::read_to_string(path) {
      plugin.ui_sources().register(id.to_string(), data);
    }
  }
}
