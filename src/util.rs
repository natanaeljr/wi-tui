use std::ops::Deref;

pub enum Scoped<'r, Type: ?Sized> {
  Ref(&'r Type),
  Box(Box<Type>),
}

impl<'r, Type> Deref for Scoped<'r, Type> {
  type Target = Type;

  fn deref(&self) -> &Self::Target {
    match self {
      Scoped::Ref(r) => &**r,
      Scoped::Box(b) => b.deref(),
    }
  }
}
