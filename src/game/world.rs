use super::Camera;
use crate::{gfx::*, math::glm::Mat4, util::prelude::*};
use omt::{
  glium::{texture::SrgbTexture2d, uniform, Surface},
  mlua,
  toml::Value,
  uid::Id,
};
use std::{
  collections::{BTreeMap, BTreeSet},
  fs,
  path::PathBuf,
  rc::Rc,
};

mod keys {
  pub const CLASS: &str = "class";
  pub const SHADER: &str = "shader";
  pub const MODEL: &str = "model";
  pub const TEXTURE: &str = "texture";
}

type EntityId = Id<()>;

#[derive(Clone)]
pub struct LuaEntityId(EntityId);

impl UserData for LuaEntityId {}

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
    map: MutPtr<Map>,
    item_id: &str,
    lua: &'static Lua,
    shaders: &ShaderRepository,
    models: &ModelRepository,
    textures: &TextureRepository,
  ) -> Result<Entity, String> {
    if let Some(tmpl) = self.templates.get(item_id) {
      let id = Id::new();
      let mut entity = Entity::new(id, map);

      if let Some(class_name) = &tmpl.class {
        match script::resolve(lua, class_name) {
          Ok(LuaValue::Table(class)) => {
            match class.call_function("new", class.clone()) {
              Ok(instance) => entity.instance = Some(LuaValue::Table(instance)),
              Err(e) => self.logger.error(e.to_string()),
            }
            entity.class = Some(class);
          }
          Ok(other) => {
            self
              .logger
              .error(format!("invalid class type for entity: {:?}", other));
          }
          Err(e) => self.logger.error(e.to_string()),
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
      Err(format!("could not find {}", item_id))
    }
  }
}

impl AsPtr for EntityRepository {}

#[derive(Default)]
pub struct EntityTemplate {
  class: Option<String>,
  shader: Option<String>,
  model: Option<String>,
  texture: Option<String>,
}

#[derive(Default)]
pub struct Entity {
  id: EntityId,
  map: MutPtr<Map>,

  class: Option<LuaTable<'static>>,
  instance: Option<LuaValue<'static>>,

  shader: Option<Rc<Shader>>,
  model: Option<Rc<Model>>,
  texture: Option<Rc<SrgbTexture2d>>,
  transform: Mat4,
}

impl Entity {
  fn new(id: EntityId, map: MutPtr<Map>) -> Self {
    Self {
      id,
      map,
      ..Default::default()
    }
  }

  fn update<L: Logger>(&mut self, logger: &L) {
    if let Some(class) = &self.class {
      if let Ok(LuaValue::Function(on_update)) = class.get("update") {
        let handle = self.as_ptr_mut();
        if let Some(instance) = &self.instance {
          let res: Result<(), mlua::Error> = on_update.call((instance.clone(), handle));
          if let Err(e) = res {
            logger.error(e.to_string());
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
          let transform = glm::scale(&transform, &glm::vec3(5.0, 5.0, 1.0));
          let transform = glm::rotate(&transform, 45_f32.to_radians(), &glm::vec3(0.0, 0.0, 1.0));
          let transform: [[f32; 4]; 4] = transform.into();
          let view = camera.view();
          let proj = camera.projection();

          let uniforms = uniform! {
            model: transform,
            view: view,
            projection: proj,
            tex: &**texture,
          };

          let uniforms = uniforms.add::<f32>("c", 1.0);

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

  fn dispose(&mut self) {
    self.map.dispose_of(self.id);
  }

  fn is_mutable(&self) -> bool {
    self.class.is_some() && self.instance.is_some()
  }

  fn is_renderable(&self) -> bool {
    self.model.is_some() && self.texture.is_some()
  }
}

impl AsPtr for Entity {}

impl UserData for MutPtr<Entity> {
  fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
    fields.add_field_method_get("id", |_, this| Ok(LuaEntityId(this.id)));
  }

  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method_mut("dispose", |_, this, _: ()| {
      this.dispose();
      Ok(())
    });
    methods.add_method_mut("set_position", |_, this, vec3: LuaTable| Ok(()));
    methods.add_method_mut(
      "set_rotation",
      |_, this, (degrees, angle): (f32, LuaTable)| Ok(()),
    );
    methods.add_method_mut("set_scale", |_, this, vec3: LuaValue| Ok(()));
  }
}

#[derive(Clone)]
pub struct Tile;

#[derive(Default)]
pub struct MapData {
  pub width: usize,
  pub height: usize,
}

pub struct Map {
  data: MapData,
  tiles: Vec<Tile>,

  spawned_entities: BTreeMap<EntityId, Box<Entity>>,

  static_entities: Vec<ConstPtr<Entity>>,
  available_static_entities_positions: BTreeSet<usize>,

  mutable_entities: Vec<MutPtr<Entity>>,
  available_mutable_entities_positions: BTreeSet<usize>,

  drawable_entities: Vec<ConstPtr<Entity>>,
  available_drawable_entities_positions: BTreeSet<usize>,

  logger: ChildLogger,

  lua: &'static Lua,

  entities: ConstPtr<EntityRepository>,
  shaders: ConstPtr<ShaderRepository>,
  models: ConstPtr<ModelRepository>,
  textures: ConstPtr<TextureRepository>,
}

impl Map {
  pub fn new(
    data: MapData,
    logger: ChildLogger,
    lua: &'static Lua,
    entities: ConstPtr<EntityRepository>,
    shaders: ConstPtr<ShaderRepository>,
    models: ConstPtr<ModelRepository>,
    textures: ConstPtr<TextureRepository>,
  ) -> Self {
    let tiles = vec![Tile; data.width * data.height];
    Self {
      data,
      tiles,

      spawned_entities: Default::default(),

      static_entities: Default::default(),
      available_static_entities_positions: Default::default(),

      mutable_entities: Default::default(),
      available_mutable_entities_positions: Default::default(),

      drawable_entities: Default::default(),
      available_drawable_entities_positions: Default::default(),

      logger,

      lua,

      entities,
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

  pub fn spawn(&mut self, item_id: &str) {
    let map_ptr = self.as_ptr_mut();
    let entity = self.entities.construct(
      map_ptr,
      item_id,
      self.lua,
      &self.shaders,
      &self.models,
      &self.textures,
    );

    if let Ok(entity) = entity {
      let mut entity = Box::new(entity);

      if entity.is_renderable() {
        let ptr = ConstPtr::from(&entity);
        self.drawable_entities.push(ptr);
      }

      if entity.is_mutable() {
        let ptr = MutPtr::from(&mut entity);
        self.mutable_entities.push(ptr);
      } else {
        let ptr = ConstPtr::from(&entity);
        self.static_entities.push(ptr);
      }

      self.spawned_entities.insert(entity.id, entity);
    } else {
      self
        .logger
        .warn(format!("unable to load entity '{}'", item_id));
    }
  }

  pub fn dispose_of(&mut self, id: EntityId) {
    if let Some(entity) = self.spawned_entities.get(&id) {
      if entity.is_renderable() {
        if let Some(idx) = self.drawable_entities.iter().position(|e| e.id == id) {
          self.drawable_entities.swap_remove(idx);
        }
      }

      if entity.is_mutable() {
        if let Some(idx) = self.mutable_entities.iter().position(|e| e.id == id) {
          self.mutable_entities.swap_remove(idx);
        }
      } else if let Some(idx) = self.static_entities.iter().position(|e| e.id == id) {
        self.static_entities.swap_remove(idx);
      }

      self.spawned_entities.remove(&id);
    }
  }

  pub fn register_to_lua(&mut self, lua: &Lua) {
    let globals = lua.globals();
    globals.set("Map", self.as_ptr_mut()).unwrap();
  }

  fn lookup_entity(&mut self, id: &EntityId) -> Option<MutPtr<Entity>> {
    self
      .spawned_entities
      .get_mut(id)
      .map(|entity| entity.as_ptr_mut())
  }
}

impl AsPtr for Map {}

impl UserData for MutPtr<Map> {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method_mut("lookup_entity", |_, this, id: LuaEntityId| {
      Ok(this.lookup_entity(&id.0))
    });
  }
}
