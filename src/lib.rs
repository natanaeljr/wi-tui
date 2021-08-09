extern crate euclid;

use crate::render::RenderCtx;
use crate::widgets::{Widget, RenderResult};
use crossterm::event::{Event, KeyCode, KeyModifiers};

pub mod render;
pub mod util;
pub mod widgets;

pub struct TuiController {
  alternate: bool,
  render_ctx: RenderCtx,
  pub root_widget: Box<dyn Widget>,
}

impl TuiController {
  pub fn new(alternate: bool) -> Self {
    Self {
      alternate,
      render_ctx: RenderCtx::new(alternate),
      root_widget: Box::new(""),
    }
  }

  pub fn root_widget<'a, W>(mut self, root_widget: W) -> Self
  where
    W: Widget + 'static,
  {
    self.root_widget = Box::new(root_widget);
    self
  }

  pub fn run(&mut self) -> RenderResult {
    loop {
      self.root_widget.render(&mut self.render_ctx)?;
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
        Event::Mouse(_) => {}
        Event::Resize(cols, rows) => {
          if self.alternate {
            self.render_ctx.resize(cols as usize, rows as usize);
            break;
          }
        }
      }
    }
  }
}
