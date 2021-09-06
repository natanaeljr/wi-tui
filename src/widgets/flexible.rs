use crate::render::RenderCtx;
use crate::widgets::{AnyEvent, EventResult, LayoutResult, RenderError, RenderResult, Widget};
use euclid::default::Size2D;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum FlexFit {
  Tight,
  Loose,
}

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

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    self.child.layout(avail_size)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    ctx.render_child_widget(ctx.get_frame().clone(), &self.child)
  }

  fn flex(&self) -> (usize, FlexFit) {
    (self.flex, self.fit.clone())
  }
}
