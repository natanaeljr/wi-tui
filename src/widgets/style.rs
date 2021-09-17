use std::ops::{BitOr, Deref, DerefMut};

pub use crossterm::style::Attribute;
pub use crossterm::style::Attributes;
pub use crossterm::style::Color;
use crossterm::style::Stylize;
use euclid::default::Size2D;

use crate::render::RenderCtx;
use crate::widgets::{AnyEvent, Capability, EventResult, LayoutResult, RenderResult, Widget};
use crate::{FlexFit, Style};

pub struct Styled<Child> {
  pub style: Style,
  pub child: Child,
}

impl<Child> Widget for Styled<Child>
where
  Child: Widget,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    self.child.event(event, size)
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    self.child.layout(avail_size)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    if !self.style.attrs.is_empty() {
      ctx.renderer().add_attributes(self.style.attrs);
    }
    if let Some(bg) = self.style.bg.as_ref() {
      ctx.renderer().set_background(bg);
    }
    if let Some(fg) = self.style.fg.as_ref() {
      ctx.renderer().set_foreground(fg);
    }
    self.child.render(ctx)
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    self.child.has_capability(capability)
  }
}
