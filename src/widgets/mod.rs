use crate::render::{RenderCtx, Renderer};
use std::ops::Deref;

mod align;
mod button;
mod checkbox;
mod container;
mod flex;
mod label;
mod line;
mod padding;
mod progressbar;
mod scrollbar;
mod table;
mod tabs;
mod textbox;
mod vertical;

// Built-in Widgets
pub use button::Button;
pub use container::VerticalContainer;
pub use table::Column;
pub use table::{Table, TableData, TableColumns};

pub enum LayoutError {
  InsufficientSpace,
}

type LayoutResult = Result<Size2D<usize>, LayoutError>;

pub trait Widget {
  fn event(&mut self);
  fn update(&mut self);
  fn layout(&mut self, max_size: &Size2D<usize>) -> LayoutResult;
  fn render(&self, ctx: &mut RenderCtx) -> Option<()>;
}

// Wrapping Widgets
pub use align::Align;
use euclid::default::Size2D;
pub use padding::Padding;

pub struct Border;
pub struct Bread;

// Inner Widgets
pub struct Text;

// Default impl of Widgets

// impl Widget for Fn {}

impl Widget for &str {
  fn event(&mut self) {
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&mut self, max_size: &Size2D<usize>) -> LayoutResult {
    todo!()
  }

  fn render(&self, ctx: &mut RenderCtx) -> Option<()> {
    ctx.renderer.print(self);
    Some(())
  }
}

impl Widget for String {
  fn event(&mut self) {
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&mut self, max_size: &Size2D<usize>) -> LayoutResult {
    todo!()
  }

  fn render(&self, ctx: &mut RenderCtx) -> Option<()> {
    ctx.renderer.print(self);
    Some(())
  }
}

impl Widget for Box<dyn Widget> {
  fn event(&mut self) {
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&mut self, max_size: &Size2D<usize>) -> LayoutResult {
    todo!()
  }

  fn render(&self, ctx: &mut RenderCtx) -> Option<()> {
    self.deref().render(ctx);
    Some(())
  }
}
