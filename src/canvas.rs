use crossterm::cursor::{MoveTo, MoveToColumn, MoveToNextLine, MoveToRow};
use crossterm::style::{Attributes, Color, ContentStyle, Print, SetAttributes, SetBackgroundColor, SetForegroundColor, Attribute, SetAttribute};
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
  draw_buffer: Vec<Cell>,
  active_buffer: Vec<Cell>,
}

impl Canvas {
  pub(crate) fn new(size: Size2D<usize>) -> Self {
    assert_ne!(size.area(), 0);
    let frame = Rect::from_size(size);
    let mut buffer = Vec::<Cell>::new();
    buffer.resize(size.area(), Default::default());
    Self {
      frame,
      draw_buffer: buffer.clone(),
      active_buffer: buffer,
    }
  }

  pub(crate) fn resize(&mut self, size: Size2D<usize>) {
    assert_ne!(size.area(), 0);
    let new_len = size.area();
    self.frame = Rect::from_size(size);
    // --- maybe tmp?
    self.draw_buffer.clear();
    self.active_buffer.clear();
    execute!(std::io::stdout(), terminal::Clear(ClearType::All));
    // ---
    self.draw_buffer.resize(new_len, Default::default());
    self.active_buffer.resize(new_len, Default::default());
  }

  pub(crate) fn write(&mut self, point: &Point2D<usize>, data: &str) {
    assert!(self.frame.contains(point.clone()));
    let begin = (point.y * self.frame.width()) + point.x;
    let end = (begin + data.chars().count()).min(self.draw_buffer.len());
    for (idx, value) in (begin..end).zip(data.chars()) {
      self.draw_buffer[idx].data = Some(value);
    }
  }

  pub(crate) fn fill_background(&mut self, rect: &Rect<usize>, color: &Color) {
    assert!(self.frame.contains_rect(rect));
    Self::traverse_buffer(self.frame.width(), rect, |idx| {
      self.draw_buffer[idx].style.background_color = Some(color.clone());
    });
  }

  pub(crate) fn fill_foreground(&mut self, rect: &Rect<usize>, color: &Color) {
    assert!(self.frame.contains_rect(rect));
    Self::traverse_buffer(self.frame.width(), rect, |idx| {
      self.draw_buffer[idx].style.foreground_color = Some(color.clone());
    });
  }

  pub(crate) fn merge_attributes(&mut self, rect: &Rect<usize>, attributes: Attributes) {
    assert!(self.frame.contains_rect(rect));
    Self::traverse_buffer(self.frame.width(), rect, |idx| {
      self.draw_buffer[idx].style.attributes = self.draw_buffer[idx].style.attributes | attributes;
    });
  }

  pub(crate) fn overwrite_attributes(&mut self, rect: &Rect<usize>, attributes: Attributes) {
    assert!(self.frame.contains_rect(rect));
    Self::traverse_buffer(self.frame.width(), rect, |idx| {
      self.draw_buffer[idx].style.attributes = attributes;
    });
  }

  pub(crate) fn clear_attributes(&mut self, rect: &Rect<usize>) {
    assert!(self.frame.contains_rect(rect));
    Self::traverse_buffer(self.frame.width(), rect, |idx| {
      self.draw_buffer[idx].style.attributes = Attributes::default();
    });
  }

  fn traverse_buffer<F: FnMut(/*idx:*/ usize)>(width: usize, rect: &Rect<usize>, mut func: F) {
    for row in rect.y_range() {
      for col in rect.x_range() {
        let idx = width * row + col;
        func(idx);
      }
    }
  }

  pub(crate) fn render(&mut self) {
    let mut stdout = std::io::stdout();

    queue!(stdout, MoveTo(0, 0), SetBackgroundColor(Color::Reset), SetForegroundColor(Color::Reset), SetAttribute(Attribute::Reset));
    let mut cursor_pos = Point2D::<usize>::zero();
    let mut bg = Color::Reset;
    let mut fg = Color::Reset;
    let mut attributes = Attributes::default();

    for (idx, (draw_cell, active_cell)) in self.draw_buffer.iter_mut().zip(self.active_buffer.iter_mut()).enumerate() {
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
        update_cursor(&mut stdout);
        if !attributes.is_empty() {
          queue!(stdout, SetAttribute(Attribute::Reset));
        }
        attributes = draw_cell.style.attributes.clone();
        queue!(stdout, SetAttributes(draw_cell.style.attributes));
      }

      // background
      if draw_cell.style.background_color != active_cell.style.background_color || draw_cell.style.background_color.unwrap_or(Color::Reset) != bg {
        active_cell.style.background_color = draw_cell.style.background_color.clone();
        bg = draw_cell.style.background_color.unwrap_or(Color::Reset);
        update_cursor(&mut stdout);
        queue!(
          stdout,
          SetBackgroundColor(draw_cell.style.background_color.unwrap_or(Color::Reset))
        );
      }

      // foreground
      if draw_cell.style.foreground_color != active_cell.style.foreground_color || draw_cell.style.foreground_color.unwrap_or(Color::Reset) != fg {
        active_cell.style.foreground_color = draw_cell.style.foreground_color.clone();
        fg = draw_cell.style.foreground_color.unwrap_or(Color::Reset);
        update_cursor(&mut stdout);
        queue!(
          stdout,
          SetForegroundColor(draw_cell.style.foreground_color.unwrap_or(Color::Reset))
        );
      }

      // character
      active_cell.data = draw_cell.data;
      update_cursor(&mut stdout);
      queue!(stdout, Print(draw_cell.data.unwrap_or(' ')));

      // cursor move
      cursor_pos.x += 1;
      if cursor_pos.x >= self.frame.width() {
        queue!(stdout, MoveToNextLine(1));
        cursor_pos.x = 0;
        cursor_pos.y += 1;
      }
    }

    stdout.flush();
  }
}
