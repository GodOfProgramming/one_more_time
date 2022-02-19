use crate::{
  gfx::*,
  math::glm,
  scripting::prelude::*,
  util::{DirID, Logger},
};
use imgui_glium_renderer::glium::{
  backend::Facade,
  index::{IndexBuffer, PrimitiveType},
  texture::{RawImage2d, SrgbTexture2d},
  uniform,
  vertex::VertexBuffer,
  Surface,
};
use mlua::Table;
use std::{collections::BTreeMap, fs, path::PathBuf, rc::Rc};
use toml::Value;

mod keys {
  pub const SCRIPT: &str = "script";
  pub const ON_UPDATE: &str = "on_update";
  pub const SHADER: &str = "shader";
  pub const MODEL: &str = "model";
  pub const TEXTURE: &str = "texture";
}

pub struct Model {
  vertices: Vertices,
  indices: Indices,
  primitive: PrimitiveType,
  vbuff: VertexBuffer<Vertex>,
  ibuff: IndexBuffer<u32>,
}

impl Model {
  pub fn new<F: Facade>(
    facade: &F,
    vertices: Vertices,
    indices: Indices,
    primitive: PrimitiveType,
  ) -> Result<Model, String> {
    let vbuff = VertexBuffer::new(facade, &vertices).map_err(|err| err.to_string())?;
    let ibuff = IndexBuffer::new(facade, primitive, &indices).map_err(|err| err.to_string())?;
    Ok(Self {
      vertices,
      indices,
      primitive,
      vbuff,
      ibuff,
    })
  }
}

pub struct ModelRepository {
  models: BTreeMap<String, Rc<Model>>,
}

impl ModelRepository {
  pub fn new<F: Facade>(facade: &F) -> Self {
    let mut repo = Self {
      models: Default::default(),
    };

    let sprite = Sprite::new();
    repo.models.insert(
      "sprite".to_string(),
      Rc::new(
        Model::new(
          facade,
          sprite.vertices,
          sprite.indices,
          PrimitiveType::TrianglesList,
        )
        .unwrap(),
      ),
    );

    repo
  }

  pub fn get(&self, id: &str) -> Option<Rc<Model>> {
    self.models.get(id).cloned()
  }
}

#[derive(Default)]
pub struct EntityRepository {
  templates: BTreeMap<String, Rc<EntityTemplate>>,
}

impl EntityRepository {
  pub fn new<L, I>(logger: &L, iter: I) -> Self
  where
    L: Logger,
    I: Iterator<Item = (PathBuf, DirID)>,
  {
    let mut repo = Self::default();

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

                let on_update = if let Some(Value::String(on_update)) = v.get(keys::ON_UPDATE) {
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
                  on_update,
                  shader,
                  model,
                  texture,
                };

                repo.templates.insert(id.into(), Rc::new(tmpl));
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

    repo
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
          if let Some(on_update) = &tmpl.on_update {
            entity.on_update = on_update.clone();
            if let Ok(table) = lua.create_table() {
              entity.data = Some(table);
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
  on_update: Option<String>,
  shader: Option<String>,
  model: Option<String>,
  texture: Option<String>,
}

#[derive(Default)]
pub struct Entity {
  lua: Option<&'static Lua>,
  on_update: String,
  shader: Option<Rc<Shader>>,
  model: Option<Rc<Model>>,
  texture: Option<Rc<SrgbTexture2d>>,
  data: Option<Table<'static>>,
}

impl Entity {
  pub fn update(&mut self) {
    let lua_type = self.create_lua_type();
    if let Some(lua) = &self.lua {
      let res: Result<(), mlua::Error> = lua
        .globals()
        .call_function(self.on_update.as_str(), lua_type);
      if let Err(_e) = res {
        // todo
      }
    }
  }

  pub fn draw<S: Surface>(&self, surface: &mut S) {
    if let Some(shader) = &self.shader {
      if let Some(model) = &self.model {
        if let Some(texture) = &self.texture {
          let uniforms = uniform! {
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

impl LuaType<Entity> {}

impl LuaTypeTrait for Entity {}

impl UserData for LuaType<Entity> {}

#[derive(Clone)]
pub struct Tile;

pub struct MapData {
  width: usize,
  height: usize,
}

struct Map {
  data: MapData,
  tiles: Vec<Tile>,
  entities: BTreeMap<String, Entity>,
}

impl Map {
  pub fn new(data: MapData) -> Self {
    Self {
      tiles: vec![Tile; data.width * data.height],
      entities: Default::default(),
      data,
    }
  }

  pub fn update(&mut self) {
    for entity in self.entities.values_mut() {
      entity.update();
    }
  }

  pub fn spawn_entity(id: String, name: String) {}
}
