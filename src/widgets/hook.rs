use crate::render::RenderCtx;
use crate::widgets::{AnyEvent, Capability, EventResult, LayoutResult, LayoutSize, RenderResult, Widget};
use crate::FlexFit;
use euclid::default::Size2D;
use std::ops::{Deref, DerefMut};

pub struct Hook<Child> {
  on_event: Box<dyn FnMut(/*child:*/ &mut Child, /*event:*/ &AnyEvent, /*size:*/ &Size2D<usize>) -> EventResult>,
  on_layout: Box<dyn Fn(/*child:*/ &Child, /*avail_size:*/ &Size2D<usize>) -> LayoutSize>,
  on_render: Box<dyn Fn(/*child:*/ &Child, /*ctx:*/ &RenderCtx) -> RenderResult>,
  on_has_capability: Box<dyn Fn(/*child:*/ &Child, /*capability:*/ &Capability) -> bool>,
  child: Child,
}

impl<Child> Hook<Child>
where
  Child: Widget,
{
  pub fn child(child: Child) -> Self {
    Self {
      on_event: Box::new(|child, event, size| child.event(event, size)),
      on_layout: Box::new(|child, avail_size| child.layout(avail_size)),
      on_render: Box::new(|child, render_ctx| render_ctx.render_child_widget(render_ctx.get_frame().clone(), child)),
      on_has_capability: Box::new(|child, capability| child.has_capability(capability)),
      child,
    }
  }

  pub fn on_event<F>(mut self, func: F) -> Self
  where
    F: 'static + FnMut(/*child:*/ &mut Child, /*event:*/ &AnyEvent, /*size:*/ &Size2D<usize>) -> EventResult,
  {
    self.on_event = Box::new(func);
    self
  }

  pub fn on_layout<F>(mut self, func: F) -> Self
  where
    F: 'static + Fn(/*child:*/ &Child, /*avail_size:*/ &Size2D<usize>) -> LayoutSize,
  {
    self.on_layout = Box::new(func);
    self
  }

  pub fn on_render<F>(mut self, func: F) -> Self
  where
    F: 'static + Fn(/*child:*/ &Child, /*ctx:*/ &RenderCtx) -> RenderResult,
  {
    self.on_render = Box::new(func);
    self
  }

  pub fn on_has_capability<F>(mut self, func: F) -> Self
  where
    F: 'static + Fn(/*child:*/ &Child, /*capability:*/ &Capability) -> bool,
  {
    self.on_has_capability = Box::new(func);
    self
  }
}

impl<Child> Widget for Hook<Child> {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    self.on_event.deref_mut()(&mut self.child, event, size)
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutSize {
    self.on_layout.deref()(&self.child, avail_size)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    self.on_render.deref()(&self.child, ctx)
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    self.on_has_capability.deref()(&self.child, capability)
  }
}
