use super::Camera;
use crate::{
  gfx::*,
  scripting::prelude::*,
  util::{ChildLogger, DirID, Logger},
};
use imgui_glium_renderer::glium::{self, texture::SrgbTexture2d, uniform, Surface};
use mlua::Table;
use std::{cell::RefCell, collections::BTreeMap, fs, path::PathBuf, rc::Rc};
use toml::Value;

mod keys {
  pub const SCRIPT: &str = "script";
  pub const CLASS: &str = "class";
  pub const SHADER: &str = "shader";
  pub const MODEL: &str = "model";
  pub const TEXTURE: &str = "texture";
}

pub struct EntityRepository {
  templates: BTreeMap<String, Rc<EntityTemplate>>,
  logger: ChildLogger,
}

impl EntityRepository {
  pub fn new<I>(logger: ChildLogger, iter: I) -> Self
  where
    I: Iterator<Item = (PathBuf, DirID)>,
  {
    let mut templates = BTreeMap::default();

    for (path, id) in iter {
      match fs::read_to_string(&path) {
        Ok(data) => match data.parse::<Value>() {
          Ok(root) => {
            if let Some(table) = root.as_table() {
              for (k, v) in table {
                let id = id.extend(k);

                let script = if let Some(Value::String(script)) = v.get(keys::SCRIPT) {
                  Some(script.clone())
                } else {
                  None
                };

                let class = if let Some(Value::String(on_update)) = v.get(keys::CLASS) {
                  Some(on_update.clone())
                } else {
                  None
                };

                let shader = if let Some(Value::String(shader)) = v.get(keys::SHADER) {
                  Some(shader.clone())
                } else {
                  None
                };

                let model = if let Some(Value::String(model)) = v.get(keys::MODEL) {
                  Some(model.clone())
                } else {
                  None
                };

                let texture = if let Some(Value::String(texture)) = v.get(keys::TEXTURE) {
                  Some(texture.clone())
                } else {
                  None
                };

                let tmpl = EntityTemplate {
                  script,
                  class,
                  shader,
                  model,
                  texture,
                };

                templates.insert(id.into(), Rc::new(tmpl));
              }
            } else {
              logger.error(format!("entity file is not a table '{:?}'", path));
            }
          }
          Err(err) => logger.error(format!("cannot parse entity file '{:?}': {}", path, err)),
        },
        Err(err) => logger.error(format!("cannot read entity file '{:?}': {}", path, err)),
      }
    }

    Self { templates, logger }
  }

  pub fn construct(
    &self,
    id: &str,
    scripts: &ScriptRepository,
    shaders: &ShaderRepository,
    models: &ModelRepository,
    textures: &TextureRepository,
  ) -> Result<Entity, String> {
    if let Some(tmpl) = self.templates.get(id) {
      let mut entity = Entity::default();

      if let Some(script) = &tmpl.script {
        if let Some(lua) = scripts.get(script) {
          entity.lua = Some(lua);
          if let Some(class_name) = &tmpl.class {
            let globals = lua.globals();
            if let Ok(mlua::Value::Table(class)) = globals.get(class_name.as_str()) {
              entity.class = Some(class_name.clone());

              let data = match class.get("new") {
                Ok(mlua::Value::Function(new)) => {
                  let res: Result<Table, mlua::Error> = new.call(class);

                  match res {
                    Ok(data) => data,
                    Err(e) => {
                      self.logger.error(e.to_string());
                      lua.create_table().unwrap()
                    }
                  }
                }
                Ok(mlua::Value::Nil) => lua.create_table().unwrap(),
                Ok(_) => {
                  self
                    .logger
                    .error("invalid type for initalizer function".to_string());
                  lua.create_table().unwrap()
                }
                Err(e) => {
                  self.logger.error(e.to_string());
                  lua.create_table().unwrap()
                }
              };

              entity.data = Some(mlua::Value::Table(data));
            }
          }
        }
      }

      if let Some(shader) = &tmpl.shader {
        if let Some(shader) = shaders.get(shader) {
          entity.shader = Some(shader.clone());
          if let Some(model) = &tmpl.model {
            if let Some(model) = models.get(model) {
              entity.model = Some(model.clone());
              if let Some(texture) = &tmpl.texture {
                if let Some(texture) = textures.get(texture) {
                  entity.texture = Some(texture.clone());
                }
              }
            }
          }
        }
      }

      Ok(entity)
    } else {
      Err(format!("could not find {}", id))
    }
  }
}

#[derive(Default)]
pub struct EntityTemplate {
  script: Option<String>,
  class: Option<String>,
  shader: Option<String>,
  model: Option<String>,
  texture: Option<String>,
}

#[derive(Default)]
pub struct Entity {
  lua: Option<&'static Lua>,
  class: Option<String>,
  shader: Option<Rc<Shader>>,
  model: Option<Rc<Model>>,
  texture: Option<Rc<SrgbTexture2d>>,
  data: Option<mlua::Value<'static>>,
}

impl Entity {
  pub fn update<L: Logger>(&mut self, logger: &L) {
    if let Some(lua) = &self.lua {
      if let Some(class) = &self.class {
        let globals = lua.globals();

        if let Ok(mlua::Value::Table(class)) = globals.get(class.as_str()) {
          if let Ok(mlua::Value::Function(on_update)) = class.get("update") {
            if let Some(data) = &self.data {
              let res: Result<(), mlua::Error> = on_update.call(data.clone());
              if let Err(e) = res {
                logger.error(e.to_string());
              }
            }
          }
        }
      }
    }
  }

  pub fn draw_to<S: Surface>(&self, surface: &mut S, camera: &Camera) {
    if let Some(shader) = &self.shader {
      if let Some(model) = &self.model {
        if let Some(texture) = &self.texture {
          use crate::math::glm;
          let transform = glm::Mat4::identity();
          let transform = transform.scale(0.5);
          let transform: [[f32; 4]; 4] = transform.into();
          let view: [[f32; 4]; 4] = glm::Mat4::identity().into(); // camera.view();
          let proj: [[f32; 4]; 4] = glm::Mat4::identity().into(); // camera.projection();

          let uniforms = uniform! {
            model: transform,
            view: view,
            projection: proj,
            tex: &**texture,
          };
          surface
            .draw(
              &model.vbuff,
              &model.ibuff,
              shader,
              &uniforms,
              &Default::default(),
            )
            .unwrap();
        }
      }
    }
  }
}

impl AsPtr for Entity {}

impl UserData for MutPtr<Entity> {}

#[derive(Clone)]
pub struct Tile;

#[derive(Default)]
pub struct MapData {
  pub width: usize,
  pub height: usize,
}

pub struct Map<'r> {
  data: MapData,
  tiles: Vec<Tile>,
  named_entities: BTreeMap<String, Rc<RefCell<Entity>>>,
  anonymous_entities: Vec<Rc<RefCell<Entity>>>,
  static_entities: Vec<ConstPtr<Entity>>,
  mutable_entities: Vec<MutPtr<Entity>>,
  drawable_entities: Vec<ConstPtr<Entity>>,

  logger: ChildLogger,

  entities: &'r EntityRepository,
  scripts: &'r ScriptRepository,
  shaders: &'r ShaderRepository,
  models: &'r ModelRepository,
  textures: &'r TextureRepository,
}

impl<'r> Map<'r> {
  pub fn new(
    data: MapData,
    logger: ChildLogger,
    entities: &'r EntityRepository,
    scripts: &'r ScriptRepository,
    shaders: &'r ShaderRepository,
    models: &'r ModelRepository,
    textures: &'r TextureRepository,
  ) -> Self {
    let tiles = vec![Tile; data.width * data.height];
    Self {
      data,
      tiles,
      named_entities: Default::default(),
      anonymous_entities: Default::default(),
      static_entities: Default::default(),
      mutable_entities: Default::default(),
      drawable_entities: Default::default(),

      logger,

      entities,
      scripts,
      shaders,
      models,
      textures,
    }
  }

  pub fn update<L: Logger>(&mut self, logger: &L) {
    for entity in &mut self.mutable_entities {
      entity.update(logger);
    }
  }

  pub fn draw_to<S: Surface>(&self, surface: &mut S, camera: &Camera) {
    for entity in &self.drawable_entities {
      entity.draw_to(surface, camera);
    }
  }

  pub fn spawn_entity(&mut self, id: &str) {
    let entity =
      self
        .entities
        .construct(id, self.scripts, self.shaders, self.models, self.textures);
    if let Ok(entity) = entity {
      let is_mutable = entity.lua.is_some() && entity.class.is_some();
      let is_renderable = entity.model.is_some() && entity.texture.is_some();

      let entity = Rc::new(RefCell::new(entity));

      self.anonymous_entities.push(entity.clone());

      if is_renderable {
        let ptr = MutPtr::from(entity.clone()).into();
        self.drawable_entities.push(ptr);
      }

      if is_mutable {
        let ptr = MutPtr::from(entity);
        self.mutable_entities.push(ptr);
      } else {
        let ptr = MutPtr::from(entity).into();
        self.static_entities.push(ptr);
      }
    } else {
      self.logger.warn(format!("unable to load entity '{}'", id));
    }
  }

  pub fn spawn_named_entity(&mut self, id: &str, name: String) {
    let entity =
      self
        .entities
        .construct(id, self.scripts, self.shaders, self.models, self.textures);
    if let Ok(entity) = entity {
      let is_static = entity.lua.is_some();

      let entity = Rc::new(RefCell::new(entity));

      self.named_entities.insert(name, entity.clone());

      if is_static {
        let ptr = MutPtr::from(entity).into();
        self.static_entities.push(ptr);
      } else {
        let ptr = MutPtr::from(entity);
        self.mutable_entities.push(ptr);
      }
    } else {
      self.logger.warn(format!(
        "unable to load entity '{}' with name '{}'",
        id, name
      ));
    }
  }
}
