use crate::render::{RenderCtx, Renderer};
use crate::FlexFit;
use crate::widgets::{AnyEvent, Capability, EventResult, LayoutResult, RenderResult, Widget};
use euclid::default::Size2D;
use std::ops::{Deref, DerefMut};

pub struct Button<Child> {
  child: Child,
}

impl<Child> Button<Child>
where
  Child: Widget,
{
  pub fn child(child: Child) -> Self {
    Self { child }
  }
}

impl Widget for Button<&str> {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    todo!()
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    todo!()
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    ctx.renderer().write(&self.child);
    Ok(())
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    todo!()
  }
}
