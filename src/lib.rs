extern crate euclid;

use std::any::Any;
use std::io::Write;
use std::time::Duration;

pub use crossterm::event::{Event, KeyCode, KeyModifiers, MouseEventKind};
pub use crossterm::style::{Attribute, Attributes, Color};

#[cfg(feature = "logging")]
pub use crate::log::enable_pretty_env_logging;
use crate::render::RenderCtx;
use crate::util::{Scoped, ScopedMut};
use crate::widgets::{AnyEvent, LayoutError, RenderResult, Styled, Widget};

#[macro_use]
pub(crate) mod log;

pub mod canvas;
pub mod render;
pub mod util;
pub mod widgets;

// TODO: Make all Widgets divisible between property and widget so we can have (along with WidgetExt):
// let widget = "Hello".bordered(Borders::lines());

pub struct WiTui {
  alternate: bool,
  render_ctx: RenderCtx,
  pub root: Box<dyn Widget>,
}

impl WiTui {
  pub fn root_widget<W: Widget + 'static>(root: W) -> Self {
    Self {
      alternate: false,
      render_ctx: RenderCtx::new(false),
      root: Box::new(root) as Box<dyn Widget>,
    }
  }

  pub fn alternate(mut self, alternate: bool) -> Self {
    // temporary hack to recreate renderer
    drop(self.render_ctx);
    Self {
      alternate,
      render_ctx: RenderCtx::new(alternate),
      root: self.root,
    }
  }

  pub fn print(&mut self) -> RenderResult {
    let result = self.root.render(&mut self.render_ctx);
    self.render_ctx.renderer().flush();
    result
  }

  pub fn quit(mut self) {}

  // TODO: Compute FPS, ms/frame
  //  Provide API with rendering info, for displaying in a widget
  pub fn run_loop(&mut self) -> RenderResult {
    loop {
      self.print();
      if !self.alternate {
        break;
      }
      let mut quit = false;
      self.event_loop(&mut quit);
      if quit {
        break;
      }
    }
    Ok(())
  }

  fn event_loop(&mut self, quit: &mut bool) {
    loop {
      match crossterm::event::read().unwrap() {
        Event::Key(key) => {
          if key.modifiers == KeyModifiers::empty() {
            match key.code {
              KeyCode::Char('q') => {
                *quit = true;
                break;
              }
              _ => {}
            }
          }
        }
        Event::Mouse(mouse) => {
          if let MouseEventKind::Down(_) = mouse.kind {
            self
              .root
              .event(&AnyEvent::Input(Event::Mouse(mouse)), &self.render_ctx.get_frame().size);
            break;
          }
        }
        Event::Resize(cols, rows) => {
          // let (original_size, new_size) = flush_resize_events(Event::Resize(cols, rows));
          // let (cols, rows) = new_size;
          // eprintln!("Resize from: {:?}, to: {:?}", original_size, new_size);
          if self.alternate {
            self.render_ctx.resize(cols as usize, rows as usize);
            self.render_ctx.renderer().force_render_once(); // TEMPORARY: just for resize (URXVT BUG)
            break; // TODO: uncomment
          }
        }
      }
    }
  }
}

// Resize events can occur in batches.
// With a simple loop they can be flushed.
// This function will keep the first and last resize event.
fn flush_resize_events(event: Event) -> ((u16, u16), (u16, u16)) {
  if let Event::Resize(x, y) = event {
    let mut last_resize = (x, y);
    // while let Ok(true) = crossterm::event::poll(Duration::from_millis(50)) {
    //   if let Ok(Event::Resize(x, y)) = crossterm::event::read() {
    //     last_resize = (x, y);
    //   }
    // }
    return ((x, y), last_resize);
  }
  ((0, 0), (0, 0))
}

pub trait ChildrenStorage: 'static {
  fn len(&self) -> usize;
  fn child(&self, index: usize) -> Option<Scoped<dyn Widget>>;
  fn child_mut(&mut self, index: usize) -> Option<ScopedMut<dyn Widget>>;
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<W> ChildrenStorage for Vec<W>
where
  W: Widget + 'static,
{
  fn len(&self) -> usize {
    self.len()
  }

  fn child(&self, index: usize) -> Option<Scoped<dyn Widget>> {
    self.get(index).and_then(|c| Some(Scoped::Ref(c as &dyn Widget)))
  }

  fn child_mut(&mut self, index: usize) -> Option<ScopedMut<dyn Widget>> {
    self
      .get_mut(index)
      .and_then(|c| Some(ScopedMut::Ref(c as &mut dyn Widget)))
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}

#[derive(Debug)]
struct MinMaxFlex {
  min: usize,
  max: usize,
  flex: usize,
  fit: FlexFit,
}

fn compute_flex_layout(avail_size: usize, input_layout: &Vec<MinMaxFlex>) -> Result<(usize, Vec<usize>), LayoutError> {
  let mut final_values = Vec::new();
  final_values.reserve(input_layout.len());

  let mut fixed_sum = 0;
  let mut flex_sum = 0;
  for (col, layout) in input_layout.iter().enumerate() {
    let fixed = if layout.flex == 0 { layout.max } else { layout.min };
    fixed_sum += fixed;
    flex_sum += layout.flex;
    final_values.push(fixed);
  }

  if avail_size < fixed_sum {
    return Err(LayoutError::InsufficientSpace);
  }

  let mut total_space = fixed_sum;
  let mut avail_flex_space = avail_size - fixed_sum;
  let mut flex_unit = avail_flex_space as f32 / flex_sum as f32;
  let mut flex_sum = flex_sum as f32;

  // Compute additional space for flexible layouts
  for (idx, layout) in input_layout
    .iter()
    .enumerate()
    .filter(|(idx, layout)| layout.fit == FlexFit::Loose)
  {
    // compute actual flex constrained to min/max
    let flex_space = (flex_unit * layout.flex as f32).round() as usize;
    let avail_space = layout.max - layout.min;
    let layout_add = std::cmp::min(avail_space, flex_space);
    // recalculate flex unit if there were remainder, to add more for other layouts and fit the space
    let flex_remainder = flex_space - layout_add;
    let actual_flex = (layout_add as f32 * layout.flex as f32) / flex_space as f32;
    info!("TIGHT: actual_flex: {}", actual_flex);
    avail_flex_space -= layout_add;
    flex_sum -= layout.flex as f32;
    flex_unit = avail_flex_space as f32 / flex_sum as f32;
    // add space to layout minimum to flex it
    final_values[idx] += layout_add;
    total_space += layout_add;
  }

  // Compute additional space for flexible layouts
  for (idx, layout) in input_layout
    .iter()
    .enumerate()
    .filter(|(idx, layout)| layout.fit == FlexFit::Tight)
  {
    info!("LOOSE");
    // compute actual flex constrained to min/max
    let flex_space = (flex_unit * layout.flex as f32).round() as usize;
    // let avail_space = layout.max - layout.min;
    // let layout_add = std::cmp::min(avail_space, flex_space);
    let layout_add = flex_space;
    // recalculate flex unit if there were remainder, to add more for other layouts and fit the space
    let flex_remainder = flex_space - layout_add;
    avail_flex_space -= layout_add;
    flex_sum -= layout.flex as f32;
    flex_unit = avail_flex_space as f32 / flex_sum as f32;
    // add space to layout minimum to flex it
    final_values[idx] += layout_add;
    total_space += layout_add;
  }

  // Finally check again the space and return the final spaces
  if avail_size >= total_space {
    Ok((total_space, final_values))
  } else {
    Err(LayoutError::InsufficientSpace)
  }
}

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

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum FlexFit {
  Tight,
  Loose,
}

macro_rules! stylize_method {
  ($attr_method:ident, Attribute::$attribute:ident) => {
    pub fn $attr_method(mut self) -> Self {
      self.attr(Attribute::$attribute)
    }
  };
  ($color_method_fg:ident, $color_method_bg:ident, Color::$color:ident) => {
    pub fn $color_method_fg(mut self) -> Self {
      self.fg(Color::$color)
    }
    pub fn $color_method_bg(mut self) -> Self {
      self.bg(Color::$color)
    }
  };
}

#[derive(Debug, Clone)]
pub struct Style {
  pub fg: Option<Color>,
  pub bg: Option<Color>,
  pub attrs: Attributes,
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

  pub fn child<Child: Widget>(self, child: Child) -> Styled<Child> {
    Styled { style: self, child }
  }

  stylize_method!(reset, Attribute::Reset);
  stylize_method!(bold, Attribute::Bold);
  stylize_method!(underlined, Attribute::Underlined);
  stylize_method!(reverse, Attribute::Reverse);
  stylize_method!(dim, Attribute::Dim);
  stylize_method!(italic, Attribute::Italic);
  stylize_method!(negative, Attribute::Reverse);
  stylize_method!(slow_blink, Attribute::SlowBlink);
  stylize_method!(rapid_blink, Attribute::RapidBlink);
  stylize_method!(hidden, Attribute::Hidden);
  stylize_method!(crossed_out, Attribute::CrossedOut);

  stylize_method!(black, on_black, Color::Black);
  stylize_method!(dark_grey, on_dark_grey, Color::DarkGrey);
  stylize_method!(red, on_red, Color::Red);
  stylize_method!(dark_red, on_dark_red, Color::DarkRed);
  stylize_method!(green, on_green, Color::Green);
  stylize_method!(dark_green, on_dark_green, Color::DarkGreen);
  stylize_method!(yellow, on_yellow, Color::Yellow);
  stylize_method!(dark_yellow, on_dark_yellow, Color::DarkYellow);
  stylize_method!(blue, on_blue, Color::Blue);
  stylize_method!(dark_blue, on_dark_blue, Color::DarkBlue);
  stylize_method!(magenta, on_magenta, Color::Magenta);
  stylize_method!(dark_magenta, on_dark_magenta, Color::DarkMagenta);
  stylize_method!(cyan, on_cyan, Color::Cyan);
  stylize_method!(dark_cyan, on_dark_cyan, Color::DarkCyan);
  stylize_method!(white, on_white, Color::White);
  stylize_method!(grey, on_grey, Color::Grey);
}

impl Default for Style {
  fn default() -> Self {
    Self::new()
  }
}
