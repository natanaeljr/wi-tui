use crate::render::RenderCtx;
use crate::util::Scoped;
use crate::widgets::{AnyEvent, LayoutError, LayoutResult, LayoutSize, RenderError, RenderResult, Widget};
use euclid::default::{Point2D, Rect, Size2D};
use std::ops::Deref;

pub trait Children: 'static {
  fn len(&self) -> usize;
  fn get_child(&self, index: usize) -> Option<Scoped<dyn Widget>>;
}

impl<W> Children for Vec<W>
where
  W: Widget + 'static,
{
  fn len(&self) -> usize {
    self.len()
  }

  fn get_child(&self, index: usize) -> Option<Scoped<dyn Widget>> {
    self.get(index).and_then(|c| Some(Scoped::Ref(c as &dyn Widget)))
  }
}

pub struct Container {
  pub children: Option<Box<dyn Children>>,
}

impl Container {
  pub fn new() -> Self {
    Self { children: None }
  }

  pub fn children<C: Children + 'static>(mut self, children: C) -> Self {
    self.children = Some(Box::new(children));
    self
  }
}

impl Widget for Container {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) {
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    let children = self.children.as_ref().unwrap();

    let mut layout = LayoutSize::default();
    let mut avail_size = parent_size.clone();

    for idx in 0..children.len() {
      let child = children.get_child(idx).unwrap();
      let child_layout = child.layout(&avail_size)?;

      if !avail_size.contains(child_layout.min.clone()) {
        return Err(LayoutError::InsufficientSpace);
      }
      avail_size.width -= child_layout.min.width;

      layout.min.width = layout
        .min
        .width
        .checked_add(child_layout.min.width)
        .unwrap_or(std::usize::MAX);
      layout.max.width = layout
        .max
        .width
        .checked_add(child_layout.max.width)
        .unwrap_or(std::usize::MAX);

      layout.min.height = layout.min.height.max(child_layout.min.height);
      layout.max.height = layout.max.height.max(child_layout.max.height);
    }

    Ok(layout)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let frame = ctx.get_frame().clone();
    let frame_size = frame.size.clone();
    let layout = self.layout(&frame_size).map_err(|e| RenderError::Layout(e))?;

    let children = self.children.as_ref().unwrap();

    let mut avail_size = frame_size.clone();
    let mut the_x = 0;
    for idx in 0..children.len() {
      let child = children.get_child(idx).unwrap();
      let child_layout = child.layout(&avail_size).unwrap();
      avail_size.width -= child_layout.min.width;
      let child_frame = Rect::new(Point2D::new(frame.origin.x + the_x, frame.origin.y), child_layout.max);
      ctx.render_child_widget(child_frame, child.deref())?;
      the_x += child_layout.max.width;
    }

    Ok(())
  }
}
