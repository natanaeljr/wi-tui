use crate::render::RenderCtx;
use crate::widgets::Widget;
use std::any::Any;
use std::ops::{Deref, DerefMut};

pub struct Column<Heading> {
  heading: Heading,
  hidden: bool,
  width: usize,
  // TODO: width: Constraints
}

impl<Heading> Column<Heading>
where
  Heading: Widget,
{
  pub fn heading(heading: Heading) -> Self {
    Self {
      heading,
      hidden: false,
      width: 8,
    }
  }
  pub fn width(mut self, width: usize) -> Self {
    self.width = width;
    self
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

  fn render(&self, ctx: &mut RenderCtx) -> Option<()> {
    self.heading.render(ctx);
    Some(())
  }
}

pub trait TableColumns: 'static {
  type Heading;
  fn len(&self) -> usize;
  fn column(&self, idx: usize) -> Option<&Column<Self::Heading>>;
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<Heading> TableColumns for Vec<Column<Heading>>
where
  Heading: Widget + 'static,
{
  type Heading = Heading;

  fn len(&self) -> usize {
    Self::len(self)
  }

  fn column(&self, idx: usize) -> Option<&Column<Self::Heading>> {
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
  fn rows_len(&self) -> usize;
  fn cell(&self, row: usize, col: usize) -> Option<&Self::Item>;
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<Item> TableData for Vec<Vec<Item>>
where
  Item: Widget + 'static,
{
  type Item = Item;

  fn rows_len(&self) -> usize {
    self.len()
  }

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

// TODO: how to merge cells
// TODO: Generic for Heading
// TODO: column indices for cherry-picking columns from TableData
//  that is to allow TableData to have more columns than what is shown.
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

  fn render(&self, ctx: &mut RenderCtx) -> Option<()> {
    let columns = self.columns_ref().unwrap();
    let mut the_x = 0;
    for c in 0..columns.len() {
      let column = columns.column(c).unwrap();
      // set render context, box constrains
      column.render(ctx);
      the_x += column.width + 1;
      ctx.renderer.move_to_column_relative((the_x + 1) as u16);
    }

    ctx.renderer.next_line();

    let data = self.data_ref().unwrap();
    for r in 0..data.rows_len() {
      let mut the_x = 0;
      for c in 0..columns.len() {
        let column = columns.column(c).unwrap();
        let cell = data.cell(r, c).unwrap();
        // set render context, box constrains
        cell.render(ctx);
        the_x += column.width + 1;
        ctx.renderer.move_to_column_relative((the_x + 1) as u16);
      }
      ctx.renderer.next_line();
    }

    Some(())
  }
}
