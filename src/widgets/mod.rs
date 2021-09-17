use crate::log::debug;
use crate::render::{RenderCtx, Renderer};
use crate::util::Scoped;
use crate::FlexFit;
use crossterm::style::StyledContent;
use euclid::default::Size2D;
use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

//
// Export std Widgets
//
pub use align::Align;
pub use borders::Borders;
pub use button::Button;
pub use checkbox::CheckBox;
pub use columnview::ColumnView;
pub use expanded::Expanded;
pub use fillchar::FillChar;
pub use flexible::Flexible;
pub use hook::Hook;
pub use leak::Leak;
pub use min::Min;
pub use padding::Padding;
pub use repeat::Repeat;
pub use rowview::RowView;
pub use stack::Stack;
pub use styled::Styled;
#[doc(inline)]
pub use table::Table;
#[doc(inline)]
pub use text::Text;

mod align;
mod borders;
mod button;
mod checkbox;
mod columnview;
mod expanded;
mod fillchar;
mod flex;
mod flexible;
mod hook;
mod input;
mod leak;
mod min;
mod padding;
mod progressbar;
mod repeat;
mod rowview;
mod scrollbar;
mod stack;
mod styled;
pub mod table;
mod tabs;
pub mod text;

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
  pub min: Size2D<usize>,
  pub max: Size2D<usize>,
  pub flex: usize,
  pub fit: FlexFit,
}

impl LayoutSize {
  pub fn min_max(min: Size2D<usize>, max: Size2D<usize>) -> Self {
    Self {
      min,
      max,
      flex: 0,
      fit: FlexFit::Tight,
    }
  }

  pub fn min(mut self, min: Size2D<usize>) -> Self {
    self.min = min;
    self
  }

  pub fn max(mut self, max: Size2D<usize>) -> Self {
    self.max = max;
    self
  }

  pub fn flex(mut self, flex: usize) -> Self {
    self.flex = flex;
    self
  }

  pub fn fit(mut self, fit: FlexFit) -> Self {
    self.fit = fit;
    self
  }
}

impl Default for LayoutSize {
  fn default() -> Self {
    Self {
      min: Default::default(),
      max: Default::default(),
      flex: 0,
      fit: FlexFit::Loose,
    }
  }
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

pub enum Capability {
  Selectable,
  Custom(String),
}

pub trait BoxWidget: Widget + 'static {
  fn box_widget(self) -> Box<dyn Widget>;
}

impl<W> BoxWidget for W
where
  W: Widget + 'static,
{
  fn box_widget(self) -> Box<dyn Widget> {
    Box::new(self) as Box<dyn Widget>
  }
}

pub trait Widget {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult;
  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult;
  fn render(&self, ctx: &RenderCtx) -> RenderResult;
  fn has_capability(&self, capability: &Capability) -> bool;
}

// TODO: Default impl of Widgets
// impl Widget for Fn {}

impl Widget for &str {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    todo!()
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    let min = Size2D::new(1, 1);
    let mut max = Size2D::new(self.chars().count(), 1);
    // clamp max size to parent size
    // max.width = std::cmp::min(max.width, avail_size.width);
    // max.height = std::cmp::min(max.height, avail_size.height);
    // check for minimum space in parent size
    if avail_size.contains(min.clone()) {
      Ok(LayoutSize::min_max(min, max))
    } else {
      Err(LayoutError::InsufficientSpace)
    }
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let frame = ctx.get_frame().size.clone();
    if frame.width < self.chars().count() {
      let buf = self
        .chars()
        .take(frame.width.checked_sub(1).unwrap_or(0))
        .collect::<String>();
      ctx.renderer().write(buf.as_str());
      ctx.renderer().write("…");
    } else {
      ctx.renderer().write(self);
    }
    Ok(())
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    false
  }
}

impl Widget for String {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    todo!()
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    let min = Size2D::new(1, 1);
    let mut max = Size2D::new(self.chars().count(), 1);
    // clamp max size to parent size
    // max.width = std::cmp::min(max.width, avail_size.width);
    // max.height = std::cmp::min(max.height, avail_size.height);
    // check for minimum space in parent size
    if avail_size.contains(min.clone()) {
      Ok(LayoutSize::min_max(min, max))
    } else {
      Err(LayoutError::InsufficientSpace)
    }
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let frame = ctx.get_frame().size.clone();
    if frame.width < self.chars().count() {
      let buf = self
        .chars()
        .take(frame.width.checked_sub(1).unwrap_or(0))
        .collect::<String>();
      ctx.renderer().write(buf.as_str());
      ctx.renderer().write("…");
    } else {
      ctx.renderer().write(self);
    }
    Ok(())
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    false
  }
}

impl Widget for char {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    todo!()
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    let size = Size2D::new(1, 1);
    // check for minimum space in parent size
    if avail_size.contains(size.clone()) {
      Ok(LayoutSize::min_max(size.clone(), size))
    } else {
      Err(LayoutError::InsufficientSpace)
    }
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    ctx.renderer().write(self.to_string().as_str());
    Ok(())
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    false
  }
}

impl Widget for u32 {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    todo!()
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    let value = format!("{}", self);
    let min = Size2D::new(1, 1);
    let mut max = Size2D::new(value.len(), 1);
    // clamp max size to parent size
    max.width = std::cmp::min(max.width, avail_size.width);
    max.height = std::cmp::min(max.height, avail_size.height);
    // check for minimum space in parent size
    if avail_size.contains(min.clone()) {
      Ok(LayoutSize::min_max(min, max))
    } else {
      Err(LayoutError::InsufficientSpace)
    }
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let val = format!("{}", self);
    let avail_size = ctx.get_frame().size.clone();
    if avail_size.width < val.len() {
      let buf = val.split_at(avail_size.width.checked_sub(1).unwrap_or(0)).0;
      ctx.renderer().write(buf);
      ctx.renderer().write("…");
    } else {
      ctx.renderer().write(val.as_str());
    }
    Ok(())
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    false
  }
}

impl Widget for usize {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    todo!()
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    let value = format!("{}", self);
    let min = Size2D::new(1, 1);
    let mut max = Size2D::new(value.len(), 1);
    // clamp max size to parent size
    max.width = std::cmp::min(max.width, avail_size.width);
    max.height = std::cmp::min(max.height, avail_size.height);
    // check for minimum space in parent size
    if avail_size.contains(min.clone()) {
      Ok(LayoutSize::min_max(min, max))
    } else {
      Err(LayoutError::InsufficientSpace)
    }
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let val = format!("{}", self);
    let avail_size = ctx.get_frame().size.clone();
    if avail_size.width < val.len() {
      let buf = val.split_at(avail_size.width.checked_sub(1).unwrap_or(0)).0;
      ctx.renderer().write(buf);
      ctx.renderer().write("…");
    } else {
      ctx.renderer().write(val.as_str());
    }
    Ok(())
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    false
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

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    self.deref().layout(avail_size)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    self.deref().render(ctx)
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    self.deref().has_capability(capability)
  }
}

impl<T> Widget for RefCell<T>
where
  T: ?Sized + Widget,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    self.get_mut().event(event, size)
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    self.deref().layout(avail_size)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    self.deref().render(ctx)
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    self.deref().has_capability(capability)
  }
}

impl<T> Widget for Box<T>
where
  T: ?Sized + Widget,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    self.deref_mut().event(event, size)
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    self.deref().layout(avail_size)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    self.deref().render(ctx);
    Ok(())
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    self.deref().has_capability(capability)
  }
}

impl Widget for () {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    EventResult::Unhandled
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    debug!("layout() : avail_size: {:?}", avail_size);
    let layout = LayoutSize::min_max(Default::default(), Default::default());
    debug!("layout() : layout: {:?}", layout);
    Ok(layout)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    debug!("render() : frame: {:?}, ", &ctx.get_frame());
    Ok(())
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    false
  }
}

impl<D> Widget for StyledContent<D>
where
  D: Widget + std::fmt::Display,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    EventResult::Unhandled
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    self.content().layout(avail_size)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    if !self.style().attributes.is_empty() {
      ctx.renderer().add_attributes(self.style().attributes);
    }
    if let Some(bg) = self.style().background_color.as_ref() {
      ctx.renderer().set_background(bg);
    }
    if let Some(fg) = self.style().foreground_color.as_ref() {
      ctx.renderer().set_foreground(fg);
    }
    self.content().render(ctx)
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    self.content().has_capability(capability)
  }
}
