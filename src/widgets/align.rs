use crate::render::RenderCtx;
use crate::widgets::flexible::FlexFit;
use crate::widgets::{AnyEvent, EventResult, LayoutResult, RenderError, RenderResult, Widget};
use euclid::default::Size2D;
use euclid::SideOffsets2D;
use std::cmp::{max, min};
use std::ops::Sub;
use std::rc::Rc;

pub enum HorizontalAlignment {
  Left,
  Center { round_to: HorizontalSide },
  Right,
}

pub enum VerticalAlignment {
  Top,
  Middle { round_to: VerticalSide },
  Bottom,
}

pub enum HorizontalSide {
  Left,
  Right,
}

pub enum VerticalSide {
  Top,
  Bottom,
}

pub struct Align<Child> {
  pub vertical: VerticalAlignment,
  pub horizontal: HorizontalAlignment,
  pub child: Child,
}

impl<Child> Align<Child>
where
  Child: Widget,
{
  pub fn top_left(child: Child) -> Self {
    Self {
      vertical: VerticalAlignment::Top,
      horizontal: HorizontalAlignment::Left,
      child,
    }
  }
  pub fn top_center(child: Child) -> Self {
    Self {
      vertical: VerticalAlignment::Top,
      horizontal: HorizontalAlignment::Center {
        round_to: HorizontalSide::Left,
      },
      child,
    }
  }
  pub fn top_right(child: Child) -> Self {
    Self {
      vertical: VerticalAlignment::Top,
      horizontal: HorizontalAlignment::Right,
      child,
    }
  }
  pub fn middle_left(child: Child) -> Self {
    Self {
      vertical: VerticalAlignment::Middle {
        round_to: VerticalSide::Top,
      },
      horizontal: HorizontalAlignment::Left,
      child,
    }
  }
  pub fn center(child: Child) -> Self {
    Self {
      vertical: VerticalAlignment::Middle {
        round_to: VerticalSide::Top,
      },
      horizontal: HorizontalAlignment::Center {
        round_to: HorizontalSide::Left,
      },
      child,
    }
  }
  pub fn middle_right(child: Child) -> Self {
    Self {
      vertical: VerticalAlignment::Middle {
        round_to: VerticalSide::Top,
      },
      horizontal: HorizontalAlignment::Right,
      child,
    }
  }
  pub fn bottom_left(child: Child) -> Self {
    Self {
      vertical: VerticalAlignment::Bottom,
      horizontal: HorizontalAlignment::Left,
      child,
    }
  }
  pub fn bottom_center(child: Child) -> Self {
    Self {
      vertical: VerticalAlignment::Bottom,
      horizontal: HorizontalAlignment::Center {
        round_to: HorizontalSide::Left,
      },
      child,
    }
  }
  pub fn bottom_right(child: Child) -> Self {
    Self {
      vertical: VerticalAlignment::Bottom,
      horizontal: HorizontalAlignment::Right,
      child,
    }
  }

  pub fn vertical(mut self, vertical: VerticalAlignment) -> Self {
    self.vertical = vertical;
    self
  }

  pub fn horizontal(mut self, horizontal: HorizontalAlignment) -> Self {
    self.horizontal = horizontal;
    self
  }
}

impl<Child> Widget for Align<Child>
where
  Child: Widget,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    todo!()
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    self.child.layout(avail_size)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let frame = ctx.get_frame().size.clone();

    let layout = self.child.layout(&frame).map_err(|e| RenderError::Layout(e))?;
    let max_size = Size2D::new(min(layout.max.width, frame.width), min(layout.max.height, frame.height));
    let remainder = frame.sub(max_size);

    let mut offsets = SideOffsets2D::default();

    match &self.horizontal {
      HorizontalAlignment::Left => {
        offsets.right = remainder.width;
      }
      HorizontalAlignment::Right => {
        offsets.left = remainder.width;
      }
      HorizontalAlignment::Center { round_to: round } => {
        let half = remainder.width as f32 / 2.0;
        match round {
          HorizontalSide::Left => {
            offsets.left = half.floor() as usize;
            offsets.right = half.ceil() as usize;
          }
          HorizontalSide::Right => {
            offsets.left = half.ceil() as usize;
            offsets.right = half.floor() as usize;
          }
        }
      }
    }

    match &self.vertical {
      VerticalAlignment::Top => {
        offsets.bottom = remainder.height;
      }
      VerticalAlignment::Bottom => {
        offsets.top = remainder.height;
      }
      VerticalAlignment::Middle { round_to: round } => {
        let half = remainder.height as f32 / 2.0;
        match round {
          VerticalSide::Top => {
            offsets.top = half.floor() as usize;
            offsets.bottom = half.ceil() as usize;
          }
          VerticalSide::Bottom => {
            offsets.top = half.ceil() as usize;
            offsets.bottom = half.floor() as usize;
          }
        }
      }
    }

    let child_frame = ctx.get_frame().inner_rect(offsets);
    ctx.render_child_widget(child_frame, &self.child)
  }

  fn flex(&self) -> (usize, FlexFit) {
    self.child.flex()
  }
}
