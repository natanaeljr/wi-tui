use crate::render::{RenderCtx, Renderer};
use crate::widgets::Widget;
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
  fn event(&mut self) {
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&mut self) {
    todo!()
  }

  fn render(&self, ctx: &mut RenderCtx) -> Option<()> {
    ctx.renderer.print(&self.child);
    Some(())
  }
}
