use crate::render::RenderCtx;
use crate::widgets::{AnyEvent, Capability, EventResult, LayoutResult, LayoutSize, RenderResult, Widget};
use crate::FlexFit;
use euclid::default::Size2D;

pub struct Expanded<Child> {
  pub flex: usize,
  pub child: Child,
}

impl<Child> Expanded<Child> {
  pub fn child(child: Child) -> Self {
    Self { flex: 1, child }
  }

  pub fn flex_child(flex: usize, child: Child) -> Self {
    Self { flex, child }
  }
}

impl<Child> Widget for Expanded<Child>
where
  Child: Widget,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    self.child.event(event, size)
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    let layout = self.child.layout(avail_size)?;
    Ok(layout.max(avail_size.clone()).flex(self.flex).fit(FlexFit::Tight))
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    self.child.render(ctx)
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    self.child.has_capability(capability)
  }
}
