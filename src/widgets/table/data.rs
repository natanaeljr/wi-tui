use std::any::Any;

use crate::util::Scoped;
use crate::widgets::Widget;

pub trait TableData {
  fn rows_len(&self) -> usize;
  fn cell(&self, row: usize, col: usize) -> Option<Scoped<dyn Widget>>;
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<Item> TableData for Vec<Vec<Item>>
where
  Item: Widget + 'static,
{
  fn rows_len(&self) -> usize {
    self.len()
  }

  fn cell(&self, row: usize, col: usize) -> Option<Scoped<dyn Widget>> {
    self
      .get(row)
      .and_then(|vec| vec.get(col).and_then(|cell| Some(Scoped::Ref(cell as &dyn Widget))))
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}
