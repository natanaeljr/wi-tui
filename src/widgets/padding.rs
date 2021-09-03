use crate::render::RenderCtx;
use crate::widgets::{AnyEvent, EventResult, LayoutError, LayoutResult, LayoutSize, RenderResult, Widget};
use euclid::default::{Point2D, Rect, SideOffsets2D, Size2D};

pub struct Padding<Child> {
  pub child: Child,
  pub offsets: SideOffsets2D<usize>,
}

impl<Child> Padding<Child> {
  pub fn child(child: Child) -> Self {
    Self {
      child,
      offsets: SideOffsets2D::zero(),
    }
  }

  pub fn all(mut self, all: usize) -> Self {
    self.offsets.top = all;
    self.offsets.left = all;
    self.offsets.right = all;
    self.offsets.bottom = all;
    self
  }

  pub fn top(mut self, top: usize) -> Self {
    self.offsets.top = top;
    self
  }

  pub fn left(mut self, left: usize) -> Self {
    self.offsets.left = left;
    self
  }

  pub fn right(mut self, right: usize) -> Self {
    self.offsets.right = right;
    self
  }

  pub fn bottom(mut self, bottom: usize) -> Self {
    self.offsets.bottom = bottom;
    self
  }
}

impl<Child> Widget for Padding<Child>
where
  Child: Widget,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    self.child.event(event, size)
  }

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    if parent_size.width < self.offsets.left + self.offsets.right
      || parent_size.height < self.offsets.top + self.offsets.bottom
    {
      return Err(LayoutError::InsufficientSpace);
    }

    let frame = Rect::from_size(parent_size.clone()).inner_rect(self.offsets.clone());
    let mut layout = self.child.layout(&frame.size)?;
    layout.min.width += self.offsets.left + self.offsets.right;
    layout.max.width += self.offsets.left + self.offsets.right;
    layout.min.height += self.offsets.top + self.offsets.bottom;
    layout.max.height += self.offsets.top + self.offsets.bottom;
    Ok(layout)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let child_frame = ctx.get_frame().inner_rect(self.offsets.clone());
    ctx.render_child(child_frame, &self.child)
  }
}
