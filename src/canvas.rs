use crossterm::style::{Attributes, Color, ContentStyle};
use euclid::default::{Point2D, Rect, Size2D};
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
    self.draw_buffer.resize(new_len, Default::default());
    self.active_buffer.resize(new_len, Default::default());
  }

  pub(crate) fn write(&mut self, point: &Point2D<usize>, data: &str) {
    assert!(self.frame.contains(point.clone()));
    let begin = (point.x * point.y) + point.x;
    let end = (begin + data.len()).min(self.draw_buffer.len());
    for (idx, value) in (begin..end).zip(data.chars()) {
      self.draw_buffer[idx].data = Some(value);
    }
  }

  pub(crate) fn fill_background(&mut self, rect: &Rect<usize>, color: &Color) {
    assert!(self.frame.contains_rect(rect));
    Self::traverse_buffer(rect, |idx| {
      self.draw_buffer[idx].style.background_color = Some(color.clone());
    });
  }

  pub(crate) fn fill_foreground(&mut self, rect: &Rect<usize>, color: &Color) {
    assert!(self.frame.contains_rect(rect));
    Self::traverse_buffer(rect, |idx| {
      self.draw_buffer[idx].style.foreground_color = Some(color.clone());
    });
  }

  pub(crate) fn merge_attributes(&mut self, rect: &Rect<usize>, attributes: Attributes) {
    assert!(self.frame.contains_rect(rect));
    Self::traverse_buffer(rect, |idx| {
      self.draw_buffer[idx].style.attributes = self.draw_buffer[idx].style.attributes | attributes;
    });
  }

  pub(crate) fn overwrite_attributes(&mut self, rect: &Rect<usize>, attributes: Attributes) {
    assert!(self.frame.contains_rect(rect));
    Self::traverse_buffer(rect, |idx| {
      self.draw_buffer[idx].style.attributes = attributes;
    });
  }

  pub(crate) fn clear_attributes(&mut self, rect: &Rect<usize>) {
    assert!(self.frame.contains_rect(rect));
    Self::traverse_buffer(rect, |idx| {
      self.draw_buffer[idx].style.attributes = Attributes::default();
    });
  }

  fn traverse_buffer<F: FnMut(/*idx:*/ usize)>(rect: &Rect<usize>, func: F) {
    rect
      .y_range()
      .zip(rect.x_range())
      .map(|(row, col)| col * row + col)
      .for_each(func);
  }

  pub(crate) fn render(&mut self) {
    let mut curr_pos = Point2D::new(0, 0);
    for (idx, (draw_cell, active_cell)) in self.draw_buffer.iter().zip(self.active_buffer.iter_mut()).enumerate() {
      if draw_cell != active_cell {
        *active_cell = draw_cell.clone();
      }
    }
  }
}
