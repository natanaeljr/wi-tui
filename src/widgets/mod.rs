use crate::render::{RenderCtx, Renderer};
use crate::util::Scoped;
use crossterm::style::StyledContent;
use euclid::default::Size2D;
use std::cell::{Cell, RefCell};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use crate::debug;

pub mod align;
pub mod frame;
pub mod button;
pub mod checkbox;
pub mod container;
pub mod expand;
pub mod fillchar;
pub mod flex;
pub mod label;
pub mod leak;
pub mod line;
pub mod padding;
pub mod progressbar;
pub mod repeat;
pub mod scrollbar;
pub mod style;
pub mod table;
pub mod tabs;
pub mod textbox;
pub mod vertical;

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

#[derive(Debug, Default)]
pub struct LayoutSize {
  min: Size2D<usize>,
  max: Size2D<usize>,
}

pub type LayoutResult = Result<LayoutSize, LayoutError>;
pub type RenderResult = Result<(), RenderError>;

pub enum EventResult {
  Unhandled,
  Done,
  LockMouseClick,
  PopupMenu { options: Vec<Box<dyn Widget>> },
}

pub enum AnyEvent {
  Input(crossterm::event::Event),
}

pub trait Widget {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult;
  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult;
  fn render(&self, ctx: &RenderCtx) -> RenderResult;
}

// TODO: Default impl of Widgets
// impl Widget for Fn {}

impl Widget for &str {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
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

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let parent_size = ctx.get_frame().size.clone();
    if parent_size.width < self.len() {
      let buf = self.split_at(parent_size.width.checked_sub(1).unwrap_or(0)).0;
      ctx.renderer().write(buf);
      ctx.renderer().write("…");
    } else {
      ctx.renderer().write(self);
    }
    Ok(())
  }
}

impl Widget for String {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
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

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let parent_size = ctx.get_frame().size.clone();
    if parent_size.width < self.len() {
      let buf = self.split_at(parent_size.width.checked_sub(1).unwrap_or(0)).0;
      ctx.renderer().write(buf);
      ctx.renderer().write("…");
    } else {
      ctx.renderer().write(self);
    }
    Ok(())
  }
}

impl Widget for char {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    todo!()
  }

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    let size = Size2D::new(1, 1);
    // check for minimum space in parent size
    if parent_size.contains(size.clone()) {
      Ok(LayoutSize {
        min: size.clone(),
        max: size,
      })
    } else {
      Err(LayoutError::InsufficientSpace)
    }
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    ctx.renderer().write(self.to_string().as_str());
    Ok(())
  }
}

impl Widget for u32 {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
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

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let val = format!("{}", self);
    let parent_size = ctx.get_frame().size.clone();
    if parent_size.width < val.len() {
      let buf = val.split_at(parent_size.width.checked_sub(1).unwrap_or(0)).0;
      ctx.renderer().write(buf);
      ctx.renderer().write("…");
    } else {
      ctx.renderer().write(val.as_str());
    }
    Ok(())
  }
}

impl Widget for usize {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
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

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let val = format!("{}", self);
    let parent_size = ctx.get_frame().size.clone();
    if parent_size.width < val.len() {
      let buf = val.split_at(parent_size.width.checked_sub(1).unwrap_or(0)).0;
      ctx.renderer().write(buf);
      ctx.renderer().write("…");
    } else {
      ctx.renderer().write(val.as_str());
    }
    Ok(())
  }
}

impl<T> Widget for Rc<T>
where
  T: ?Sized + Widget,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    if let Some(inner) = Rc::get_mut(self) {
      inner.event(event, size)
    } else {
      EventResult::Unhandled
    }
  }

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    self.deref().layout(parent_size)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    self.deref().render(ctx)
  }
}

impl<T> Widget for RefCell<T>
where
  T: ?Sized + Widget,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    self.get_mut().event(event, size)
  }

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    self.deref().layout(parent_size)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    self.deref().render(ctx)
  }
}

impl<T> Widget for Box<T>
where
  T: ?Sized + Widget,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    self.deref_mut().event(event, size)
  }

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    self.deref().layout(parent_size)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    self.deref().render(ctx);
    Ok(())
  }
}

impl Widget for () {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    EventResult::Unhandled
  }

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    debug!("layout() : parent_size: {:?}", parent_size);
    let layout = LayoutSize {
      min: Default::default(),
      max: Default::default(),
    };
    debug!("layout() : layout: {:?}", layout);
    Ok(layout)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    debug!("render() : frame: {:?}, ", &ctx.get_frame());
    Ok(())
  }
}

impl<T> Widget for StyledContent<T>
where
  T: Widget + std::fmt::Display,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    todo!()
  }

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    self.content().layout(parent_size)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    if let Some(bg) = self.style().background_color.as_ref() {
      ctx.renderer().set_background(bg);
    }
    if let Some(fg) = self.style().foreground_color.as_ref() {
      ctx.renderer().set_foreground(fg);
    }
    if !self.style().attributes.is_empty() {
      ctx.renderer().add_attributes(self.style().attributes);
    }
    self.content().render(ctx)
  }
}
