use euclid::default::Size2D;

use crate::render::RenderCtx;
use crate::widgets::{AnyEvent, Capability, EventResult, LayoutResult, LayoutSize, RenderError, RenderResult, Widget};
use crate::FlexFit;

pub struct Flexible<Child> {
  pub flex: usize,
  pub fit: FlexFit,
  pub child: Child,
}

impl Flexible<()> {
  pub fn tight(flex: usize) -> Self {
    Self {
      flex,
      fit: FlexFit::Tight,
      child: (),
    }
  }

  pub fn loose(flex: usize) -> Self {
    Self {
      flex,
      fit: FlexFit::Loose,
      child: (),
    }
  }

  pub fn child<Child2: Widget>(mut self, child: Child2) -> Flexible<Child2> {
    Flexible {
      flex: self.flex,
      fit: self.fit,
      child,
    }
  }
}

impl<Child> Widget for Flexible<Child>
where
  Child: Widget,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    self.child.event(event, size)
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutSize {
    self.child.layout(avail_size).flex(self.flex).fit(self.fit.clone())
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    ctx.render_child_widget(ctx.get_frame().clone(), &self.child)
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    self.child.has_capability(capability)
  }
}
