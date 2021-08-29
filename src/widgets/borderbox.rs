use euclid::default::Size2D;
use euclid::SideOffsets2D;

use crate::render::{RenderCtx, Renderer};
use crate::widgets::repeat::Repeat;
use crate::widgets::style::Style;
use crate::widgets::{AnyEvent, LayoutError, LayoutResult, RenderError, RenderResult, Widget};
use crossterm::event::Event;

pub struct BorderBox<Border, Child> {
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

impl<Border, Child> BorderBox<Border, Child> {
  pub fn new(child: Child) -> Self {
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

// TODO: Allow Style input in presets
impl<Child> BorderBox<Style<Repeat<char>>, Child> {
  pub fn preset_lined(child: Child) -> BorderBox<Style<Repeat<char>>, Child> {
    let mut this = BorderBox::new(child);
    this
      .top(Style::new(Repeat::new('─')))
      .left(Style::new(Repeat::new('│')))
      .right(Style::new(Repeat::new('│')))
      .bottom(Style::new(Repeat::new('─')))
      .top_left(Style::new(Repeat::new('┌')))
      .top_right(Style::new(Repeat::new('┐')))
      .bottom_left(Style::new(Repeat::new('└')))
      .bottom_right(Style::new(Repeat::new('┘')))
  }

  pub fn preset_double(child: Child) -> BorderBox<Style<Repeat<char>>, Child> {
    let mut this = BorderBox::new(child);
    this
      .top(Style::new(Repeat::new('═')))
      .left(Style::new(Repeat::new('║')))
      .right(Style::new(Repeat::new('║')))
      .bottom(Style::new(Repeat::new('═')))
      .top_left(Style::new(Repeat::new('╔')))
      .top_right(Style::new(Repeat::new('╗')))
      .bottom_left(Style::new(Repeat::new('╚')))
      .bottom_right(Style::new(Repeat::new('╝')))
  }

  pub fn preset_dashed(child: Child) -> BorderBox<Style<Repeat<char>>, Child> {
    let mut this = BorderBox::new(child);
    this
      .top(Style::new(Repeat::new('-')))
      .left(Style::new(Repeat::new('|')))
      .right(Style::new(Repeat::new('|')))
      .bottom(Style::new(Repeat::new('-')))
      .top_left(Style::new(Repeat::new('+')))
      .top_right(Style::new(Repeat::new('+')))
      .bottom_left(Style::new(Repeat::new('+')))
      .bottom_right(Style::new(Repeat::new('+')))
  }

  pub fn preset_simple_dashed(child: Child) -> BorderBox<Style<Repeat<char>>, Child> {
    let mut this = BorderBox::new(child);
    this
      .top(Style::new(Repeat::new('-')))
      .left(Style::new(Repeat::new('|')))
      .right(Style::new(Repeat::new('|')))
      .bottom(Style::new(Repeat::new('-')))
      .top_left(Style::new(Repeat::new('┌')))
      .top_right(Style::new(Repeat::new('┐')))
      .bottom_left(Style::new(Repeat::new('└')))
      .bottom_right(Style::new(Repeat::new('┘')))
  }
}

impl<Border, Child> Widget for BorderBox<Border, Child>
where
  Border: Widget,
  Child: Widget,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) {
    self.child.event(event, size)
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    let borders_width = if self.left.is_some() { 1 } else { 0 } + if self.right.is_some() { 1 } else { 0 };
    let borders_height = if self.top.is_some() { 1 } else { 0 } + if self.bottom.is_some() { 1 } else { 0 };

    let mut frame = parent_size.clone();
    if frame.width < borders_width || frame.height < borders_height {
      return Err(LayoutError::InsufficientSpace);
    }

    frame.width -= borders_width;
    frame.height -= borders_height;

    let mut layout = self.child.layout(&frame)?;
    layout.max.width += borders_width;
    layout.max.height += borders_height;
    layout.min.width += borders_width;
    layout.min.height += borders_height;

    if !frame.contains(layout.min.clone()) {
      return Err(LayoutError::InsufficientSpace);
    }
    Ok(layout)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let borders_width = if self.left.is_some() { 1 } else { 0 } + if self.right.is_some() { 1 } else { 0 };
    let borders_height = if self.top.is_some() { 1 } else { 0 } + if self.bottom.is_some() { 1 } else { 0 };

    let frame = ctx.get_frame().clone();
    if frame.width() < borders_width || frame.height() < borders_height {
      return Err(RenderError::Layout(LayoutError::InsufficientSpace));
    }

    let top_offset = if self.top.is_some() { 1 } else { 0 };
    let left_offset = if self.left.is_some() { 1 } else { 0 };
    let right_offset = if self.right.is_some() { 1 } else { 0 };
    let bottom_offset = if self.bottom.is_some() { 1 } else { 0 };

    if let Some(top) = self.top.as_ref() {
      let border_frame = frame.inner_rect(SideOffsets2D::new(0, right_offset, frame.height() - 1, left_offset));
      ctx.render_child(border_frame, top)?;
    }
    if let Some(left) = self.left.as_ref() {
      let border_frame = frame.inner_rect(SideOffsets2D::new(top_offset, frame.width() - 1, bottom_offset, 0));
      ctx.render_child(border_frame, left)?;
    }
    if let Some(right) = self.right.as_ref() {
      let border_frame = frame.inner_rect(SideOffsets2D::new(top_offset, 0, bottom_offset, frame.width() - 1));
      ctx.render_child(border_frame, right)?;
    }
    if let Some(bottom) = self.bottom.as_ref() {
      let border_frame = frame.inner_rect(SideOffsets2D::new(frame.height() - 1, right_offset, 0, left_offset));
      ctx.render_child(border_frame, bottom)?;
    }

    if self.top.is_some() && self.left.is_some() {
      if let Some(top_left) = self.top_left.as_ref() {
        let border_frame = frame.inner_rect(SideOffsets2D::new(0, frame.width() - 1, frame.height() - 1, 0));
        ctx.render_child(border_frame, top_left)?;
      }
    }

    if self.top.is_some() && self.right.is_some() {
      if let Some(top_right) = self.top_right.as_ref() {
        let border_frame = frame.inner_rect(SideOffsets2D::new(0, 0, frame.height() - 1, frame.width() - 1));
        ctx.render_child(border_frame, top_right)?;
      }
    }

    if self.bottom.is_some() && self.left.is_some() {
      if let Some(bottom_left) = self.bottom_left.as_ref() {
        let border_frame = frame.inner_rect(SideOffsets2D::new(frame.height() - 1, frame.width() - 1, 0, 0));
        ctx.render_child(border_frame, bottom_left)?;
      }
    }

    if self.bottom.is_some() && self.right.is_some() {
      if let Some(bottom_right) = self.bottom_right.as_ref() {
        let border_frame = frame.inner_rect(SideOffsets2D::new(frame.height() - 1, 0, 0, frame.width() - 1));
        ctx.render_child(border_frame, bottom_right)?;
      }
    }

    let child_frame = frame.inner_rect(SideOffsets2D::new(top_offset, right_offset, bottom_offset, left_offset));
    ctx.render_child(child_frame, &self.child)
  }
}
