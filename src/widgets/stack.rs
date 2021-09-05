use crate::render::RenderCtx;
use crate::util::Scoped;
use crate::widgets::{AnyEvent, EventResult, LayoutResult, LayoutSize, RenderError, RenderResult, Widget};
use euclid::default::Size2D;
use std::cmp::max;
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

pub struct Stack<ChildrenStorage> {
  pub children: Option<ChildrenStorage>,
  pub must_fit_all_children: bool,
}

impl<ChildrenStorage> Stack<ChildrenStorage>
where
  ChildrenStorage: Children,
{
  pub fn new() -> Self {
    Self {
      children: None,
      must_fit_all_children: true,
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

impl Stack<Vec<Box<dyn Widget>>> {
  pub fn child<C: Widget + 'static>(mut self, child: C) -> Self {
    if let Some(children) = &mut self.children {
      children.push(Box::new(child));
    } else {
      self.children = Some(vec![Box::new(child) as Box<dyn Widget>])
    }
    self
  }
}

impl<ChildrenStorage> Widget for Stack<ChildrenStorage>
where
  ChildrenStorage: Children,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    todo!()
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    let children = self.children.as_ref().unwrap();
    let mut layout = LayoutSize::default();
    for idx in 0..children.len() {
      let child = children.get_child(idx).unwrap();
      let child_layout = child.layout(&avail_size)?;
      layout.min.height = max(layout.min.height, child_layout.min.height);
      layout.min.width = max(layout.min.width, child_layout.min.width);
      layout.max.height = max(layout.max.height, child_layout.max.height);
      layout.max.width = max(layout.max.width, child_layout.max.width);
    }
    Ok(layout)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let frame = ctx.get_frame().clone();
    let layout = self.layout(&frame.size).map_err(|e| RenderError::Layout(e))?;
    let children = self.children.as_ref().unwrap();
    for idx in 0..children.len() {
      let child = children.get_child(idx).unwrap();
      ctx.render_child_dyn_widget(frame.clone(), child.deref())?;
    }
    Ok(())
  }
}
