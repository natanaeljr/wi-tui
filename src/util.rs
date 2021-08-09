use crossterm::event::KeyCode::Delete;
use std::ops::Deref;

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
