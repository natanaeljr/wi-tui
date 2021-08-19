extern crate euclid;

use crate::render::RenderCtx;
use crate::widgets::{RenderResult, Widget};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::io::Write;

pub mod canvas;
pub mod render;
pub mod util;
pub mod widgets;

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
