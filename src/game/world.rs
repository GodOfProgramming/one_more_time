use super::Camera;
use crate::{
  gfx::*,
  glium::{texture::SrgbTexture2d, uniform, Program, Surface},
  util::prelude::*,
};
use omt::core::{EntityHandle, EntityInstance, EntityModel, EntityModelLoader};
use std::{collections::BTreeMap, rc::Rc};
use uid::Id;

mod keys {
  pub const CLASS: &str = "class";
  pub const SHADER: &str = "shader";
  pub const MODEL: &str = "model";
  pub const TEXTURE: &str = "texture";
}

#[derive(Default)]
pub struct EntityModelArchive {
  models: BTreeMap<String, Box<dyn EntityModel>>,
}

impl EntityModelLoader for EntityModelArchive {
  fn register(&mut self, name: &str, model: Box<dyn EntityModel>) {
    self.models.insert(name.to_string(), model);
  }
}

type EntityId = Id<()>;

pub struct EntityArchive {
  templates: BTreeMap<String, EntityTemplate>,
  logger: ChildLogger,
}

impl EntityArchive {
  pub fn new(logger: ChildLogger) -> Self {
    Self {
      logger,
      templates: Default::default(),
    }
  }

  pub fn add_model_archive(&mut self, archive: EntityModelArchive) {
    for (id, model) in archive.models {
      self.create_template(id, model)
    }
  }

  pub fn create_template(&mut self, id: String, model: Box<dyn EntityModel>) {
    let template = EntityTemplate::new(model);
    self.templates.insert(id, template);
  }

  pub fn construct(
    &self,
    item_id: &str,
    map: MutPtr<Map>,
    shaders: &ShaderProgramArchive,
    models: &ModelRepository,
    textures: &TextureArchive,
  ) -> Result<Entity, String> {
    if let Some(tmpl) = self.templates.get(item_id) {
      let id = Id::new();
      let instance = tmpl.model.new_instance();
      let mut entity = Entity::new(id, map, instance);

      if let Some(shader) = &tmpl.shader {
        if let Some(shader) = shaders.get(shader) {
          entity.shader = Some(shader.clone());
        }
      }

      if let Some(model) = &tmpl.shape {
        if let Some(model) = models.get(model) {
          entity.model = Some(model.clone());
        }
      }

      if let Some(texture) = &tmpl.sprite {
        if let Some(texture) = textures.get(texture) {
          entity.texture = Some(texture.clone());
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
  model: Box<dyn EntityModel>,
  shader: Option<&'static str>,
  shape: Option<&'static str>,
  sprite: Option<&'static str>,
}

impl EntityTemplate {
  fn new(model: Box<dyn EntityModel>) -> Self {
    let shader = model.shader();
    let shape = model.shape();
    let sprite = model.sprite();

    Self {
      model,
      shader,
      shape,
      sprite,
    }
  }
}

pub struct Entity {
  id: EntityId,
  map: MutPtr<Map>,

  instance: Box<dyn EntityInstance>,

  shader: Option<Rc<Program>>,
  model: Option<Rc<Model>>,
  texture: Option<Rc<SrgbTexture2d>>,
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
          let transform: [[f32; 4]; 4] = self.instance.transform().into();
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
  textures: ConstPtr<TextureArchive>,
}

impl Map {
  pub fn new(
    data: MapData,
    logger: ChildLogger,
    entities: ConstPtr<EntityArchive>,
    shaders: ConstPtr<ShaderProgramArchive>,
    models: ConstPtr<ModelRepository>,
    textures: ConstPtr<TextureArchive>,
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
