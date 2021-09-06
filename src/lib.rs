extern crate euclid;

use std::any::Any;
use std::io::Write;
use std::time::Duration;

use crossterm::event::{Event, KeyCode, KeyModifiers, MouseEventKind};

#[cfg(feature = "logging")]
pub use crate::log::enable_pretty_env_logging;
use crate::render::RenderCtx;
use crate::util::{Scoped, ScopedMut};
use crate::widgets::flexible::FlexFit;
use crate::widgets::{AnyEvent, LayoutError, RenderResult, Widget};

pub mod canvas;
pub mod render;
pub mod util;
pub mod widgets;

pub(crate) mod log;
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
    fixed_sum += layout.min;
    flex_sum += layout.flex;
    final_values.push(layout.min);
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
    .filter(|(idx, layout)| layout.fit == FlexFit::Tight)
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
    .filter(|(idx, layout)| layout.fit == FlexFit::Loose)
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
