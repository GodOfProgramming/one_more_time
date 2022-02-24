use crate::util::prelude::*;
pub use omt::glm;

pub mod geom;

#[derive(Debug, Clone, Copy)]
pub struct Dim<T: Sized + std::fmt::Debug + Clone + Copy> {
  pub x: T,
  pub y: T,
}

impl<T> Dim<T>
where
  T: Sized + std::fmt::Debug + Clone + Copy,
{
  pub fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

impl AsPtr for Dim<u32> {}
