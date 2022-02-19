use crate::{gfx::*, scripting::prelude::*};
use imgui_glium_renderer::glium::{
  backend::Facade,
  index::{IndexBuffer, PrimitiveType},
  uniforms::UniformsStorage,
  vertex::VertexBuffer,
};
use std::{collections::BTreeMap, rc::Rc};

pub struct Model {
  vertices: Vertices,
  indices: Indices,
  primitive: PrimitiveType,
}

impl Model {
  pub fn new(vertices: Vertices, indices: Indices, primitive: PrimitiveType) -> Self {
    Self {
      vertices,
      indices,
      primitive,
    }
  }

  pub fn create_buffers<F: Facade>(self, facade: &F) -> Result<Buffer, String> {
    let vbuff = VertexBuffer::new(facade, &self.vertices).map_err(|err| err.to_string())?;
    let ibuff =
      IndexBuffer::new(facade, self.primitive, &self.indices).map_err(|err| err.to_string())?;
    Ok(Buffer::new(vbuff, ibuff))
  }
}

pub struct Buffer {
  vbuff: VertexBuffer<Vertex>,
  ibuff: IndexBuffer<u32>,
}

impl Buffer {
  fn new(vbuff: VertexBuffer<Vertex>, ibuff: IndexBuffer<u32>) -> Self {
    Self { vbuff, ibuff }
  }
}

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
