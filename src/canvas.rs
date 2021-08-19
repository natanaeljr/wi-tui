use crossterm::cursor::{MoveTo, MoveToColumn, MoveToNextLine, MoveToRow};
use crossterm::style::{
  Attribute, Attributes, Color, ContentStyle, Print, SetAttribute, SetAttributes, SetBackgroundColor,
  SetForegroundColor,
};
use crossterm::terminal::ClearType;
use crossterm::{cursor, execute, queue, terminal};
use euclid::default::{Point2D, Rect, Size2D};
use std::io::{Stdout, Write};
use std::iter::{Map, Zip};
use std::ops::BitOr;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Cell {
  data: Option<char>,
  style: ContentStyle,
}

pub struct Canvas {
  frame: Rect<usize>,
  draw_buffer: Vec<Vec<Cell>>,   // Rows<Cols<Cells>>
  active_buffer: Vec<Vec<Cell>>, // Rows<Cols<Cells>>
}

impl Canvas {
  pub(crate) fn new(size: Size2D<usize>) -> Self {
    assert_ne!(size.area(), 0);
    let frame = Rect::from_size(size);
    let mut rows = Vec::<Vec<Cell>>::new();
    rows.resize_with(size.height, || {
      let mut cols = Vec::<Cell>::new();
      cols.resize(size.width, Cell::default());
      cols
    });
    Self {
      frame,
      draw_buffer: rows.clone(),
      active_buffer: rows,
    }
  }

  pub(crate) fn resize(&mut self, size: Size2D<usize>) {
    assert_ne!(size.area(), 0);
    self.frame = Rect::from_size(size.clone());
    // --- maybe tmp? yes! TODO: remove
    // self.draw_buffer.clear();
    // self.active_buffer.clear();
    // execute!(std::io::stdout(), terminal::Clear(ClearType::All));
    // ---
    self.draw_buffer.resize_with(size.height, || {
      let mut cols = Vec::<Cell>::new();
      cols.resize(size.width, Cell::default());
      cols
    });
    self.active_buffer.resize_with(size.height, || {
      let mut cols = Vec::<Cell>::new();
      cols.resize(size.width, Cell::default());
      cols
    });
    // updater older rows
    self
      .draw_buffer
      .iter_mut()
      .for_each(|row| row.resize(size.width, Cell::default()));
    self
      .active_buffer
      .iter_mut()
      .for_each(|row| row.resize(size.width, Cell::default()));
  }

  pub(crate) fn write(&mut self, point: &Point2D<usize>, data: &str) {
    assert!(self.frame.contains(point.clone()));
    for (idx, char) in data.chars().take(self.frame.width()).enumerate() {
      self.draw_buffer[point.y][point.x + idx].data = Some(char);
    }
  }

  pub(crate) fn fill_background(&mut self, rect: &Rect<usize>, color: &Color) {
    assert!(self.frame.contains_rect(rect));
    for row in rect.y_range() {
      for col in rect.x_range() {
        self.draw_buffer[row][col].style.background_color = Some(color.clone());
      }
    }
  }

  pub(crate) fn fill_foreground(&mut self, rect: &Rect<usize>, color: &Color) {
    assert!(self.frame.contains_rect(rect));
    for row in rect.y_range() {
      for col in rect.x_range() {
        self.draw_buffer[row][col].style.foreground_color = Some(color.clone());
      }
    }
  }

  pub(crate) fn merge_attributes(&mut self, rect: &Rect<usize>, attributes: Attributes) {
    assert!(self.frame.contains_rect(rect));
    for row in rect.y_range() {
      for col in rect.x_range() {
        self.draw_buffer[row][col].style.attributes = self.draw_buffer[row][col].style.attributes | attributes;
      }
    }
  }

  pub(crate) fn overwrite_attributes(&mut self, rect: &Rect<usize>, attributes: Attributes) {
    assert!(self.frame.contains_rect(rect));
    for row in rect.y_range() {
      for col in rect.x_range() {
        self.draw_buffer[row][col].style.attributes = attributes;
      }
    }
  }

  pub(crate) fn clear_attributes(&mut self, rect: &Rect<usize>) {
    assert!(self.frame.contains_rect(rect));
    for row in rect.y_range() {
      for col in rect.x_range() {
        self.draw_buffer[row][col].style.attributes = Attributes::default();
      }
    }
  }

  pub(crate) fn render(&mut self) {
    let mut stdout = std::io::stdout();

    queue!(
      stdout,
      MoveTo(0, 0),
      SetBackgroundColor(Color::Reset),
      SetForegroundColor(Color::Reset),
      SetAttribute(Attribute::Reset)
    );
    let mut cursor_pos = Point2D::<usize>::zero();
    let mut bg = Color::Reset;
    let mut fg = Color::Reset;
    let mut attributes = Attributes::default();

    for (idx, (draw_cell, active_cell)) in self
      .draw_buffer
      .iter_mut()
      .flatten()
      .zip(self.active_buffer.iter_mut().flatten())
      .enumerate()
    {
      // if idx > self.frame.width() * 12 {
      //   break;
      // }
      // cursor set
      let row = idx / self.frame.width();
      let col = idx % self.frame.width();
      let mut update_cursor = |stdout: &mut Stdout| {
        // TODO: optimization for short diff values (use space then)
        // if cursor_pos.x != col {
        //   if cursor_pos.y != row {
        //     queue!(stdout, MoveTo(col as u16, row as u16));
        //   } else {
        //     queue!(stdout, MoveToColumn(col as u16));
        //   }
        // } else if cursor_pos.y != row {
        //   queue!(stdout, MoveToRow(row as u16));
        // }
      };

      // modifiers
      if draw_cell.style.attributes != attributes {
        active_cell.style.attributes = draw_cell.style.attributes.clone();
        // update_cursor(&mut stdout);
        if !attributes.is_empty() {
          queue!(stdout, SetAttribute(Attribute::Reset));
        }
        attributes = draw_cell.style.attributes.clone();
        queue!(stdout, SetAttributes(draw_cell.style.attributes));
      }

      // background
      if draw_cell.style.background_color != active_cell.style.background_color
        || draw_cell.style.background_color.unwrap_or(Color::Reset) != bg
      {
        active_cell.style.background_color = draw_cell.style.background_color.clone();
        bg = draw_cell.style.background_color.unwrap_or(Color::Reset);
        // update_cursor(&mut stdout);
        queue!(
          stdout,
          SetBackgroundColor(draw_cell.style.background_color.unwrap_or(Color::Reset))
        );
      }

      // foreground
      if draw_cell.style.foreground_color != active_cell.style.foreground_color
        || draw_cell.style.foreground_color.unwrap_or(Color::Reset) != fg
      {
        active_cell.style.foreground_color = draw_cell.style.foreground_color.clone();
        fg = draw_cell.style.foreground_color.unwrap_or(Color::Reset);
        // update_cursor(&mut stdout);
        queue!(
          stdout,
          SetForegroundColor(draw_cell.style.foreground_color.unwrap_or(Color::Reset))
        );
      }

      // character
      active_cell.data = draw_cell.data;
      // update_cursor(&mut stdout);
      queue!(stdout, Print(draw_cell.data.unwrap_or(' ')));

      // cursor move
      cursor_pos.x += 1;
      if cursor_pos.x >= self.frame.width() {
        queue!(stdout, MoveToNextLine(1));
        cursor_pos.x = 0;
        cursor_pos.y += 1;
      }

      // clear the draw cell
      *draw_cell = Cell::default();
    }

    // Pro Tip from VIM analysis: always reset the cursor to (0,0) after the rendering!
    // this makes the terminal not cause an auto-scroll that flickers the screen when resizing the window to smaller rows values.
    queue!(stdout, MoveTo(0, 0));

    stdout.flush();
  }
}
