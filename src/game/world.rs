use crate::{gfx::*, scripting::prelude::*};
use imgui_glium_renderer::glium::{
  backend::Facade,
  index::{IndexBuffer, PrimitiveType},
  uniforms::UniformsStorage,
  vertex::VertexBuffer,
};
use mlua::Value;
use std::{collections::BTreeMap, rc::Rc};

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

pub struct EneityRepository {}

pub struct Entity {
  lua: Option<Rc<Lua>>,
}

impl Entity {
  pub fn update(&mut self) {
    let lua_type = self.create_lua_type();
    if let Some(lua) = &self.lua {
      let res: Result<(), mlua::Error> = lua.globals().call_function("update", lua_type);
      if let Err(_e) = res {
        // todo
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
    for (_name, entity) in &mut self.entities {
      entity.update();
    }
  }

  pub fn spawn_entity(id: String, name: String) {}
}
