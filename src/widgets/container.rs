use crate::render::RenderCtx;
use crate::widgets::{Widget, LayoutResult};
use euclid::default::Size2D;

pub struct VerticalContainer {
  children: Vec<Box<dyn Widget>>,
}

impl VerticalContainer {
  pub fn new() -> Self {
    Self {
      children: Vec::default(),
    }
  }
  pub fn child<W: Widget + 'static>(mut self, child: W) -> Self {
    self.children.push(Box::new(child));
    self
  }
}

impl Widget for VerticalContainer {
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
    let len = self.children.len();
    for (idx, child) in self.children.iter().enumerate() {
      child.render(ctx);
      if idx < len - 1 {
        ctx.renderer.next_line();
      }
    }
    Some(())
  }
}
