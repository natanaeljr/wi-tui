use crossterm::event::Event;
use euclid::default::{Rect, Size2D};
use euclid::SideOffsets2D;

use crate::render::{RenderCtx, Renderer};
use crate::widgets::fillchar::FillChar;
use crate::widgets::repeat::Repeat;
use crate::widgets::{
  AnyEvent, Capability, EventResult, LayoutError, LayoutResult, LayoutSize, RenderError, RenderResult, Widget,
};
use crate::Style;

use crate::log::debug;
use crate::widgets::stack::Stack;
use crate::FlexFit;

pub struct Borders<Border, Child> {
  // sides
  pub top: Option<Border>,
  pub left: Option<Border>,
  pub right: Option<Border>,
  pub bottom: Option<Border>,
  // corners
  pub top_left: Option<Border>,
  pub top_right: Option<Border>,
  pub bottom_left: Option<Border>,
  pub bottom_right: Option<Border>,
  // child
  pub child: Child,
}

impl Borders<Box<dyn Widget>, ()> {
  pub fn new() -> Self {
    Self {
      top: None,
      left: None,
      right: None,
      bottom: None,
      top_left: None,
      top_right: None,
      bottom_left: None,
      bottom_right: None,
      child: (),
    }
  }
}

impl<Border, Child> Borders<Border, Child>
where
  Border: Widget,
  Child: Widget,
{
  pub fn with_child(child: Child) -> Self {
    Self {
      top: None,
      left: None,
      right: None,
      bottom: None,
      top_left: None,
      top_right: None,
      bottom_left: None,
      bottom_right: None,
      child,
    }
  }

  pub fn child<Child2: Widget>(mut self, child: Child2) -> Borders<Border, Child2> {
    Borders {
      top: self.top,
      left: self.left,
      right: self.right,
      bottom: self.bottom,
      top_left: self.top_left,
      top_right: self.top_right,
      bottom_left: self.bottom_left,
      bottom_right: self.bottom_right,
      child,
    }
  }

  pub fn top(mut self, border: Border) -> Self {
    self.top = Some(border);
    self
  }

  pub fn left(mut self, border: Border) -> Self {
    self.left = Some(border);
    self
  }

  pub fn right(mut self, border: Border) -> Self {
    self.right = Some(border);
    self
  }

  pub fn bottom(mut self, border: Border) -> Self {
    self.bottom = Some(border);
    self
  }

  pub fn top_left(mut self, border: Border) -> Self {
    self.top_left = Some(border);
    self
  }

  pub fn top_right(mut self, border: Border) -> Self {
    self.top_right = Some(border);
    self
  }

  pub fn bottom_left(mut self, border: Border) -> Self {
    self.bottom_left = Some(border);
    self
  }

  pub fn bottom_right(mut self, border: Border) -> Self {
    self.bottom_right = Some(border);
    self
  }
}

impl<Child> Borders<Box<dyn Widget>, Child>
where
  Child: Widget,
{
  pub fn borders_line(mut self, style: Style) -> Self {
    let this = self
      .top(Box::new(style.clone().child(FillChar::new('─'))) as Box<dyn Widget>)
      .left(Box::new(style.clone().child(FillChar::new('│'))) as Box<dyn Widget>)
      .right(Box::new(style.clone().child(FillChar::new('│'))) as Box<dyn Widget>)
      .bottom(Box::new(style.clone().child(FillChar::new('─'))) as Box<dyn Widget>)
      .top_left(Box::new(style.clone().child(FillChar::new('┌'))) as Box<dyn Widget>)
      .top_right(Box::new(style.clone().child(FillChar::new('┐'))) as Box<dyn Widget>)
      .bottom_left(Box::new(style.clone().child(FillChar::new('└'))) as Box<dyn Widget>)
      .bottom_right(Box::new(style.clone().child(FillChar::new('┘'))) as Box<dyn Widget>);
    this
  }

  pub fn borders_double(mut self, style: Style) -> Self {
    let this = self
      .top(Box::new(style.clone().child(FillChar::new('═'))) as Box<dyn Widget>)
      .left(Box::new(style.clone().child(FillChar::new('║'))) as Box<dyn Widget>)
      .right(Box::new(style.clone().child(FillChar::new('║'))) as Box<dyn Widget>)
      .bottom(Box::new(style.clone().child(FillChar::new('═'))) as Box<dyn Widget>)
      .top_left(Box::new(style.clone().child(FillChar::new('╔'))) as Box<dyn Widget>)
      .top_right(Box::new(style.clone().child(FillChar::new('╗'))) as Box<dyn Widget>)
      .bottom_left(Box::new(style.clone().child(FillChar::new('╚'))) as Box<dyn Widget>)
      .bottom_right(Box::new(style.clone().child(FillChar::new('╝'))) as Box<dyn Widget>);
    this
  }

  pub fn borders_cross(mut self, style: Style) -> Self {
    let this = self
      .top(Box::new(style.clone().child(FillChar::new('-'))) as Box<dyn Widget>)
      .left(Box::new(style.clone().child(FillChar::new('|'))) as Box<dyn Widget>)
      .right(Box::new(style.clone().child(FillChar::new('|'))) as Box<dyn Widget>)
      .bottom(Box::new(style.clone().child(FillChar::new('-'))) as Box<dyn Widget>)
      .top_left(Box::new(style.clone().child(FillChar::new('+'))) as Box<dyn Widget>)
      .top_right(Box::new(style.clone().child(FillChar::new('+'))) as Box<dyn Widget>)
      .bottom_left(Box::new(style.clone().child(FillChar::new('+'))) as Box<dyn Widget>)
      .bottom_right(Box::new(style.clone().child(FillChar::new('+'))) as Box<dyn Widget>);
    this
  }

  pub fn borders_dash(mut self, style: Style) -> Self {
    let this = self
      .top(Box::new(style.clone().child(FillChar::new('-'))) as Box<dyn Widget>)
      .left(Box::new(style.clone().child(FillChar::new('|'))) as Box<dyn Widget>)
      .right(Box::new(style.clone().child(FillChar::new('|'))) as Box<dyn Widget>)
      .bottom(Box::new(style.clone().child(FillChar::new('-'))) as Box<dyn Widget>)
      .top_left(Box::new(style.clone().child(FillChar::new('┌'))) as Box<dyn Widget>)
      .top_right(Box::new(style.clone().child(FillChar::new('┐'))) as Box<dyn Widget>)
      .bottom_left(Box::new(style.clone().child(FillChar::new('└'))) as Box<dyn Widget>)
      .bottom_right(Box::new(style.clone().child(FillChar::new('┘'))) as Box<dyn Widget>);
    this
  }

  pub fn borders_rounded(mut self, style: Style) -> Self {
    let this = self
      .top(Box::new(style.clone().child(FillChar::new('─'))) as Box<dyn Widget>)
      .left(Box::new(style.clone().child(FillChar::new('│'))) as Box<dyn Widget>)
      .right(Box::new(style.clone().child(FillChar::new('│'))) as Box<dyn Widget>)
      .bottom(Box::new(style.clone().child(FillChar::new('─'))) as Box<dyn Widget>)
      .top_left(Box::new(style.clone().child(FillChar::new('╭'))) as Box<dyn Widget>)
      .top_right(Box::new(style.clone().child(FillChar::new('╮'))) as Box<dyn Widget>)
      .bottom_left(Box::new(style.clone().child(FillChar::new('╰'))) as Box<dyn Widget>)
      .bottom_right(Box::new(style.clone().child(FillChar::new('╯'))) as Box<dyn Widget>);
    this
  }

  pub fn top_overlay<B: 'static + Widget>(mut self, border: B) -> Self {
    self.top = Some(match self.top {
      None => Box::new(Stack::new().child(border)),
      Some(prev_top) => Box::new(Stack::new().child(prev_top).child(border)),
    } as Box<dyn Widget>);
    self
  }
}

impl<Border, Child> Widget for Borders<Border, Child>
where
  Border: Widget,
  Child: Widget,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    self.child.event(event, size)
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutSize {
    debug!("layout() : avail_size: {:?}", avail_size);
    let borders_width = if self.left.is_some() { 1 } else { 0 } + if self.right.is_some() { 1 } else { 0 };
    let borders_height = if self.top.is_some() { 1 } else { 0 } + if self.bottom.is_some() { 1 } else { 0 };

    let mut size = avail_size.clone();
    let frame = Rect::from_size(size.clone());

    let top_offset = if self.top.is_some() { 1 } else { 0 };
    let left_offset = if self.left.is_some() { 1 } else { 0 };
    let right_offset = if self.right.is_some() { 1 } else { 0 };
    let bottom_offset = if self.bottom.is_some() { 1 } else { 0 };

    // if let Some(top) = self.top.as_ref() {
    //   let border_frame = frame.inner_rect(SideOffsets2D::new(0, right_offset, frame.height() - 1, left_offset));
    //   top.layout()?;
    // }
    // if let Some(left) = self.left.as_ref() {
    //   let border_frame = frame.inner_rect(SideOffsets2D::new(top_offset, frame.width() - 1, bottom_offset, 0));
    //   left.layout()?;
    // }
    // if let Some(right) = self.right.as_ref() {
    //   let border_frame = frame.inner_rect(SideOffsets2D::new(top_offset, 0, bottom_offset, frame.width() - 1));
    //   right.layout()?;
    // }
    // if let Some(bottom) = self.bottom.as_ref() {
    //   let border_frame = frame.inner_rect(SideOffsets2D::new(frame.height() - 1, right_offset, 0, left_offset));
    //   bottom.layout()?;
    // }

    size.width -= borders_width;
    size.height -= borders_height;
    let mut layout = self.child.layout(avail_size);
    layout.max.width = layout.max.width.checked_add(borders_width).unwrap_or(std::usize::MAX);
    layout.max.height = layout.max.height.checked_add(borders_height).unwrap_or(std::usize::MAX);
    layout.min.width = layout.min.width.checked_add(borders_width).unwrap_or(std::usize::MAX);
    layout.min.height = layout.min.height.checked_add(borders_height).unwrap_or(std::usize::MAX);

    debug!("layout() : layout: {:?}", layout);
    layout
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let frame = ctx.get_frame().clone();
    debug!("render() : frame: {:?}", &frame);
    let mut layout = self.layout(&frame.size);

    layout.max.width = std::cmp::min(layout.max.width, frame.size.width);
    layout.max.height = std::cmp::min(layout.max.height, frame.size.height);
    let frame = Rect::new(frame.origin, layout.max);

    let top_offset = if self.top.is_some() { 1 } else { 0 };
    let left_offset = if self.left.is_some() { 1 } else { 0 };
    let right_offset = if self.right.is_some() { 1 } else { 0 };
    let bottom_offset = if self.bottom.is_some() { 1 } else { 0 };

    if let Some(top) = self.top.as_ref() {
      let border_frame = frame.inner_rect(SideOffsets2D::new(0, right_offset, frame.height() - 1, left_offset));
      ctx.render_child_widget(border_frame, top)?;
    }
    if let Some(left) = self.left.as_ref() {
      let border_frame = frame.inner_rect(SideOffsets2D::new(top_offset, frame.width() - 1, bottom_offset, 0));
      ctx.render_child_widget(border_frame, left)?;
    }
    if let Some(right) = self.right.as_ref() {
      let border_frame = frame.inner_rect(SideOffsets2D::new(top_offset, 0, bottom_offset, frame.width() - 1));
      ctx.render_child_widget(border_frame, right)?;
    }
    if let Some(bottom) = self.bottom.as_ref() {
      let border_frame = frame.inner_rect(SideOffsets2D::new(frame.height() - 1, right_offset, 0, left_offset));
      ctx.render_child_widget(border_frame, bottom)?;
    }

    if self.top.is_some() && self.left.is_some() {
      if let Some(top_left) = self.top_left.as_ref() {
        let border_frame = frame.inner_rect(SideOffsets2D::new(0, frame.width() - 1, frame.height() - 1, 0));
        ctx.render_child_widget(border_frame, top_left)?;
      }
    }

    if self.top.is_some() && self.right.is_some() {
      if let Some(top_right) = self.top_right.as_ref() {
        let border_frame = frame.inner_rect(SideOffsets2D::new(0, 0, frame.height() - 1, frame.width() - 1));
        ctx.render_child_widget(border_frame, top_right)?;
      }
    }

    if self.bottom.is_some() && self.left.is_some() {
      if let Some(bottom_left) = self.bottom_left.as_ref() {
        let border_frame = frame.inner_rect(SideOffsets2D::new(frame.height() - 1, frame.width() - 1, 0, 0));
        ctx.render_child_widget(border_frame, bottom_left)?;
      }
    }

    if self.bottom.is_some() && self.right.is_some() {
      if let Some(bottom_right) = self.bottom_right.as_ref() {
        let border_frame = frame.inner_rect(SideOffsets2D::new(frame.height() - 1, 0, 0, frame.width() - 1));
        ctx.render_child_widget(border_frame, bottom_right)?;
      }
    }

    let child_frame = frame.inner_rect(SideOffsets2D::new(top_offset, right_offset, bottom_offset, left_offset));
    ctx.render_child_widget(child_frame, &self.child)
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    self.child.has_capability(capability)
  }
}
