use crate::render::RenderCtx;
use crate::widgets::{AnyEvent, EventResult, LayoutResult, RenderResult, Widget};
use euclid::default::Size2D;

pub struct Flexible<Child> {
  pub flex: usize,
  pub child: Child,
}

impl Flexible<()> {
  pub fn flex(flex: usize) -> Self {
    Self { flex, child: () }
  }

  pub fn child<Child2: Widget>(mut self, child: Child2) -> Flexible<Child2> {
    Flexible { flex: self.flex, child }
  }
}

impl<Child> Flexible<Child> {
  pub fn new(flex: usize, child: Child) -> Self {
    Self { flex, child }
  }
}

impl<Child> Widget for Flexible<Child>
where
  Child: Widget,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    self.child.event(event, size)
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    self.child.layout(avail_size)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    ctx.render_child_widget(ctx.get_frame().clone(), &self.child)
  }

  fn flex(&self) -> usize {
    self.flex
  }
}
