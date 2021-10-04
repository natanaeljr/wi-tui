use euclid::default::Size2D;

use crate::render::RenderCtx;
use crate::widgets::{AnyEvent, Capability, EventResult, LayoutResult, LayoutSize, RenderResult, Widget};
use crate::FlexFit;

pub struct FillChar {
  char: char,
}

impl FillChar {
  pub fn new(char: char) -> Self {
    Self { char }
  }
}

impl Widget for FillChar {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    todo!()
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutSize {
    LayoutSize::min_max(avail_size.clone(), avail_size.clone())
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let frame = ctx.get_frame();
    let str = self.char.to_string().repeat(frame.width());
    for row in 1..frame.height() {
      ctx.renderer().write(str.as_str());
      ctx.renderer().next_line();
    }
    ctx.renderer().write(str.as_str());
    Ok(())
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    todo!()
  }
}
