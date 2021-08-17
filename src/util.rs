use std::ops::Deref;

use crossterm::event::KeyCode::Delete;

pub enum Scoped<'r, Type: ?Sized> {
  Ref(&'r Type),
  Box(Box<Type>),
}

impl<'r, Type: ?Sized> Deref for Scoped<'r, Type> {
  type Target = Type;

  fn deref(&self) -> &Self::Target {
    match self {
      Scoped::Ref(r) => &**r,
      Scoped::Box(b) => b.deref(),
    }
  }
}

#[derive(Debug)]
pub struct MinMax<T> {
  pub min: T,
  pub max: T,
}

impl<T> MinMax<T> {
  pub fn new(min: T, max: T) -> Self {
    Self { min, max }
  }
}

impl<T> Default for MinMax<T>
where
  T: Default,
{
  fn default() -> Self {
    MinMax {
      min: Default::default(),
      max: Default::default(),
    }
  }
}

pub struct Immut<Inner> {
  inner: Inner,
}

impl<Inner> Immut<Inner> {
  pub fn new(inner: Inner) -> Self {
    Self { inner }
  }
}

impl<Inner> Deref for Immut<Inner> {
  type Target = Inner;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

pub trait Immutable {
  fn immut(self) -> Immut<Self>
  where
    Self: Sized,
  {
    Immut::new(self)
  }
}
