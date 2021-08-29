use euclid::default::Size2D;
use euclid::SideOffsets2D;

use crate::render::RenderCtx;
use crate::widgets::{AnyEvent, LayoutResult, RenderError, RenderResult, Widget};

pub struct Repeat<Child> {
  pub child: Child,
}

impl<Child> Repeat<Child> {
  pub fn new(child: Child) -> Self {
    Self { child }
  }
}

impl<Child> Widget for Repeat<Child>
where
  Child: Widget,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) {
    self.child.event(event, size)
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    let mut layout = self.child.layout(parent_size)?;
    layout.max.width = layout.max.width.max(parent_size.width);
    layout.max.height = layout.max.height.max(parent_size.height);
    Ok(layout)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let frame = ctx.get_frame().clone();
    let layout = self.child.layout(&frame.size).map_err(|e| RenderError::Layout(e))?;
    let width = layout.max.width.min(frame.size.width);
    let height = layout.max.height.min(frame.size.height);
    let size = Size2D::new(width, height);
    for row in 0..frame.size.height / size.height {
      for col in 0..frame.size.width / size.width {
        let top = row * size.height;
        let right = frame.size.width - (col + 1) * size.width;
        let bottom = frame.size.height - (row + 1) * size.height;
        let left = col * size.width;
        let offsets = SideOffsets2D::new(top, right, bottom, left);
        let child_frame = frame.inner_rect(offsets);
        ctx.render_child(child_frame, &self.child)?;
      }
    }
    Ok(())
  }
}
