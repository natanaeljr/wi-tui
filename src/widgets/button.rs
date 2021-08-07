use crate::render::{RenderCtx, Renderer};
use crate::widgets::{Widget, LayoutResult, RenderResult};
use std::ops::{Deref, DerefMut};
use euclid::default::Size2D;

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
  fn event(&mut self) {
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    todo!()
  }

  fn render(&self, ctx: &mut RenderCtx) -> RenderResult {
    ctx.renderer().print(&self.child);
    Ok(())
  }
}
