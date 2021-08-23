use crate::render::RenderCtx;
use crate::widgets::{LayoutResult, RenderResult, Widget, AnyEvent};
use euclid::default::{Point2D, Rect, SideOffsets2D, Size2D};

pub struct Padding<Child> {
  pub child: Child,
  pub offsets: SideOffsets2D<usize>,
}

impl<Child> Padding<Child> {
  pub fn around(child: Child) -> Self {
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
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) {
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    todo!()
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let child_frame = ctx.get_frame().inner_rect(self.offsets.clone());
    ctx.render_child(child_frame, &self.child)
  }
}
