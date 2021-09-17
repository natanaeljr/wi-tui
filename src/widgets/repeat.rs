use crate::debug;
use euclid::default::{Point2D, Rect, Size2D};
use euclid::SideOffsets2D;

use crate::render::RenderCtx;
use crate::FlexFit;
use crate::widgets::{AnyEvent, Capability, EventResult, LayoutError, LayoutResult, RenderError, RenderResult, Widget};

// TODO: only vertical
// TODO: only horizontal
// TODO: repeat count
pub struct Repeat<Child> {
  pub child: Child,
}

impl<Child> Repeat<Child> {
  pub fn child(child: Child) -> Self {
    Self { child }
  }
}

impl<Child> Widget for Repeat<Child>
where
  Child: Widget,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    self.child.event(event, size)
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    let mut layout = self.child.layout(avail_size)?;
    layout.max.width = layout.max.width.max(avail_size.width);
    layout.max.height = layout.max.height.max(avail_size.height);
    Ok(layout)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let frame = ctx.get_frame().clone();
    // let layout = self.child.layout(&frame.size).map_err(|e| RenderError::Layout(e))?;
    // let width = layout.max.width.min(frame.size.width);
    // let height = layout.max.height.min(frame.size.height);
    // let size = Size2D::new(width, height);
    // for row in 0..frame.size.height / size.height {
    //   for col in 0..frame.size.width / size.width {
    //     let top = row * size.height;
    //     let right = frame.size.width - (col + 1) * size.width;
    //     let bottom = frame.size.height - (row + 1) * size.height;
    //     let left = col * size.width;
    //     let offsets = SideOffsets2D::new(top, right, bottom, left);
    //     let child_frame = frame.inner_rect(offsets);
    //     ctx.render_child(child_frame, &self.child)?;
    //   }
    // }

    let layout = self.child.layout(&frame.size).map_err(|e| RenderError::Layout(e))?;
    let child_width = layout.max.width.min(frame.size.width);
    let child_height = layout.max.height.min(frame.size.height);
    let child_size = Size2D::new(child_width, child_height);

    if child_size.is_empty() {
      return Ok(());
    }

    let mut avail_height = frame.size.height;
    'row_loop: while avail_height > 0 {
      let mut avail_width = frame.size.width;
      let mut size = child_size.clone();
      'col_loop: while avail_width > 0 {
        if avail_height < child_height || avail_width < child_width {
          let layout_result = self.child.layout(&Size2D::new(avail_width, avail_height));
          if let Err(e) = layout_result {
            if let LayoutError::InsufficientSpace = e {
              break 'col_loop;
            }
            return Err(RenderError::Layout(e));
          }
          let child_width = layout.max.width.min(frame.size.width);
          let child_height = layout.max.height.min(frame.size.height);
          size = Size2D::new(child_width, child_height);
        }
        let x = frame.max_x() - avail_width;
        let y = frame.max_y() - avail_height;
        let child_frame = Rect::new(Point2D::new(x, y), size.clone());
        ctx.render_child_widget(child_frame, &self.child)?;
        avail_width = avail_width.checked_sub(size.width).unwrap_or(0);
      }
      avail_height = avail_height.checked_sub(size.height).unwrap_or(0);
    }

    Ok(())
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    self.child.has_capability(capability)
  }
}
