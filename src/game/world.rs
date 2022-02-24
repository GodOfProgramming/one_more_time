use super::Camera;
use crate::{gfx::*, math::glm::Mat4, util::prelude::*};
use omt::{
  core::{EntityHandle, EntityInstance, EntityModel},
  glium::{texture::SrgbTexture2d, uniform, Surface},
  uid::Id,
};
use std::{collections::BTreeMap, rc::Rc};

mod keys {
  pub const CLASS: &str = "class";
  pub const SHADER: &str = "shader";
  pub const MODEL: &str = "model";
  pub const TEXTURE: &str = "texture";
}

type EntityId = Id<()>;

pub struct EntityArchive {
  templates: BTreeMap<String, Rc<EntityTemplate>>,
  logger: ChildLogger,
}

impl EntityArchive {
  pub fn new(logger: ChildLogger) -> Self {
    Self {
      logger,
      templates: Default::default(),
    }
  }

  pub fn register(name: &str) {}

  pub fn construct(
    &self,
    item_id: &str,
    map: MutPtr<Map>,
    shaders: &ShaderProgramArchive,
    models: &ModelRepository,
    textures: &TextureRepository,
  ) -> Result<Entity, String> {
    if let Some(tmpl) = self.templates.get(item_id) {
      let id = Id::new();
      let instance = tmpl.entity_model.new_instance();
      let mut entity = Entity::new(id, map, instance);

      if let Some(shader) = &tmpl.shader {
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

      Ok(entity)
    } else {
      Err(format!("could not find {}", item_id))
    }
  }
}

impl AsPtr for EntityArchive {}

pub struct EntityTemplate {
  entity_model: Box<dyn EntityModel>,
  shader: Option<Rc<Shader>>,
  model: Option<String>,
  texture: Option<String>,
}

pub struct Entity {
  id: EntityId,
  map: MutPtr<Map>,

  instance: Box<dyn EntityInstance>,

  shader: Option<Rc<Shader>>,
  model: Option<Rc<Model>>,
  texture: Option<Rc<SrgbTexture2d>>,
  transform: Mat4,
}

impl Entity {
  fn new(id: EntityId, map: MutPtr<Map>, instance: Box<dyn EntityInstance>) -> Self {
    Self {
      id,
      map,
      instance,
      shader: Default::default(),
      model: Default::default(),
      texture: Default::default(),
      transform: Mat4::identity(),
    }
  }

  fn update(&mut self) {
    let mut ptr = self.as_ptr_mut();
    self.instance.update(&mut *ptr);
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

  fn should_update(&self) -> bool {
    self.instance.should_update()
  }

  fn is_renderable(&self) -> bool {
    self.model.is_some() && self.texture.is_some()
  }
}

impl EntityHandle for Entity {
  fn dispose(&mut self) {
    self.map.dispose_of(self.id);
  }
}

impl AsPtr for Entity {}

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

  drawable_entities: Vec<ConstPtr<Entity>>,

  logger: ChildLogger,

  entities: ConstPtr<EntityArchive>,
  shaders: ConstPtr<ShaderProgramArchive>,
  models: ConstPtr<ModelRepository>,
  textures: ConstPtr<TextureRepository>,
}

impl Map {
  pub fn new(
    data: MapData,
    logger: ChildLogger,
    entities: ConstPtr<EntityArchive>,
    shaders: ConstPtr<ShaderProgramArchive>,
    models: ConstPtr<ModelRepository>,
    textures: ConstPtr<TextureRepository>,
  ) -> Self {
    let tiles = vec![Tile; data.width * data.height];
    Self {
      data,
      tiles,

      spawned_entities: Default::default(),

      drawable_entities: Default::default(),

      logger,

      entities,
      shaders,
      models,
      textures,
    }
  }

  pub fn update(&mut self) {
    for entity in &mut self
      .spawned_entities
      .values_mut()
      .filter(|e| e.should_update())
    {
      entity.update();
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
      item_id,
      map_ptr,
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

      self.spawned_entities.remove(&id);
    }
  }

  fn lookup_entity(&mut self, id: &EntityId) -> Option<MutPtr<Entity>> {
    self
      .spawned_entities
      .get_mut(id)
      .map(|entity| entity.as_ptr_mut())
  }
}

impl AsPtr for Map {}
