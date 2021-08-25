use crate::render::{RenderCtx, Renderer};
use crate::widgets::{AnyEvent, LayoutResult, RenderResult, Widget};
use euclid::default::Size2D;
use std::ops::{Deref, DerefMut};

pub struct Button<Child> {
  child: Child,
}

impl<Child> Button<Child>
where
  Child: Widget,
{
  pub fn new(child: Child) -> Self {
    Self { child }
  }
}

impl Widget for Button<&str> {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) {
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    todo!()
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    ctx.renderer().write(&self.child);
    Ok(())
  }
}
