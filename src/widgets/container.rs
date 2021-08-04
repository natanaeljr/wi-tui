use crate::render::RenderCtx;
use crate::widgets::Widget;

pub struct Container {
  split: Split,
  children: Vec<Box<dyn Widget>>,
}

enum Split {
  Horizontal,
  Vertical,
}

impl Container {
  pub fn horizontal() -> Self {
    Self {
      split: Split::Horizontal,
      children: Vec::default(),
    }
  }
  pub fn vertical() -> Self {
    Self {
      split: Split::Vertical,
      children: Vec::default(),
    }
  }
  pub fn child<W: Widget + 'static>(mut self, child: W) -> Self {
    self.children.push(Box::new(child));
    self
  }
}

impl Widget for Container {
  fn event(&mut self) {
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&mut self) {
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
