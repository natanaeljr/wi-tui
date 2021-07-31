use crate::rect::Rect;
use crossterm::{cursor, execute, terminal};
use std::io::Write;

pub struct Renderer {
  size: (usize, usize),      // xy, cr
  reset_pos: (usize, usize), // xy, cr
  frame: Rect,
  frame_cursor: (usize, usize), // xy, cr
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
      size: (cols as usize, rows as usize),
      reset_pos: (pos_c as usize, pos_r as usize),
      frame: Default::default(),
      frame_cursor: (0, 0),
      nl_counter: 0,
    };
    this.set_frame(Rect::from_size((0, 0), (rows as usize, cols as usize)).unwrap());
    this
  }

  pub fn print(&mut self, buf: &str) {
    std::thread::sleep(std::time::Duration::from_millis(100));
    let space = self.frame.width() - self.frame_cursor.0;
    if buf.len() > space {
      let (buf, _) = buf.split_at(space);
      print!("{}", buf);
      self.frame_cursor.0 += space;
    } else {
      print!("{}", buf);
      self.frame_cursor.0 += buf.len();
    }
    std::io::stdout().flush();
  }

  pub fn next_line(&mut self) {
    if self.frame_cursor.1 >= self.frame.y.1 {
      return;
    }
    self.frame_cursor.0 = self.frame.x.0;
    self.frame_cursor.1 += 1;
    let mut stdout = std::io::stdout();
    execute!(
      stdout,
      cursor::MoveTo(self.frame.x.0 as u16, (self.frame_cursor.1 + self.reset_pos.1) as u16),
      // cursor::MoveToNextLine(1),
      // cursor::MoveToColumn(self.frame.x.0 as u16)
    );
    if self.reset_pos.1 + self.frame_cursor.1 >= self.size.1 {
      self.reset_pos.1 -= 1;
      execute!(stdout, crossterm::terminal::ScrollUp(1),);
    }
    if self.frame_cursor.1 >= self.nl_counter {
      self.nl_counter += 1;
    }
    // print!("{:?} {:?} {:?}", self.reset_pos, self.frame_cursor, self.size);
    // std::thread::sleep(std::time::Duration::from_secs(5));
  }

  pub fn set_frame(&mut self, frame: Rect) {
    // let frame = Rect::from_size(
    //   (frame.x.0, frame.y.0 + self.reset_pos.1),
    //   (frame.width(), frame.height()),
    // )
    // .unwrap();
    self.frame = frame;
    self.frame_cursor = (frame.x.0, frame.y.0);
    let mut stdout = std::io::stdout();
    execute!(
      stdout,
      cursor::MoveTo(frame.x.0 as u16, (frame.y.0 + self.reset_pos.1) as u16)
    );
  }

  pub fn move_to(&mut self, x: u16, y: u16) -> Option<()> {
    if x < self.frame.x.0 as u16 || x > self.frame.x.1 as u16 {
      return None;
    }
    if y < self.frame.y.0 as u16 || y > self.frame.y.1 as u16 {
      return None;
    }
    self.frame_cursor = (x as usize, y as usize);
    let mut stdout = std::io::stdout();
    execute!(stdout, cursor::MoveTo(x, y + self.reset_pos.1 as u16));
    Some(())
  }
}

impl Drop for Renderer {
  fn drop(&mut self) {
    let mut stdout = std::io::stdout();
    // execute!(stdout, terminal::LeaveAlternateScreen);
    // self.move_to(0, (self.size.1) as u16);
    execute!(stdout, cursor::MoveTo(0, (self.reset_pos.1 + self.nl_counter) as u16),);
    std::thread::sleep(std::time::Duration::from_secs(2));
    terminal::disable_raw_mode().unwrap();
    println!();
  }
}

pub struct RenderCtx {
  // widget constraints box
  pub renderer: Renderer,
}
