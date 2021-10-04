use crate::render::RenderCtx;
use crate::widgets::{
  AnyEvent, Capability, EventResult, LayoutError, LayoutResult, LayoutSize, RenderError, RenderResult, Widget,
};
use crate::FlexFit;
use euclid::default::{Point2D, Rect, SideOffsets2D, Size2D};

// TODO: offsets of (min + max + flex)
pub struct Padding<Child> {
  pub offsets: SideOffsets2D<usize>,
  pub child: Child,
}

impl Default for Padding<()> {
  fn default() -> Self {
    Self {
      offsets: SideOffsets2D::zero(),
      child: (),
    }
  }
}

impl Padding<()> {
  pub fn all(all: usize) -> Self {
    Self {
      offsets: SideOffsets2D::new_all_same(all),
      child: (),
    }
  }

  pub fn child<Child: Widget>(mut self, child: Child) -> Padding<Child> {
    Padding {
      child,
      offsets: self.offsets,
    }
  }
}

impl<Child> Padding<Child> {
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

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutSize {
    let frame = Rect::from_size(avail_size.clone()).inner_rect(self.offsets.clone());
    let mut layout = self.child.layout(avail_size);
    layout.min.width += self.offsets.left + self.offsets.right;
    layout.max.width += self.offsets.left + self.offsets.right;
    layout.min.height += self.offsets.top + self.offsets.bottom;
    layout.max.height += self.offsets.top + self.offsets.bottom;
    layout
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let frame = ctx.get_frame();
    if frame.width() < (self.offsets.left + self.offsets.right)
      || frame.height() < (self.offsets.top + self.offsets.bottom)
    {
      return Err(RenderError::Layout(LayoutError::InsufficientSpace));
    }
    let child_frame = ctx.get_frame().inner_rect(self.offsets.clone());
    ctx.render_child_widget(child_frame, &self.child)
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    self.child.has_capability(capability)
  }
}
