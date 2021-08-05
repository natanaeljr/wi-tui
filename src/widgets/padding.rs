use crate::render::RenderCtx;
use crate::widgets::{LayoutResult, Widget};
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

  fn layout(&mut self, max_size: &Size2D<usize>) -> LayoutResult {
    todo!()
  }

  fn render(&self, ctx: &mut RenderCtx) -> Option<()> {
    let ext_size = Size2D::new(50, 10);
    let frame = Rect::from_size(ext_size).inner_rect(self.offsets.clone());
    ctx.renderer.set_frame(frame);
    self.child.render(ctx)
  }
}
