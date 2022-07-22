use crate::render::RenderCtx;
use crate::widgets::{AnyEvent, Capability, EventResult, LayoutError, LayoutResult, LayoutSize, RenderResult, Widget};
use crate::{Event, MouseEventKind};
use euclid::default::Size2D;

pub struct CheckBox {
  marked: bool,
  marked_str: &'static str,
  unmarked_str: &'static str,
}

impl CheckBox {
  pub fn new() -> Self {
    Self { marked: false, marked_str: "◉", unmarked_str: "○" }
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
    match event {
      AnyEvent::Input(event) => match event {
        Event::Mouse(mouse) => match mouse.kind {
          MouseEventKind::Down(_) => {
            self.marked = !self.marked;
            return EventResult::Done;
          }
          _ => {}
        },
        _ => {}
      },
    }
    EventResult::Unhandled
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
      ctx.renderer().write(self.marked_str);
    } else {
      ctx.renderer().write(self.unmarked_str);
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
