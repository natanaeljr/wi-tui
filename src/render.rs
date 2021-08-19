use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, Ref, RefCell, RefMut};
use std::io::Write;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crossterm::style::{Attributes, Color};
use crossterm::terminal::{ClearType, DisableLineWrap, EnableLineWrap, ScrollUp, ScrollDown};
use crossterm::{cursor, execute, terminal};
use euclid::default::{Box2D, Point2D, Rect, Size2D};

use crate::canvas::Canvas;
use crate::util::{Immut, Immutable};
use crate::widgets::{RenderResult, Widget};

// TODO: Check https://docs.rs/sdl2/0.34.5/sdl2/render/

// TODO:
//  PUSH settings on Context creation
//  POP settings on Context drop

// TODO: push/pop context with widget ID, for event hierarchy tracking later

pub struct Renderer {
  size: Size2D<usize>,
  reset_pos: Point2D<usize>,
  frame: Rect<usize>,
  frame_cursor: Point2D<usize>,
  nl_counter: usize,
  alternate: bool,
  canvas: Canvas,
}

impl Renderer {
  pub(crate) fn new(alternate: bool) -> Self {
    let mut stdout = std::io::stdout();
    if alternate {
      execute!(stdout, terminal::EnterAlternateScreen);
      execute!(stdout, cursor::MoveTo(0, 0));
      execute!(stdout, DisableLineWrap);
    }
    terminal::enable_raw_mode().unwrap();
    let (cols, rows) = terminal::size().unwrap();
    let (pos_c, pos_r) = cursor::position().unwrap_or((0,0));
    let mut stdout = std::io::stdout();
    let mut this = Self {
      size: Size2D::new(cols as usize, rows as usize),
      reset_pos: Point2D::new(pos_c as usize, pos_r as usize),
      frame: Default::default(),
      frame_cursor: Point2D::new(0, 0),
      nl_counter: 0,
      alternate,
      canvas: Canvas::new(Size2D::new(cols as usize, rows as usize)),
    };
    this.set_frame(Rect::from_size(Size2D::new(cols as usize, rows as usize)));
    this
  }

  fn resize(&mut self, cols: usize, rows: usize) {
    // return; // TODO: remove
    assert!(self.alternate);
    self.size.width = cols;
    self.size.height = rows;
    self.nl_counter = 0;
    self.set_frame(Rect::from_size(Size2D::new(cols as usize, rows as usize)));
    self.canvas.resize(Size2D::new(cols as usize, rows as usize));
    let mut stdout = std::io::stdout();
    // execute!(stdout, terminal::Clear(ClearType::All));
  }

  pub fn write(&mut self, buf: &str) {
    // std::thread::sleep(std::time::Duration::from_millis(500));
    let space = self.frame.width() - (self.frame_cursor.x - self.frame.origin.x);
    if buf.chars().count() > space {
      let (buf, _) = buf.split_at(space);
      // print!("{}", buf);
      self.canvas.write(&self.frame_cursor, buf);
      self.frame_cursor.x += space;
    } else {
      // print!("{}", buf);
      self.canvas.write(&self.frame_cursor, buf);
      self.frame_cursor.x += buf.len();
    }
    // std::io::stdout().flush();
  }

  pub(crate) fn flush(&mut self) {
    self.canvas.render();
  }

  pub fn set_background(&mut self, color: &Color) {
    self.canvas.fill_background(&self.frame, color);
  }

  pub fn set_foreground(&mut self, color: &Color) {
    self.canvas.fill_foreground(&self.frame, color);
  }

  pub fn set_attributes(&mut self, attributes: Attributes) {
    self.canvas.overwrite_attributes(&self.frame, attributes);
  }

  pub fn add_attributes(&mut self, attributes: Attributes) {
    self.canvas.merge_attributes(&self.frame, attributes);
  }

  pub fn next_line(&mut self) {
    if self.frame_cursor.y >= self.frame.max_y() {
      return;
    }
    self.frame_cursor.x = self.frame.min_x();
    self.frame_cursor.y += 1;
    let mut stdout = std::io::stdout();
    execute!(
      stdout,
      cursor::MoveTo(
        self.frame.min_x() as u16,
        (self.frame_cursor.y + self.reset_pos.y) as u16
      ),
      // cursor::MoveToNextLine(1),
      // cursor::MoveToColumn(self.frame.min_x() as u16)
    );
    if self.reset_pos.y + self.frame_cursor.y >= self.size.height {
      self.reset_pos.y -= 1;
      execute!(stdout, crossterm::terminal::ScrollUp(1),);
    }
    if self.frame_cursor.y >= self.nl_counter {
      self.nl_counter += 1;
    }
    // print!("{:?} {:?} {:?}", self.reset_pos, self.frame_cursor, self.size);
    // std::thread::sleep(std::time::Duration::from_secs(5));
  }

  fn set_frame(&mut self, frame: Rect<usize>) {
    // let frame = Rect::from_size(
    //   (frame.min_x(), frame.min_y() + self.reset_pos.y),
    //   (frame.width(), frame.height()),
    // )
    // .unwrap();
    self.frame = frame;
    self.frame_cursor = frame.min();
    let mut stdout = std::io::stdout();
    // execute!(
    //   stdout,
    //   cursor::MoveTo(frame.min_x() as u16, (frame.min_y() + self.reset_pos.y) as u16)
    // );

    if self.reset_pos.y + self.frame_cursor.y + 1 >= self.size.height {
      let diff = self.reset_pos.y + self.frame_cursor.y + 1 - self.size.height;
      self.reset_pos.y -= diff;
      execute!(stdout, crossterm::terminal::ScrollUp(diff as u16),);
    }
    if self.frame_cursor.y >= self.nl_counter {
      self.nl_counter = self.frame_cursor.y;
    }
  }

  pub fn move_to(&mut self, x: u16, y: u16) -> Option<()> {
    if x < self.frame.min_x() as u16 || x > self.frame.max_x() as u16 {
      return None;
    }
    if y < self.frame.min_y() as u16 || y > self.frame.max_y() as u16 {
      return None;
    }
    self.frame_cursor = Point2D::new(x as usize, y as usize);
    let mut stdout = std::io::stdout();
    // execute!(stdout, cursor::MoveTo(x, y + self.reset_pos.y as u16));
    Some(())
  }

  pub fn move_to_column_relative(&mut self, x: u16) -> Option<()> {
    let the_x = self.frame.min_x() as u16 + x;
    if the_x > self.frame.max_x() as u16 {
      return None;
    }
    self.frame_cursor.x = the_x as usize;
    let mut stdout = std::io::stdout();
    // execute!(stdout, cursor::MoveToColumn(the_x));
    Some(())
  }
}

impl Drop for Renderer {
  fn drop(&mut self) {
    let mut stdout = std::io::stdout();

    if self.alternate {
      // std::thread::sleep(std::time::Duration::from_secs(20));
      execute!(stdout, EnableLineWrap);
      execute!(stdout, terminal::LeaveAlternateScreen);
    } else {
      execute!(stdout, cursor::MoveTo(0, (self.reset_pos.y + self.nl_counter) as u16),);
    }

    terminal::disable_raw_mode().unwrap();

    if !self.alternate {
      println!();
    }
    stdout.flush();
  }
}

pub struct RenderCtx {
  // widget constraints box
  renderer: Rc<RefCell<Renderer>>,
  frame: Rect<usize>,
  depth: usize,
  // parent: Option<Box<RenderCtx>>,
}

impl RenderCtx {
  pub(crate) fn new(alternate: bool) -> Self {
    let mut this = Self {
      renderer: Rc::new(RefCell::new(Renderer::new(alternate))),
      frame: Default::default(),
      depth: 0,
      // parent: None,
    };
    let frame = this.renderer().frame.clone();
    this.frame = frame;
    this
  }

  pub fn renderer(&self) -> RefMut<'_, Renderer> {
    self.renderer.deref().borrow_mut()
  }

  pub fn get_frame(&self) -> &Rect<usize> {
    &self.frame
  }

  fn set_frame(&mut self, frame: Rect<usize>) {
    self.frame = frame.clone();
    self.renderer().set_frame(frame);
  }

  #[inline]
  pub fn render_child<W: Widget>(&self, frame: Rect<usize>, child: &W) -> RenderResult {
    self.render_child_widget(frame, child as &dyn Widget)
  }

  pub fn render_child_widget(&self, frame: Rect<usize>, child: &dyn Widget) -> RenderResult {
    let mut ctx = Self {
      renderer: self.renderer.clone(),
      frame,
      depth: self.depth + 1,
    };
    self.renderer().set_frame(ctx.frame.clone());
    let result = child.render(&ctx);
    self.renderer().set_frame(self.frame.clone());
    result
  }

  pub(crate) fn resize(&mut self, cols: usize, rows: usize) {
    self.renderer.deref().borrow_mut().resize(cols, rows);
    let frame = self.renderer().frame.clone();
    self.frame = frame;
  }
}

impl Immutable for RenderCtx {}
