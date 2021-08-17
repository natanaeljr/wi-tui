use crate::render::RenderCtx;
use crate::widgets::{LayoutResult, RenderError, RenderResult, Widget};
use euclid::default::Size2D;
use euclid::SideOffsets2D;
use std::ops::Sub;
use std::rc::Rc;

pub enum HorizontalAlignment {
  Left,
  Right,
  Center,
}

pub struct Align<Child> {
  pub child: Child,
  pub vertical: u32, /* TODO: enum */
  pub horizontal: HorizontalAlignment,
}

impl<Child> Align<Child>
where
  Child: Widget,
{
  pub fn center(child: Child) -> Self {
    Self {
      child,
      vertical: 0,
      horizontal: HorizontalAlignment::Center,
    }
  }
  pub fn right(child: Child) -> Self {
    Self {
      child,
      vertical: 0,
      horizontal: HorizontalAlignment::Right,
    }
  }
  pub fn left(child: Child) -> Self {
    Self {
      child,
      vertical: 0,
      horizontal: HorizontalAlignment::Left,
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

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    self.child.layout(parent_size)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let parent_size = ctx.get_frame().size.clone();
    let mut child_size = self.child.layout(&parent_size).map_err(|e| RenderError::Layout(e))?;
    child_size.max.width = std::cmp::min(child_size.max.width, parent_size.width);
    child_size.max.height = std::cmp::min(child_size.max.height, parent_size.height);
    let remainder_size = parent_size.sub(child_size.max);
    let offsets = match self.horizontal {
      HorizontalAlignment::Left => SideOffsets2D::new(0, remainder_size.width, 0, 0),
      HorizontalAlignment::Right => SideOffsets2D::new(0, 0, 0, remainder_size.width),
      HorizontalAlignment::Center => {
        let half = remainder_size.width as f32 / 2.0;
        let left = half.floor() as usize;
        let right = half.ceil() as usize;
        SideOffsets2D::new(0, right, 0, left)
      }
    };

    let child_frame = ctx.get_frame().inner_rect(offsets);
    // let mut child_ctx = myctx.into_child_ctx(child_frame);
    // self.child.render(&child_ctx);
    ctx.render_child(child_frame, &self.child)
  }
}
