use std::ops::Deref;

use euclid::default::{Point2D, Rect, Size2D};

use crate::render::RenderCtx;
use crate::util::Scoped;
use crate::widgets::{AnyEvent, EventResult, LayoutError, LayoutResult, LayoutSize, RenderError, RenderResult, Widget};

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

// TODO: Horizontal/Vertical
// TODO: Flex calculation
// TODO: Must fit N children

pub struct Container<ChildrenStorage> {
  pub children: Option<ChildrenStorage>,
  pub must_fit_all_children: bool,
}

impl<ChildrenStorage> Container<ChildrenStorage>
where
  ChildrenStorage: Children,
{
  pub fn new() -> Self {
    Self {
      children: None,
      must_fit_all_children: false,
    }
  }

  pub fn children(mut self, children: ChildrenStorage) -> Self {
    self.children = Some(children);
    self
  }

  pub fn must_fit_all_children(mut self, must_fit_all_children: bool) -> Self {
    self.must_fit_all_children = must_fit_all_children;
    self
  }
}

impl Container<Vec<Box<dyn Widget>>> {
  pub fn child<C: Widget + 'static>(mut self, child: C) -> Self {
    if let Some(children) = &mut self.children {
      children.push(Box::new(child));
    } else {
      self.children = Some(vec![Box::new(child) as Box<dyn Widget>])
    }
    self
  }
}

impl<ChildrenStorage> Widget for Container<ChildrenStorage>
where
  ChildrenStorage: Children,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    todo!()
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    let children = self.children.as_ref().unwrap();

    let mut layout = LayoutSize::default();
    let mut avail_size = avail_size.clone();

    for idx in 0..children.len() {
      let child = children.get_child(idx).unwrap();
      let child_layout_result = child.layout(&avail_size);
      if let Err(LayoutError::InsufficientSpace) = child_layout_result {
        if self.must_fit_all_children {
          return Err(LayoutError::InsufficientSpace);
        } else {
          break;
        }
      }
      let child_layout = child_layout_result.unwrap();
      let child_size = Size2D::new(
        std::cmp::min(avail_size.width, child_layout.max.width),
        std::cmp::min(avail_size.height, child_layout.max.height),
      );

      avail_size.width -= child_size.width;

      layout.min.width = layout
        .min
        .width
        .checked_add(child_size.width)
        .unwrap_or(std::usize::MAX);
      layout.max.width = layout
        .max
        .width
        .checked_add(child_layout.max.width)
        .unwrap_or(std::usize::MAX);

      layout.min.height = layout.min.height.max(child_size.height);
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
      let child_layout_result = child.layout(&avail_size);
      if let Err(LayoutError::InsufficientSpace) = child_layout_result {
        if self.must_fit_all_children {
          return Err(RenderError::Layout(LayoutError::InsufficientSpace));
        } else {
          break;
        }
      }
      let child_layout = child_layout_result.unwrap();
      let child_size = Size2D::new(
        std::cmp::min(avail_size.width, child_layout.max.width),
        std::cmp::min(avail_size.height, child_layout.max.height),
      );
      let child_frame = Rect::new(Point2D::new(frame.min_x() + the_x, frame.min_y()), child_size);
      ctx.render_child_dyn_widget(child_frame, child.deref())?;
      avail_size.width -= child_size.width;
      the_x += child_size.width;
    }

    Ok(())
  }
}
