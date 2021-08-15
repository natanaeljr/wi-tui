use crate::render::RenderCtx;
use crate::widgets::{LayoutResult, RenderResult, Widget};
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
  fn event(&mut self) {
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    todo!()
  }

  fn render(&self, ctx: &mut RenderCtx) -> RenderResult {
    let mut child_ctx = ctx.child_ctx(ctx.get_frame().inner_rect(self.offsets.clone()));
    self.child.render(&mut child_ctx)?;
    ctx.set_frame(Rect::from_size(ctx.get_frame_size().clone()));
    Ok(())
  }
}
