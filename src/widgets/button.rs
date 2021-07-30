use crate::render::{RenderCtx, Renderer};
use crate::widgets::Widget;
use std::ops::{Deref, DerefMut};

pub struct Button {
  text: String,
}

impl Button {
  pub fn new(text: impl Into<String>) -> Self {
    Self { text: text.into() }
  }
  pub fn text(&self) -> &String {
    &self.text
  }
  pub fn set_text(&mut self, text: impl Into<String>) -> &mut Self {
    self.text = text.into();
    self
  }
}

impl Widget for Button {
  fn event(&mut self) {
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&mut self) {
    todo!()
  }

  fn render(&self, ctx: &mut RenderCtx) {
    ctx.renderer.deref_mut().print(&self.text);
  }
}
