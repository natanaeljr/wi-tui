extern crate euclid;

use crate::render::RenderCtx;
use crate::widgets::{AnyEvent, RenderResult, Widget};
use crossterm::event::{Event, KeyCode, KeyModifiers, MouseEventKind};
use std::io::Write;
use std::time::Duration;

pub mod canvas;
pub mod render;
pub mod util;
pub mod widgets;

pub(crate) mod log;
#[cfg(feature = "logging")]
pub use crate::log::enable_pretty_env_logging;

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
          let (original_size, new_size) = flush_resize_events(Event::Resize(cols, rows));
          let (cols, rows) = new_size;
          // eprintln!("Resize from: {:?}, to: {:?}", original_size, new_size);
          if self.alternate {
            self.render_ctx.resize(cols as usize, rows as usize);
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
