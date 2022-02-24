use std::{
  cell::RefCell,
  ops::{Deref, DerefMut},
  ptr,
  rc::Rc,
};

pub mod prelude {
  pub use super::*;
}

pub struct ConstPtr<T: ?Sized>(*const T);

impl<T> Default for ConstPtr<T> {
  fn default() -> Self {
    Self(ptr::null())
  }
}

impl<T> ConstPtr<T> {
  pub fn new(t: &T) -> Self {
    Self(t)
  }

  pub fn raw(&self) -> *const T {
    self.0
  }

  pub fn null(&self) -> bool {
    self.0 == ptr::null()
  }
}

impl<T> AsRef<T> for ConstPtr<T> {
  fn as_ref(&self) -> &T {
    unsafe { &*self.raw() }
  }
}

impl<T> Clone for ConstPtr<T> {
  fn clone(&self) -> Self {
    Self(self.0)
  }
}

impl<T> Copy for ConstPtr<T> {}

impl<T> Deref for ConstPtr<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    unsafe { &*self.0 }
  }
}

impl<T> From<MutPtr<T>> for ConstPtr<T> {
  fn from(ptr: MutPtr<T>) -> Self {
    Self(ptr.raw())
  }
}

impl<T> From<Rc<T>> for ConstPtr<T> {
  fn from(ptr: Rc<T>) -> Self {
    Self(ptr.as_ref())
  }
}

impl<T> From<&Box<T>> for ConstPtr<T> {
  fn from(ptr: &Box<T>) -> Self {
    Self(ptr.as_ref())
  }
}

pub struct MutPtr<T: ?Sized>(*mut T);

impl<T> Default for MutPtr<T> {
  fn default() -> Self {
    Self(ptr::null_mut())
  }
}

impl<T> Clone for MutPtr<T> {
  fn clone(&self) -> Self {
    Self(self.0)
  }
}

impl<T> Copy for MutPtr<T> {}

impl<T> MutPtr<T> {
  pub fn new(t: &mut T) -> Self {
    Self(t)
  }

  pub fn raw(&self) -> *mut T {
    self.0
  }

  pub fn null(&self) -> bool {
    self.0 == ptr::null_mut()
  }
}

impl<T> AsRef<T> for MutPtr<T> {
  fn as_ref(&self) -> &T {
    unsafe { &*self.raw() }
  }
}

impl<T> AsMut<T> for MutPtr<T> {
  fn as_mut(&mut self) -> &mut T {
    unsafe { &mut *self.raw() }
  }
}

impl<T> Deref for MutPtr<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    unsafe { &*self.0 }
  }
}

impl<T> DerefMut for MutPtr<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *self.0 }
  }
}

impl<T> From<&mut Box<T>> for MutPtr<T> {
  fn from(ptr: &mut Box<T>) -> Self {
    Self(ptr.as_mut())
  }
}

impl<T> From<Rc<RefCell<T>>> for MutPtr<T> {
  fn from(ptr: Rc<RefCell<T>>) -> Self {
    Self(ptr.as_ptr())
  }
}

pub trait AsPtr {
  fn as_ptr(&self) -> ConstPtr<Self>
  where
    Self: Sized,
  {
    ConstPtr(self)
  }

  fn as_ptr_mut(&mut self) -> MutPtr<Self>
  where
    Self: Sized,
  {
    MutPtr(self)
  }
}
