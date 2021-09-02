use euclid::default::{Rect, Size2D};

use crate::render::RenderCtx;
use crate::widgets::{AnyEvent, EventResult, LayoutResult, RenderResult, Widget};

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

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    eprintln!("[{}:{}]layout(): parent_size: {:?}", file!(), line!(), parent_size);
    let mut layout = self
      .child
      .layout(&Size2D::new(std::usize::MAX / 2, std::usize::MAX / 2))
      .unwrap();
    layout.min.width = std::cmp::min(layout.min.width, 1);
    layout.min.height = std::cmp::min(layout.min.height, 1);
    eprintln!("[{}:{}]layout(): layout: {:?}", file!(), line!(), layout);
    Ok(layout)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let frame = ctx.get_frame().clone();
    eprintln!("[{}:{}]render(): frame: {:?}, ", file!(), line!(), &frame);
    let mut layout = self
      .child
      .layout(&Size2D::new(10000, 1000))
      .unwrap();
    layout.min.width = std::cmp::max(layout.min.width, frame.size.width);
    layout.min.height = std::cmp::max(layout.min.height, frame.size.height);
    ctx.render_child(Rect::new(frame.origin.clone(), layout.min.clone()), &self.child)?;
    Ok(())
  }
}
