use crate::render::RenderCtx;
use crate::widgets::{AnyEvent, LayoutResult, RenderResult, Widget};
use euclid::default::Size2D;
use std::iter::FromIterator;

pub struct VerticalContainer {
  children: Vec<Box<dyn Widget>>,
}

impl VerticalContainer {
  pub fn new() -> Self {
    Self {
      children: Vec::default(),
    }
  }
  pub fn children<I, W>(container: I) -> Self
  where
    I: IntoIterator<Item = W>,
    W: Widget + 'static,
  {
    Self {
      children: container
        .into_iter()
        .map(|c| Box::new(c) as Box<dyn Widget>)
        .collect::<Vec<_>>(),
    }
  }
  pub fn child<W: Widget + 'static>(mut self, child: W) -> Self {
    self.children.push(Box::new(child));
    self
  }
}

impl Widget for VerticalContainer {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) {
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    todo!()
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let len = self.children.len();
    for (idx, child) in self.children.iter().enumerate() {
      child.render(ctx);
      if idx < len - 1 {
        ctx.renderer().next_line();
      }
    }
    Ok(())
  }
}
