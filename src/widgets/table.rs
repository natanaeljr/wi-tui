use crate::render::RenderCtx;
use crate::widgets::Widget;
use std::any::Any;
use std::ops::{Deref, DerefMut};

pub struct Column<Heading> {
  heading: Heading,
  hidden: bool,
  // TODO: width: Constraints
}

impl<Heading> Column<Heading>
where
  Heading: Widget,
{
  pub fn heading(heading: Heading) -> Self {
    Self { heading, hidden: false }
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

pub trait TableColumns: 'static {
  type Heading;
  fn get(&self, idx: usize) -> Option<&Column<Self::Heading>>;
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<Heading> TableColumns for Vec<Column<Heading>>
where
  Heading: Widget + 'static,
{
  type Heading = Heading;

  fn get(&self, idx: usize) -> Option<&Column<Self::Heading>> {
    self.get(idx)
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}

pub trait TableData: 'static {
  type Item;

  fn cell(&self, row: usize, col: usize) -> Option<&Self::Item>;
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<Item> TableData for Vec<Item>
where
  Item: Widget + 'static,
{
  type Item = Item;

  fn cell(&self, row: usize, col: usize) -> Option<&Self::Item> {
    self.get(row * col)
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}

impl<Item> TableData for Vec<Vec<Item>>
where
  Item: Widget + 'static,
{
  type Item = Item;

  fn cell(&self, row: usize, col: usize) -> Option<&Self::Item> {
    self.get(row).and_then(|v| v.get(col))
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}

// TODO: Generic for Heading
pub struct Table<Item> {
  columns: Option<Box<dyn TableColumns<Heading = Item>>>,
  data: Option<Box<dyn TableData<Item = Item>>>,
  // layout: all cells and columns rects
  // fixed_cols: usize,
  // fixed_rows: usize,
  // column_separator
}
impl<Item> Table<Item>
where
  Item: Widget + 'static,
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

  pub fn columns_ref(&self) -> Option<&dyn TableColumns<Heading = Item>> {
    self.columns.as_ref().and_then(|cols| Some(cols.deref()))
  }

  pub fn columns_mut(&mut self) -> Option<&mut dyn TableColumns<Heading = Item>> {
    self.columns.as_mut().and_then(|cols| Some(cols.deref_mut()))
  }

  pub fn columns_ref_as<D>(&self) -> Option<&D>
  where
    D: TableColumns<Heading = Item>,
  {
    self
      .columns
      .as_ref()
      .and_then(|cols| cols.deref().as_any().downcast_ref::<D>())
  }

  pub fn columns_mut_as<D>(&mut self) -> Option<&mut D>
  where
    D: TableColumns<Heading = Item>,
  {
    self
      .columns
      .as_mut()
      .and_then(|cols| cols.deref_mut().as_any_mut().downcast_mut::<D>())
  }

  pub fn data<D>(mut self, data: D) -> Self
  where
    D: TableData<Item = Item>,
  {
    self.data = Some(Box::new(data));
    self
  }

  pub fn data_ref(&self) -> Option<&dyn TableData<Item = Item>> {
    self.data.as_ref().and_then(|data| Some(data.deref()))
  }

  pub fn data_mut(&mut self) -> Option<&mut dyn TableData<Item = Item>> {
    self.data.as_mut().and_then(|data| Some(data.deref_mut()))
  }

  pub fn data_ref_as<D>(&self) -> Option<&D>
  where
    D: TableData<Item = Item>,
  {
    self
      .data
      .as_ref()
      .and_then(|data| data.deref().as_any().downcast_ref::<D>())
  }

  pub fn data_mut_as<D>(&mut self) -> Option<&mut D>
  where
    D: TableData<Item = Item>,
  {
    self
      .data
      .as_mut()
      .and_then(|data| data.deref_mut().as_any_mut().downcast_mut::<D>())
  }

  // pub fn theme() -> Self {}
}

impl<Item> Widget for Table<Item>
where
  Item: Widget + 'static,
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
      let column = self.columns_ref().unwrap().get(c).unwrap();
      // set render context, box constrains
      column.render(ctx);
      for r in 0..1 {
        let cell = self.data_ref().unwrap().cell(r, c).unwrap();
        // set render context, box constrains
        cell.render(ctx);
      }
    }
  }
}
