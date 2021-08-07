use crossterm::{cursor, execute, terminal};
use euclid::default::{Box2D, Point2D, Rect, Size2D};
use std::borrow::BorrowMut;
use std::cell::{Cell, RefCell, RefMut};
use std::io::Write;
use std::ops::Deref;
use std::rc::Rc;

// TODO: Check https://docs.rs/sdl2/0.34.5/sdl2/render/

pub struct Renderer {
  size: Size2D<usize>,
  reset_pos: Point2D<usize>,
  frame: Rect<usize>,
  frame_cursor: Point2D<usize>,
  nl_counter: usize,
}

impl Renderer {
  pub fn new() -> Self {
    // println!();
    terminal::enable_raw_mode().unwrap();
    let (cols, rows) = terminal::size().unwrap();
    let (pos_c, pos_r) = cursor::position().unwrap();
    let mut stdout = std::io::stdout();
    // execute!(stdout, terminal::EnterAlternateScreen);
    let mut this = Self {
      size: Size2D::new(cols as usize, rows as usize),
      reset_pos: Point2D::new(pos_c as usize, pos_r as usize),
      frame: Default::default(),
      frame_cursor: Point2D::new(0, 0),
      nl_counter: 0,
    };
    this.set_frame(Rect::from_size(Size2D::new(cols as usize, rows as usize)));
    this
  }

  pub fn print(&mut self, buf: &str) {
    // std::thread::sleep(std::time::Duration::from_millis(500));
    let space = self.frame.width() - (self.frame_cursor.x - self.frame.origin.x);
    if buf.len() > space {
      let (buf, _) = buf.split_at(space);
      print!("{}", buf);
      self.frame_cursor.x += space;
    } else {
      print!("{}", buf);
      self.frame_cursor.x += buf.len();
    }
    std::io::stdout().flush();
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
    execute!(
      stdout,
      cursor::MoveTo(frame.min_x() as u16, (frame.min_y() + self.reset_pos.y) as u16)
    );

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
    execute!(stdout, cursor::MoveTo(x, y + self.reset_pos.y as u16));
    Some(())
  }

  pub fn move_to_column_relative(&mut self, x: u16) -> Option<()> {
    let the_x = self.frame.min_x() as u16 + x;
    if the_x > self.frame.max_x() as u16 {
      return None;
    }
    self.frame_cursor.x = the_x as usize;
    let mut stdout = std::io::stdout();
    execute!(stdout, cursor::MoveToColumn(the_x));
    Some(())
  }
}

impl Drop for Renderer {
  fn drop(&mut self) {
    let mut stdout = std::io::stdout();
    // execute!(stdout, terminal::LeaveAlternateScreen);
    // self.move_to(0, (self.size.height) as u16);
    execute!(stdout, cursor::MoveTo(0, (self.reset_pos.y + self.nl_counter) as u16),);
    // std::thread::sleep(std::time::Duration::from_secs(1));
    terminal::disable_raw_mode().unwrap();
    println!();
  }
}

pub struct RenderCtx {
  // widget constraints box
  renderer: Rc<RefCell<Renderer>>,
  frame_size: Size2D<usize>,
}

impl RenderCtx {
  pub fn new() -> Self {
    let mut this = Self {
      renderer: Rc::new(RefCell::new(Renderer::new())),
      frame_size: Default::default(),
    };
    let tmp = this.renderer().frame.size.clone();
    this.frame_size = tmp;
    this
  }
  pub fn child_ctx(&self, frame: Rect<usize>) -> Self {
    let mut this = Self {
      renderer: self.renderer.clone(),
      frame_size: Default::default(),
    };
    this.set_frame(frame);
    this
  }
  pub fn renderer(&mut self) -> RefMut<'_, Renderer> {
    self.renderer.deref().borrow_mut()
  }
  pub fn set_frame(&mut self, frame: Rect<usize>) {
    self.frame_size = frame.size.clone();
    self.renderer().set_frame(frame);
  }
  pub fn get_frame(&self) -> Rect<usize> {
    self.renderer.deref().borrow().frame.clone()
  }
  pub fn get_frame_size(&self) -> &Size2D<usize> {
    &self.frame_size
  }
}
