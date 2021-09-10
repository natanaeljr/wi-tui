use euclid::default::{Rect, Size2D};

use crate::debug;
use crate::render::RenderCtx;
use crate::widgets::flexible::FlexFit;
use crate::widgets::{AnyEvent, Capability, EventResult, LayoutResult, RenderResult, Widget};

// TODO: LeakHorizontal
// TODO: LeakVertical

// TODO: percentage
pub struct Leak<Child> {
  pub child: Child,
}

impl<Child> Leak<Child> {
  pub fn child(child: Child) -> Self {
    Self { child }
  }
}

impl<Child> Widget for Leak<Child>
where
  Child: Widget,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    todo!()
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    debug!("layout() : avail_size: {:?}", avail_size);
    let mut layout = self.child.layout(&Size2D::new(1000, 200)).unwrap();
    layout.min.width = std::cmp::min(layout.min.width, 1);
    layout.min.height = std::cmp::min(layout.min.height, 1);
    debug!("layout() : layout: {:?}", layout);
    Ok(layout)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let frame = ctx.get_frame().clone();
    debug!("render() : frame: {:?}, ", &frame);
    let mut layout = self.child.layout(&Size2D::new(1000, 200)).unwrap();
    layout.min.width = std::cmp::max(layout.min.width, frame.size.width);
    layout.min.height = std::cmp::max(layout.min.height, frame.size.height);
    ctx.render_child_widget(Rect::new(frame.origin.clone(), layout.min.clone()), &self.child)?;
    Ok(())
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    self.child.has_capability(capability)
  }
}
