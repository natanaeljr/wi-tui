use crate::render::RenderCtx;
use crate::widgets::{AnyEvent, EventResult, LayoutError, LayoutResult, LayoutSize, RenderError, RenderResult, Widget};
use euclid::default::{Rect, Size2D};

pub struct Minimize<Child> {
  min: Size2D<usize>,
  child: Child,
}

impl Minimize<()> {
  pub fn zero() -> Self {
    Self {
      min: Size2D::zero(),
      child: (),
    }
  }
  pub fn min(min: Size2D<usize>) -> Self {
    Self { min, child: () }
  }
}

impl<Child> Minimize<Child> {
  pub fn child<C: Widget>(self, child: C) -> Minimize<C> {
    Minimize { min: self.min, child }
  }
}

impl<Child> Widget for Minimize<Child>
where
  Child: Widget,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    todo!()
  }

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    if !parent_size.contains(self.min.clone()) {
      return Err(LayoutError::InsufficientSpace);
    }
    let layout_result = self.child.layout(parent_size);
    if let Err(e) = layout_result {
      if let LayoutError::InsufficientSpace = e {
        return Ok(LayoutSize {
          min: self.min.clone(),
          max: self.min.clone(),
        });
      }
      return Err(e);
    }
    let mut layout = layout_result.unwrap();
    layout.min = self.min.clone();
    layout.max.width = std::cmp::max(layout.max.width, layout.min.width);
    layout.max.height = std::cmp::max(layout.max.height, layout.min.height);
    Ok(layout)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let frame = ctx.get_frame();
    if frame.size.is_empty() {
      return Ok(());
    }
    if !frame.size.contains(self.min.clone()) {
      // info!("render() : LayoutError::InsufficientSpace");
      return Err(RenderError::Layout(LayoutError::InsufficientSpace));
    }

    let layout_result = self.child.layout(&frame.size);
    if let Err(e) = layout_result {
      if let LayoutError::InsufficientSpace = e {
        // Just like Leak::render
        let mut layout = self.child.layout(&Size2D::new(1000, 200)).unwrap();
        layout.min.width = std::cmp::max(layout.min.width, frame.size.width);
        layout.min.height = std::cmp::max(layout.min.height, frame.size.height);
        ctx.render_child(Rect::new(frame.origin.clone(), layout.min.clone()), &self.child)?;
      } else {
        return Err(RenderError::Layout(e));
      }
    } else {
      ctx.render_child(frame.clone(), &self.child)?;
    }

    Ok(())
  }
}
