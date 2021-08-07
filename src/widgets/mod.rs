use crate::render::{RenderCtx, Renderer};
use euclid::default::Size2D;
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
pub mod table;
mod tabs;
mod textbox;
mod vertical;

// Built-in Widgets
pub use button::Button;
pub use container::VerticalContainer;
pub use table::Table;

#[derive(Debug)]
pub enum LayoutError {
  InsufficientSpace,
}

impl Display for LayoutError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      LayoutError::InsufficientSpace => {
        write!(f, "Layout error: insufficient space")
      }
    }
  }
}

impl Error for LayoutError {}

#[derive(Debug)]
pub enum RenderError {
  Layout(LayoutError),
}

impl Display for RenderError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      RenderError::Layout(err) => {
        write!(f, "Render error -> {}", err)
      }
    }
  }
}

impl Error for RenderError {}

type LayoutResult = Result<Size2D<usize>, LayoutError>;
type RenderResult = Result<(), RenderError>;

pub trait Widget {
  fn event(&mut self);
  fn update(&mut self);
  /// layout must return the **minimum** required space for drawing this widget
  fn layout(&self, max_size: &Size2D<usize>) -> LayoutResult;
  fn render(&self, ctx: &mut RenderCtx) -> RenderResult;
}

// Wrapping Widgets
pub use align::Align;
pub use padding::Padding;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub struct Border;
pub struct Bread;

// Inner Widgets
pub struct Text;

// TODO: Default impl of Widgets
// impl Widget for Fn {}
// impl Widget for Rc {}
// impl Widget for Cell {}
// impl Widget for RefCell {}

impl Widget for &str {
  fn event(&mut self) {
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&self, max_size: &Size2D<usize>) -> LayoutResult {
    let size = Size2D::new(self.len(), 1);
    if max_size.contains(size.clone()) {
      Ok(size)
    } else {
      Err(LayoutError::InsufficientSpace)
    }
  }

  fn render(&self, ctx: &mut RenderCtx) -> RenderResult {
    ctx.renderer().print(self);
    Ok(())
  }
}

impl Widget for String {
  fn event(&mut self) {
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&self, max_size: &Size2D<usize>) -> LayoutResult {
    let size = Size2D::new(self.len(), 1);
    if max_size.contains(size.clone()) {
      Ok(size)
    } else {
      Err(LayoutError::InsufficientSpace)
    }
  }

  fn render(&self, ctx: &mut RenderCtx) -> RenderResult {
    ctx.renderer().print(self);
    Ok(())
  }
}

impl Widget for u32 {
  fn event(&mut self) {
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&self, max_size: &Size2D<usize>) -> LayoutResult {
    let val = format!("{}", self);
    let size = Size2D::new(val.len(), 1);
    if max_size.contains(size.clone()) {
      Ok(size)
    } else {
      Err(LayoutError::InsufficientSpace)
    }
  }

  fn render(&self, ctx: &mut RenderCtx) -> RenderResult {
    let val = format!("{}", self);
    ctx.renderer().print(&val);
    Ok(())
  }
}

impl Widget for Box<dyn Widget> {
  fn event(&mut self) {
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&self, max_size: &Size2D<usize>) -> LayoutResult {
    self.deref().layout(max_size)
  }

  fn render(&self, ctx: &mut RenderCtx) -> RenderResult {
    self.deref().render(ctx);
    Ok(())
  }
}
