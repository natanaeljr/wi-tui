use crate::render::RenderCtx;
use crate::widgets::{AnyEvent, Capability, EventResult, LayoutError, LayoutResult, LayoutSize, RenderResult, Widget};
use euclid::default::Size2D;
use std::borrow::Cow;

pub enum TextWrap {
  Word,
  Hard,
}

pub enum TextAlign {
  Left,
  Center,
  Right,
  Justify,
}

pub enum TextOverflow {
  Fold,
  Crop,
  Symbol(char),
}

pub struct Text {
  pub wrap: TextWrap,
  pub align: TextAlign,
  pub overflow: TextOverflow,
  pub data: String,
}

impl Text {
  pub fn new<S: ToString>(data: S) -> Self {
    Self {
      wrap: TextWrap::Word,
      align: TextAlign::Left,
      overflow: TextOverflow::Fold,
      data: data.to_string(),
    }
  }

  pub fn align(mut self, alignment: TextAlign) -> Self {
    self.align = alignment;
    self
  }
}

impl Widget for Text {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    self.data.event(event, size)
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutSize {
    let min = Size2D::new(1, 1);
    let wrapped_text = textwrap::wrap(self.data.as_str(), textwrap::Options::new(avail_size.width));
    let mut longest = 0;
    for line in wrapped_text.iter() {
      let count = line.as_ref().chars().count();
      if count > longest {
        longest = count;
      }
    }
    let max = Size2D::new(longest, wrapped_text.len());
    LayoutSize::min_max(min, max)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let frame = ctx.get_frame().clone();

    match self.align {
      TextAlign::Left | TextAlign::Center | TextAlign::Right => {
        let wrapped_text = textwrap::wrap(self.data.as_str(), textwrap::Options::new(frame.width()));
        for line in wrapped_text.iter().take(frame.height()) {
          match self.align {
            TextAlign::Left => {
              ctx.renderer().write(line.as_ref());
            }
            TextAlign::Center => {
              let padding = frame.width() - line.as_ref().chars().count();
              ctx.renderer().move_right(padding / 2);
              ctx.renderer().write(line.as_ref());
            }
            TextAlign::Right => {
              let padding = frame.width() - line.as_ref().chars().count();
              ctx.renderer().move_right(padding);
              ctx.renderer().write(line.as_ref());
            }
            TextAlign::Justify => {}
          }
          ctx.renderer().next_line();
        }
      }
      TextAlign::Justify => {
        let mut settings = justify::Settings::default();
        settings.width = frame.width();
        settings.separator = "\n";
        let str = justify::justify(self.data.as_str(), &settings);
        for line in str.lines() {
          ctx.renderer().write(line);
          ctx.renderer().next_line();
        }
      }
    }

    Ok(())
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    self.data.has_capability(capability)
  }
}
