use crate::render::RenderCtx;
use crate::widgets::flexible::FlexFit;
use crate::widgets::{AnyEvent, EventResult, LayoutError, LayoutResult, LayoutSize, RenderError, RenderResult, Widget};
use euclid::default::{Rect, Size2D};

// TODO: Minimize should actually be a widget that forces the Render to the min layout
// like the property of FlexFit::tight.
// A new widget should be created for this current Minimize behaviour. (MinMax()?)
// TODO: wait, maybe we should do a Shrink instead? (opposite of Expand?)

// TODO: turn this Min into MinMax (or Sized) with Option<> for each

pub struct Min<Child> {
  min: Size2D<usize>,
  child: Child,
}

impl Min<()> {
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

impl<Child> Min<Child> {
  pub fn child<C: Widget>(self, child: C) -> Min<C> {
    Min { min: self.min, child }
  }
}

impl<Child> Widget for Min<Child>
where
  Child: Widget,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    todo!()
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    if !avail_size.contains(self.min.clone()) {
      return Err(LayoutError::InsufficientSpace);
    }
    let layout_result = self.child.layout(avail_size);
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
        // let mut layout = self.child.layout(&Size2D::new(1000, 200)).unwrap();
        // layout.min.width = std::cmp::max(layout.min.width, frame.size.width);
        // layout.min.height = std::cmp::max(layout.min.height, frame.size.height);
        // ctx.render_child_widget(Rect::new(frame.origin.clone(), layout.min.clone()), &self.child)?;
      } else {
        return Err(RenderError::Layout(e));
      }
    } else {
      ctx.render_child_widget(frame.clone(), &self.child)?;
    }

    Ok(())
  }

  fn flex(&self) -> (usize, FlexFit) {
    self.child.flex()
  }
}
