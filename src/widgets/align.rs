use crate::render::RenderCtx;
use crate::widgets::Widget;

pub struct Align<Child> {
  child: Child,
  vertical: u32,   /* TODO: enum */
  horizontal: u32, /* TODO: enum */
}

impl<Child> Align<Child>
where
  Child: Widget,
{
  pub fn centered(child: Child) -> Self {
    Self {
      child,
      vertical: 0,
      horizontal: 0,
    }
  }
}

impl<Child> Widget for Align<Child>
where
  Child: Widget,
{
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
    self.child.render(ctx)
  }
}
