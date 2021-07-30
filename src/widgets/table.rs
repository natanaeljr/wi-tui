use crate::render::RenderCtx;
use crate::widgets::Widget;
use std::ops::Deref;

pub struct Column<Heading> {
  heading: Heading,
  hidden: bool,
  fixed: bool,
}

impl<Heading> Column<Heading>
where
  Heading: Widget,
{
  pub fn heading(heading: Heading) -> Self {
    Self {
      heading,
      fixed: false,
      hidden: false,
    }
  }
}

impl<Heading> Widget for Column<Heading>
where
  Heading: Widget,
{
  fn event(&mut self) {
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&mut self) {
    todo!()
  }

  fn render(&self, ctx: &mut RenderCtx) {
    todo!()
  }
}
//
// impl<Heading> Into<Column<Heading>> for &str {
//   fn into(self) -> Column<Heading> {
//     Column::new(self)
//   }
// }
//
// impl<Heading> Into<Column<Heading>> for String {
//   fn into(self) -> Column<Heading> {
//     Column::new(self)
//   }
// }

pub trait TableColumns {
  type Heading;
  fn column(&self, idx: usize) -> Option<&Column<Self::Heading>>;
}

impl<Heading> TableColumns for Vec<Column<Heading>>
where
  Heading: Widget,
{
  type Heading = Heading;

  fn column(&self, idx: usize) -> Option<&Column<Self::Heading>> {
    self.get(idx)
  }
}

pub trait TableData {
  type Item;

  fn cell(&self, row: usize, col: usize) -> Option<&Self::Item>;
}

impl<Item> TableData for Vec<Item>
where
  Item: Widget,
{
  type Item = Item;

  fn cell(&self, row: usize, col: usize) -> Option<&Self::Item> {
    self.get(row * col)
  }
}

impl<Item> TableData for Vec<Vec<Item>>
where
  Item: Widget,
{
  type Item = Item;

  fn cell(&self, row: usize, col: usize) -> Option<&Self::Item> {
    self.get(row).and_then(|v| v.get(col))
  }
}

// TODO: Generic for Heading
pub struct Table<Item> {
  columns: Option<Box<dyn TableColumns<Heading = Item>>>,
  data: Option<Box<dyn TableData<Item = Item>>>,
  // layout: all cells and columns rects
}
impl<Item> Table<Item>
where
  Item: Widget,
{
  pub fn new() -> Self {
    Self {
      columns: None,
      data: None,
    }
  }

  pub fn columns<C: 'static>(mut self, columns: C) -> Self
  where
    C: TableColumns<Heading = Item>,
  {
    self.columns = Some(Box::new(columns));
    self
  }

  pub fn data<D: 'static>(mut self, data: D) -> Self
  where
    D: TableData<Item = Item>,
  {
    self.data = Some(Box::new(data));
    self
  }

  // pub fn theme() -> Self {}
}

impl<Item> Widget for Table<Item>
where
  Item: Widget,
{
  fn event(&mut self) {
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&mut self) {
    todo!()
  }

  fn render(&self, ctx: &mut RenderCtx) {
    for c in 0..1 {
      let column = self.columns.as_ref().unwrap().deref().column(c).unwrap();
      // set render context, box constrains
      column.render(ctx);
      for r in 0..1 {
        let cell = self.data.as_ref().unwrap().deref().cell(r, c).unwrap();
        // set render context, box constrains
        cell.render(ctx);
      }
    }
  }
}
