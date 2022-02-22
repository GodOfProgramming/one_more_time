use super::*;
use crate::util::prelude::*;
use omt::glium::{
  backend::Facade,
  index::{IndexBuffer, PrimitiveType},
  vertex::VertexBuffer,
};
use std::{collections::BTreeMap, rc::Rc};

pub struct Model {
  vertices: Vertices,
  indices: Indices,
  pub primitive: PrimitiveType,
  pub vbuff: VertexBuffer<Vertex>,
  pub ibuff: IndexBuffer<u32>,
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

impl AsPtr for ModelRepository {}
