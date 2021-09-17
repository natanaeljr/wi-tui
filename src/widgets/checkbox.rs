use crate::render::RenderCtx;
use crate::widgets::{AnyEvent, Capability, EventResult, LayoutError, LayoutResult, LayoutSize, RenderResult, Widget};
use euclid::default::Size2D;

pub struct CheckBox {
  marked: bool,
}

impl CheckBox {
  pub fn new() -> Self {
    Self { marked: false }
  }

  pub fn marked(mut self) -> Self {
    self.marked = true;
    self
  }

  pub fn unmarked(mut self) -> Self {
    self.marked = false;
    self
  }
}

impl Widget for CheckBox {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    todo!()
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    let size = Size2D::new(1, 1);
    // check for minimum space in parent size
    if avail_size.contains(size.clone()) {
      Ok(LayoutSize::min_max(size.clone(), size))
    } else {
      Err(LayoutError::InsufficientSpace)
    }
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    if self.marked {
      ctx.renderer().write("◉");
    } else {
      ctx.renderer().write("○");
    }

    // if self.marked {
    //   ctx.renderer().write("▣"); // or ■
    // } else {
    //   ctx.renderer().write("▢"); // or □
    // }

    // if self.marked {
    //   ctx.renderer().write("☑");
    // } else {
    //   ctx.renderer().write("☐");
    // }

    // others: ◇ ◈ ◎ ◯

    // ref: https://www.fileformat.info/info/unicode/block/geometric_shapes/list.htm

    Ok(())
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    todo!()
  }
}
