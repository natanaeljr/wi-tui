use crate::render::RenderCtx;
use crate::widgets::{LayoutResult, RenderResult, Widget};
use euclid::default::Size2D;
use std::ops::BitOr;

pub use crossterm::style::Attribute;
pub use crossterm::style::Attributes;
pub use crossterm::style::Color;

pub struct Style<Child> {
  pub fg: Option<Color>,
  pub bg: Option<Color>,
  pub attrs: Attributes,
  pub child: Child,
}

impl<Child> Style<Child> {
  pub fn new(child: Child) -> Self {
    Self {
      fg: None,
      bg: None,
      attrs: Default::default(),
      child,
    }
  }

  pub fn fg(mut self, color: Color) -> Self {
    self.fg = Some(color);
    self
  }

  pub fn bg(mut self, color: Color) -> Self {
    self.bg = Some(color);
    self
  }

  pub fn attr(mut self, attr: Attribute) -> Self {
    self.attrs = self.attrs | attr;
    self
  }
}

impl<Child> Widget for Style<Child>
where
  Child: Widget,
{
  fn event(&mut self) {
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    self.child.layout(parent_size)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    if !self.attrs.is_empty() {
      ctx.renderer().add_attributes(self.attrs);
    }
    if let Some(bg) = self.bg.as_ref() {
      ctx.renderer().set_background(bg);
    }
    if let Some(fg) = self.fg.as_ref() {
      ctx.renderer().set_foreground(fg);
    }
    self.child.render(ctx)
  }
}
