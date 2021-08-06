use crate::render::RenderCtx;
use crate::widgets::{LayoutResult, Widget};
use euclid::default::Size2D;
use std::any::Any;
use std::cmp::max;
use std::ops::{Deref, DerefMut, SubAssign};

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
      width: 10,
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

  fn layout(&mut self, max_size: &Size2D<usize>) -> LayoutResult {
    todo!()
  }

  fn render(&self, ctx: &mut RenderCtx) -> Option<()> {
    self.heading.render(ctx);
    Some(())
  }
}

pub trait TableColumn: Widget {
  fn get_width(&self) -> usize;
}

impl<Heading> TableColumn for Column<Heading>
where
  Heading: Widget,
{
  fn get_width(&self) -> usize {
    self.width
  }
}

pub trait TableColumns {
  fn len(&self) -> usize;
  fn column(&self, idx: usize) -> Option<&dyn TableColumn>;
  fn column_mut(&mut self, idx: usize) -> Option<&mut dyn TableColumn>;
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<C> TableColumns for Vec<C>
where
  C: TableColumn + 'static,
{
  fn len(&self) -> usize {
    Self::len(self)
  }

  fn column(&self, idx: usize) -> Option<&dyn TableColumn> {
    self.get(idx).and_then(|c| Some(c as &dyn TableColumn))
  }

  fn column_mut(&mut self, idx: usize) -> Option<&mut dyn TableColumn> {
    self.get_mut(idx).and_then(|c| Some(c as &mut dyn TableColumn))
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}

pub trait TableData {
  fn rows_len(&self) -> usize;
  fn cell(&self, row: usize, col: usize) -> Option<&dyn Widget>;
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

  fn cell(&self, row: usize, col: usize) -> Option<&dyn Widget> {
    self
      .get(row)
      .and_then(|v| v.get(col).and_then(|i| Some(i as &dyn Widget)))
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
pub struct Table {
  columns: Option<Box<dyn TableColumns>>,
  data: Option<Box<dyn TableData>>,
  // layout: all cells and columns rects
  // fixed_cols: usize,
  // fixed_rows: usize,
  // column_separator
}

impl Table {
  pub fn new() -> Self {
    Self {
      columns: None,
      data: None,
    }
  }

  pub fn columns<C: TableColumns + 'static>(mut self, columns: C) -> Self {
    self.columns = Some(Box::new(columns));
    self
  }

  pub fn columns_ref(&self) -> Option<&dyn TableColumns> {
    self.columns.as_ref().and_then(|cols| Some(cols.deref()))
  }

  pub fn columns_mut(&mut self) -> Option<&mut (dyn TableColumns + 'static)> {
    self.columns.as_mut().and_then(|cols| Some(cols.deref_mut()))
  }

  pub fn columns_ref_as<C: TableColumns + 'static>(&self) -> Option<&C> {
    self
      .columns
      .as_ref()
      .and_then(|cols| cols.deref().as_any().downcast_ref::<C>())
  }

  pub fn columns_mut_as<C: TableColumns + 'static>(&mut self) -> Option<&mut C> {
    self
      .columns
      .as_mut()
      .and_then(|cols| cols.deref_mut().as_any_mut().downcast_mut::<C>())
  }

  pub fn data<D: TableData + 'static>(mut self, data: D) -> Self {
    self.data = Some(Box::new(data));
    self
  }

  pub fn data_ref(&self) -> Option<&dyn TableData> {
    self.data.as_ref().and_then(|data| Some(data.deref()))
  }

  pub fn data_mut(&mut self) -> Option<&mut (dyn TableData + 'static)> {
    self.data.as_mut().and_then(|data| Some(data.deref_mut()))
  }

  pub fn data_ref_as<D: TableData + 'static>(&self) -> Option<&D> {
    self
      .data
      .as_ref()
      .and_then(|data| data.deref().as_any().downcast_ref::<D>())
  }

  pub fn data_mut_as<D: TableData + 'static>(&mut self) -> Option<&mut D> {
    self
      .data
      .as_mut()
      .and_then(|data| data.deref_mut().as_any_mut().downcast_mut::<D>())
  }

  // pub fn theme() -> Self {}
}

impl Widget for Table {
  fn event(&mut self) {
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&mut self, max_size: &Size2D<usize>) -> LayoutResult {
    let columns = self.columns_mut().unwrap();
    let mut size = max_size.clone();
    for c in 0..columns.len() {
      let column = columns.column_mut(c).unwrap();
      let col_layout = column.layout(&size)?;
      size.width -= col_layout.width;
      // TODO: continue here
      the_x += column.get_width() + 1;
      ctx.renderer.move_to_column_relative((the_x + 1) as u16);
    }

    Ok(Size2D::new(the_x, 2))
  }

  fn render(&self, ctx: &mut RenderCtx) -> Option<()> {
    let columns = self.columns_ref().unwrap();
    let mut the_x = 0;
    for c in 0..columns.len() {
      let column = columns.column(c).unwrap();
      // set render context, box constrains
      column.render(ctx);
      the_x += column.get_width() + 1;
      ctx.renderer.move_to_column_relative((the_x + 1) as u16);
    }

    ctx.renderer.next_line();

    let data = self.data_ref().unwrap();
    for r in 0..data.rows_len() {
      let mut the_x = 0;
      for c in 0..columns.len() {
        let column = columns.column(c).unwrap();
        if let Some(cell) = data.cell(r, c) {
          // set render context, box constrains
          cell.render(ctx);
        }
        the_x += column.get_width() + 1;
        ctx.renderer.move_to_column_relative((the_x + 1) as u16);
      }
      ctx.renderer.next_line();
    }

    Some(())
  }
}
