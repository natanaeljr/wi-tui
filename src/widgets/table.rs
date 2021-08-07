use crate::render::RenderCtx;
use crate::widgets::{LayoutError, LayoutResult, RenderError, RenderResult, Widget};
use euclid::default::{Point2D, Rect, Size2D};
use std::any::Any;
use std::cmp::max;
use std::num::NonZeroUsize;
use std::ops::{Deref, DerefMut, Sub, SubAssign};

#[derive(Clone)]
pub enum ColumnAutoWidth {
  Heading,
  LargestCell,
  // TODO: LargestVisibleCell,
  // TODO: LargestCellOrHeading
  // TODO: LargestVisibleCellOrHeading
}

#[derive(Clone)]
pub enum ColumnMax {
  Fixed(NonZeroUsize),
  Auto,
}

#[derive(Clone)]
pub struct ColumnConstraints {
  pub min: NonZeroUsize,
  pub max: ColumnMax,
  pub auto: Option<ColumnAutoWidth>,
  pub flex_weight: usize,
  // pub flex_max_auto: bool = true,
}

impl Default for ColumnConstraints {
  fn default() -> Self {
    Self {
      min: NonZeroUsize::new(1).unwrap(),
      max: ColumnMax::Auto,
      auto: None,
      flex_weight: 1,
    }
  }
}

impl ColumnConstraints {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn min(mut self, min: NonZeroUsize) -> Self {
    self.min = min;
    self
  }
  pub fn max(mut self, max: ColumnMax) -> Self {
    self.max = max;
    self
  }
  pub fn auto(mut self, auto: ColumnAutoWidth) -> Self {
    self.auto = Some(auto);
    self
  }
}

#[derive(Clone)]
pub enum ColumnWidth {
  Fixed(NonZeroUsize),
  Dynamic(ColumnConstraints),
}

impl ColumnWidth {
  pub fn auto_heading() -> Self {
    ColumnWidth::Dynamic(ColumnConstraints::new().auto(ColumnAutoWidth::Heading))
  }
  pub fn auto_data() -> Self {
    ColumnWidth::Dynamic(ColumnConstraints::new().auto(ColumnAutoWidth::LargestCell))
  }
}

pub struct Column<Heading> {
  pub heading: Heading,
  pub width: ColumnWidth,
}

impl<Heading> Column<Heading>
where
  Heading: Widget,
{
  pub fn heading(heading: Heading) -> Self {
    Self {
      heading,
      width: ColumnWidth::auto_data(),
    }
  }
  pub fn width(mut self, width: ColumnWidth) -> Self {
    self.width = width;
    self
  }
  pub fn width_fixed(mut self, fixed: NonZeroUsize) -> Self {
    self.width = ColumnWidth::Fixed(fixed);
    self
  }
  pub fn width_auto_heading(mut self) -> Self {
    self.width = ColumnWidth::Dynamic(ColumnConstraints::new().auto(ColumnAutoWidth::Heading));
    self
  }
  pub fn width_auto_data(mut self) -> Self {
    self.width = ColumnWidth::Dynamic(ColumnConstraints::new().auto(ColumnAutoWidth::LargestCell));
    self
  }
  pub fn width_dynamic(mut self, constraints: ColumnConstraints) -> Self {
    self.width = ColumnWidth::Dynamic(constraints);
    self
  }
  pub fn width_no_flex(mut self) -> Self {
    match &mut self.width {
      ColumnWidth::Fixed(_) => {}
      ColumnWidth::Dynamic(constraints) => {
        constraints.flex_weight = 0;
      }
    }
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

  fn layout(&self, max_size: &Size2D<usize>) -> LayoutResult {
    self.heading.layout(max_size)
  }

  fn render(&self, ctx: &mut RenderCtx) -> RenderResult {
    self.heading.render(ctx);
    Ok(())
  }
}

pub trait TableColumn: Widget {
  fn get_width(&self) -> ColumnWidth;
}

impl<Heading> TableColumn for Column<Heading>
where
  Heading: Widget,
{
  fn get_width(&self) -> ColumnWidth {
    self.width.clone()
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
  fn cell_mut(&mut self, row: usize, col: usize) -> Option<&mut dyn Widget>;
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

  fn cell_mut(&mut self, row: usize, col: usize) -> Option<&mut dyn Widget> {
    self
      .get_mut(row)
      .and_then(|v| v.get_mut(col).and_then(|i| Some(i as &mut dyn Widget)))
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}

pub struct TableLayout {
  // fixed_rows
// fixed_cols
// column_separator
// must_show_all_columns
// flex_mode: first_cols/distributed/weight  // weight could be measured based on the computed auto width
// default col/row styles
}

// TODO: how to merge cells
// TODO: column indices for cherry-picking columns from TableData
//  that is to allow TableData to have more columns than what is shown.
pub struct Table {
  columns: Option<Box<dyn TableColumns>>,
  data: Option<Box<dyn TableData>>,
  layout: TableLayout,
}

impl Table {
  pub fn new() -> Self {
    Self {
      columns: None,
      data: None,
      layout: TableLayout {},
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

  fn layout_flex(size: &Size2D<usize>, input_layout: Vec<ColumnLayoutWidth>) -> Result<Vec<usize>, LayoutError> {
    let mut output_layout = Vec::new();
    output_layout.reserve(input_layout.len());

    let mut fixed_width = 0;
    let mut flex_total_weight = 0;
    for layout in input_layout.iter() {
      match layout {
        ColumnLayoutWidth::Fixed(fixed) => {
          fixed_width += fixed;
          output_layout.push(*fixed);
        }
        ColumnLayoutWidth::Flex { min, max, weight } => {
          fixed_width += min;
          flex_total_weight += weight;
          output_layout.push(0 /* computed later */);
        }
      }
    }

    if !size.contains(Size2D::new(fixed_width, 1)) {
      return Err(LayoutError::InsufficientSpace);
    }

    let mut total_width = fixed_width;
    let mut avail_flex_width = size.width - fixed_width;
    let mut flex_unit = avail_flex_width as f32 / flex_total_weight as f32;

    for (idx, layout) in input_layout.iter().enumerate() {
      match layout {
        ColumnLayoutWidth::Fixed(_) => {}
        ColumnLayoutWidth::Flex { min, max, weight } => {
          let flex = (*weight as f32 * flex_unit).round() as usize;
          let add_width = std::cmp::min(flex, *max - min);
          let rest = flex - add_width;
          avail_flex_width -= add_width;
          flex_total_weight -= weight;
          flex_unit = avail_flex_width as f32 / flex_total_weight as f32;
          output_layout[idx] = min + add_width;
          total_width += add_width;
        }
      }
    }

    if size.contains(Size2D::new(total_width, 1)) {
      Ok(output_layout)
    } else {
      Err(LayoutError::InsufficientSpace)
    }
  }

  fn layout_internal(&self, max_size: &Size2D<usize>) -> Result<(Size2D<usize>, Vec<ColumnLayoutWidth>), LayoutError> {
    let mut column_layout = Vec::new();

    let mut avail_size = max_size.clone();
    dbg!(max_size);
    let mut col_used_height = 0;

    let columns = self.columns_ref().unwrap();
    let data = self.data_ref().unwrap();

    for col in 0..columns.len() {
      let column = columns.column(col).unwrap();
      let column_size = column.layout(&avail_size)?;
      let column_width = match column.get_width() {
        ColumnWidth::Fixed(fixed) => {
          dbg!(avail_size);
          column_layout.push(ColumnLayoutWidth::Fixed(fixed.get()));
          fixed.get()
        }
        ColumnWidth::Dynamic(constraints) => {
          let dynamic_width = {
            let auto_width = if let Some(auto) = constraints.auto {
              match auto {
                ColumnAutoWidth::Heading => column_size.width,
                ColumnAutoWidth::LargestCell => {
                  let mut max_width = 0;
                  for row in 0..data.rows_len() {
                    if let Some(cell) = data.cell(row, col) {
                      let cell_width = cell.layout(&avail_size)?.width;
                      if cell_width > max_width {
                        max_width = cell_width;
                      }
                    }
                  }
                  max_width
                }
              }
            } else {
              0
            };
            let width = match constraints.max {
              ColumnMax::Fixed(max) => std::cmp::min(auto_width, max.get()),
              ColumnMax::Auto => auto_width,
            };
            let width = std::cmp::max(width, constraints.min.get());
            width
          };

          if constraints.flex_weight > 0 {
            column_layout.push(ColumnLayoutWidth::Flex {
              min: constraints.min.get(),
              max: match constraints.max {
                ColumnMax::Fixed(max) => max.get(),
                ColumnMax::Auto => dynamic_width,
              },
              weight: constraints.flex_weight,
            });
            constraints.min.get()
          } else {
            column_layout.push(ColumnLayoutWidth::Fixed(dynamic_width));
            dynamic_width
          }
        }
      };
      if !avail_size.contains(Size2D::new(column_width, column_size.height)) {
        return Err(LayoutError::InsufficientSpace);
      }
      avail_size.width -= column_width;
      col_used_height = std::cmp::max(col_used_height, column_size.height);
    }

    let mut row_used_height = 0;
    if data.rows_len() > 0 {
      let row_avail_size = Size2D::new(max_size.width, max_size.height - col_used_height);
      for col in 0..columns.len() {
        if let Some(cell) = data.cell(0, col) {
          let cell_height = cell.layout(&row_avail_size)?.height;
          row_used_height = std::cmp::max(row_used_height, cell_height);
        }
      }
    }

    let used_width = max_size.width - avail_size.width;
    let used_height = col_used_height + row_used_height;
    let min_size = Size2D::new(used_width, used_height);

    if max_size.contains(min_size.clone()) {
      Ok((min_size, column_layout))
    } else {
      Err(LayoutError::InsufficientSpace)
    }
  }
}

enum ColumnLayoutWidth {
  Fixed(usize),
  Flex { min: usize, max: usize, weight: usize },
}

impl Widget for Table {
  fn event(&mut self) {
    // NOTE: pass down to the column, make it possible for the column to spawn a Popup Menu with filled options,
    // as we go back up the hierarchy the Popup can be filled up.
    todo!()
  }

  fn update(&mut self) {
    todo!()
  }

  fn layout(&self, max_size: &Size2D<usize>) -> LayoutResult {
    self.layout_internal(max_size).map(|ok| ok.0)
  }

  fn render(&self, ctx: &mut RenderCtx) -> RenderResult {
    let (_, layout) = self
      .layout_internal(ctx.get_frame_size())
      .map_err(|err| RenderError::Layout(err))?;
    let flexed_layout = Self::layout_flex(ctx.get_frame_size(), layout).map_err(|err| RenderError::Layout(err))?;

    let columns = self.columns_ref().unwrap();
    let mut the_x = 0;
    for c in 0..columns.len() {
      let column = columns.column(c).unwrap();
      // TODO: set render context, box constrains
      let prev_frame = ctx.get_frame();
      let mut child_ctx = ctx.child_ctx(Rect::new(
        Point2D::new(ctx.get_frame().min_x() + the_x, ctx.get_frame().min_y()),
        Size2D::new(flexed_layout[c], 1),
      ));
      column.render(&mut child_ctx);
      ctx.set_frame(prev_frame);
      the_x += flexed_layout[c];
      // ctx.renderer.move_to_column_relative((the_x + 1) as u16);
    }

    // ctx.renderer.next_line();

    let data = self.data_ref().unwrap();
    for r in 0..data.rows_len() {
      let mut the_x = 0;
      for c in 0..columns.len() {
        let column = columns.column(c).unwrap();
        if let Some(cell) = data.cell(r, c) {
          // TODO: set render context, box constrains
          let prev_frame = ctx.get_frame();
          let mut child_ctx = ctx.child_ctx(Rect::new(
            Point2D::new(
              ctx.get_frame().min_x() + the_x,
              ctx.get_frame().min_y() + r + 1, /*col*/
            ),
            Size2D::new(flexed_layout[c], 1),
          ));
          cell.render(&mut child_ctx);
          ctx.set_frame(prev_frame);
        }
        the_x += flexed_layout[c];
        // ctx.renderer.move_to_column_relative((the_x + 1) as u16);
      }
      // ctx.renderer.next_line();
    }

    Ok(())
  }
}
