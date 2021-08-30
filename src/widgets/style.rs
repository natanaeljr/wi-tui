use crate::render::RenderCtx;
use crate::widgets::{AnyEvent, LayoutResult, RenderResult, Widget};
use euclid::default::Size2D;
use std::ops::{BitOr, Deref, DerefMut};

pub use crossterm::style::Attribute;
pub use crossterm::style::Attributes;
pub use crossterm::style::Color;
use crossterm::style::Stylize;

#[derive(Debug, Clone)]
pub struct Style {
  pub fg: Option<Color>,
  pub bg: Option<Color>,
  pub attrs: Attributes,
}

macro_rules! stylize_method {
  ($attr_method:ident Attribute::$attribute:ident) => {
    pub fn $attr_method(mut self) -> Self {
      self.attr(Attribute::$attribute)
    }
  };
  ($color_method_fg:ident, $color_method_bg:ident Color::$color:ident) => {
    pub fn $color_method_fg(mut self) -> Self {
      self.fg(Color::$color)
    }
    pub fn $color_method_bg(mut self) -> Self {
      self.bg(Color::$color)
    }
  };
}

impl Style {
  pub fn new() -> Self {
    Self {
      fg: None,
      bg: None,
      attrs: Default::default(),
    }
  }

  pub fn fg(mut self, color: Color) -> Self {
    self.fg = Some(color);
    self
  }

  pub fn bg(mut self, color: Color) -> Self {
    self.bg = Some(color);
    self
  }

  pub fn attr(mut self, attr: Attribute) -> Self {
    self.attrs = self.attrs | attr;
    self
  }

  pub fn apply<Child: Widget>(self, child: Child) -> Styled<Child> {
    Styled { style: self, child }
  }

  stylize_method!(reset Attribute::Reset);
  stylize_method!(bold Attribute::Bold);
  stylize_method!(underlined Attribute::Underlined);
  stylize_method!(reverse Attribute::Reverse);
  stylize_method!(dim Attribute::Dim);
  stylize_method!(italic Attribute::Italic);
  stylize_method!(negative Attribute::Reverse);
  stylize_method!(slow_blink Attribute::SlowBlink);
  stylize_method!(rapid_blink Attribute::RapidBlink);
  stylize_method!(hidden Attribute::Hidden);
  stylize_method!(crossed_out Attribute::CrossedOut);

  stylize_method!(black, on_black Color::Black);
  stylize_method!(dark_grey, on_dark_grey Color::DarkGrey);
  stylize_method!(red, on_red Color::Red);
  stylize_method!(dark_red, on_dark_red Color::DarkRed);
  stylize_method!(green, on_green Color::Green);
  stylize_method!(dark_green, on_dark_green Color::DarkGreen);
  stylize_method!(yellow, on_yellow Color::Yellow);
  stylize_method!(dark_yellow, on_dark_yellow Color::DarkYellow);
  stylize_method!(blue, on_blue Color::Blue);
  stylize_method!(dark_blue, on_dark_blue Color::DarkBlue);
  stylize_method!(magenta, on_magenta Color::Magenta);
  stylize_method!(dark_magenta, on_dark_magenta Color::DarkMagenta);
  stylize_method!(cyan, on_cyan Color::Cyan);
  stylize_method!(dark_cyan, on_dark_cyan Color::DarkCyan);
  stylize_method!(white, on_white Color::White);
  stylize_method!(grey, on_grey Color::Grey);
}

impl Default for Style {
  fn default() -> Self {
    Self::new()
  }
}

pub struct Styled<Child> {
  pub style: Style,
  pub child: Child,
}

impl<Child> Widget for Styled<Child>
where
  Child: Widget,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) {
    self.child.event(event, size)
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    self.child.layout(parent_size)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    if !self.style.attrs.is_empty() {
      ctx.renderer().add_attributes(self.style.attrs);
    }
    if let Some(bg) = self.style.bg.as_ref() {
      ctx.renderer().set_background(bg);
    }
    if let Some(fg) = self.style.fg.as_ref() {
      ctx.renderer().set_foreground(fg);
    }
    self.child.render(ctx)
  }
}
