use crate::render::RenderCtx;
use crate::widgets::{LayoutResult, Widget};
use euclid::default::{Point2D, Rect, Size2D};

pub struct Padding<Child> {
  pub child: Child,
  pub top: usize,
  pub left: usize,
  pub right: usize,
  pub bottom: usize,
  layout: (usize, usize),
}

impl<Child> Padding<Child> {
  pub fn around(child: Child) -> Self {
    Self {
      child,
      top: 0,
      left: 0,
      right: 0,
      bottom: 0,
      layout: (50, 9),
    }
  }

  pub fn top(mut self, top: usize) -> Self {
    self.top = top;
    self
  }

  pub fn left(mut self, left: usize) -> Self {
    self.left = left;
    self
  }

  pub fn right(mut self, right: usize) -> Self {
    self.right = right;
    self
  }

  pub fn bottom(mut self, bottom: usize) -> Self {
    self.bottom = bottom;
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
    let frame = Rect::new(
      Point2D::new(self.left, self.top),
      Size2D::new(self.layout.0 - self.right, self.layout.1 - self.bottom),
    );
    ctx.renderer.set_frame(frame);
    self.child.render(ctx)
  }
}