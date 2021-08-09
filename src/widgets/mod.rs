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
  Unknown,
}

impl Display for LayoutError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      LayoutError::InsufficientSpace => {
        write!(f, "Layout error: insufficient space")
      }
      LayoutError::Unknown => {
        write!(f, "Layout error: unknown")
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

#[derive(Debug)]
pub struct LayoutSize {
  min: Size2D<usize>,
  max: Size2D<usize>,
}

pub type LayoutResult = Result<LayoutSize, LayoutError>;
pub type RenderResult = Result<(), RenderError>;

pub trait Widget {
  fn event(&mut self);
  fn update(&mut self);
  /// layout must return the **minimum** required space for drawing this widget
  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult;
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

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    let min = Size2D::new(1, 1);
    let mut max = Size2D::new(self.len(), 1);
    // clamp max size to parent size
    // max.width = std::cmp::min(max.width, parent_size.width);
    // max.height = std::cmp::min(max.height, parent_size.height);
    // check for minimum space in parent size
    if parent_size.contains(min.clone()) {
      Ok(LayoutSize { min, max })
    } else {
      Err(LayoutError::InsufficientSpace)
    }
  }

  fn render(&self, ctx: &mut RenderCtx) -> RenderResult {
    let parent_size = ctx.get_frame_size();
    let mut string = String::new();
    let mut buf = *self;
    if parent_size.width < self.len() {
      buf = self.split_at(parent_size.width.checked_sub(1).unwrap_or(0)).0;
      string = format!("{}…", buf);
      buf = string.as_str();
    }
    ctx.renderer().print(buf);
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

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    let min = Size2D::new(1, 1);
    let mut max = Size2D::new(self.len(), 1);
    // clamp max size to parent size
    // max.width = std::cmp::min(max.width, parent_size.width);
    // max.height = std::cmp::min(max.height, parent_size.height);
    // check for minimum space in parent size
    if parent_size.contains(min.clone()) {
      Ok(LayoutSize { min, max })
    } else {
      Err(LayoutError::InsufficientSpace)
    }
  }

  fn render(&self, ctx: &mut RenderCtx) -> RenderResult {
    let parent_size = ctx.get_frame_size();
    let mut string = String::new();
    let mut buf = self.as_str();
    if parent_size.width < self.len() {
      buf = self.split_at(parent_size.width.checked_sub(1).unwrap_or(0)).0;
      string = format!("{}…", buf);
      buf = string.as_str();
    }
    ctx.renderer().print(buf);
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

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    let value = format!("{}", self);
    let min = Size2D::new(1, 1);
    let mut max = Size2D::new(value.len(), 1);
    // clamp max size to parent size
    max.width = std::cmp::min(max.width, parent_size.width);
    max.height = std::cmp::min(max.height, parent_size.height);
    // check for minimum space in parent size
    if parent_size.contains(min.clone()) {
      Ok(LayoutSize { min, max })
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

impl Widget for usize {
  fn event(&mut self) {
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    let value = format!("{}", self);
    let min = Size2D::new(1, 1);
    let mut max = Size2D::new(value.len(), 1);
    // clamp max size to parent size
    max.width = std::cmp::min(max.width, parent_size.width);
    max.height = std::cmp::min(max.height, parent_size.height);
    // check for minimum space in parent size
    if parent_size.contains(min.clone()) {
      Ok(LayoutSize { min, max })
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

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    self.deref().layout(parent_size)
  }

  fn render(&self, ctx: &mut RenderCtx) -> RenderResult {
    self.deref().render(ctx);
    Ok(())
  }
}
