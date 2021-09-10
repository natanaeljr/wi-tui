use crate::render::RenderCtx;
use crate::widgets::flexible::FlexFit;
use crate::widgets::{AnyEvent, EventResult, LayoutResult, LayoutSize, RenderResult, Widget};
use euclid::default::Size2D;

pub struct Expand<Child> {
  child: Child,
}

impl<Child> Expand<Child> {
  pub fn child(child: Child) -> Self {
    Self { child }
  }
}

impl<Child> Widget for Expand<Child>
where
  Child: Widget,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    self.child.event(event, size)
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    let layout = self.child.layout(avail_size)?;
    Ok(layout.max(avail_size.clone()).flex(1).fit(FlexFit::Loose))
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    self.child.render(ctx)
  }
}
