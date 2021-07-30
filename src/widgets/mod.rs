use crate::render::{RenderCtx, Renderer};

mod align;
mod button;
mod checkbox;
mod label;
mod line;
mod progressbar;
mod scrollbar;
mod table;
mod tabs;
mod textbox;

pub use button::Button;
pub use table::Column;
pub use table::Table;

pub trait Widget {
  fn event(&mut self);
  fn update(&mut self);
  fn layout(&mut self);
  fn render(&self, ctx: &mut RenderCtx);
}

// Wrapping Widgets
pub use align::Align;
use std::ops::Deref;

pub struct Padding;
pub struct Border;
pub struct Bread;

// Inner Widgets
pub struct Text;

// Default impl of Widgets
impl Widget for &str {
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
    ctx.renderer.print(self);
  }
}

impl Widget for String {
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
    ctx.renderer.print(self);
  }
}

impl Widget for Box<dyn Widget> {
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
    self.deref().render(ctx);
  }
}
