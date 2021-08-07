use std::any::Any;
use std::borrow::Cow;
use std::cmp::max;
use std::num::NonZeroUsize;
use std::ops::{Deref, DerefMut, Sub, SubAssign};

use crossterm::style::{ContentStyle, StyledContent, Stylize};
use euclid::default::{Point2D, Rect, Size2D};

use crate::render::RenderCtx;
use crate::util::Scoped;
use crate::widgets::{LayoutError, LayoutResult, LayoutSize, RenderError, RenderResult, Widget};

#[derive(Clone)]
pub enum ColumnWidthAuto {
  /// Do not adjust column width automatically
  Off,
  /// Adjust column width to the width of the heading
  Heading,
  /// Adjust column width to the width of the largest cell in that column
  AllCells,
  /// Adjust column width to the width of the largest _visible_ cell in that column
  VisibleCells,
  /// Adjust column width to the width of the largest cell in that column or the heading
  AllCellsOrHeading,
  /// Adjust column width to the width of the largest _visible_ cell in that column or the heading
  VisibleCellsOrHeading,
}

#[derive(Clone)]
pub enum ColumnWidthValue {
  /// Use a fixed value
  Fixed(usize),
  /// Use value computed from the auto setting
  Auto,
}

/// Combinations for "abcdefghijklmnopqrswxyz"
///  Fixed:           { min: 10, max: 10,      auto: None,        flex_weight: 0 } => "abcdefghij"
///  Min:             { min: 5,  max: 10,      auto: None,        flex_weight: 0 } => "abcde"
///  Distributed:     { min: 5,  max: 10|Auto, auto: None,        flex_weight: 1 } => "abcdef...j"
///  AutoFixed:       { min: 1,  max: Auto,    auto: LargestCell, flex_weight: 0 } => "abcdefghijklmnopqrswxyz"
///  AutoFlexed:      { min: 1,  max: Auto,    auto: LargestCell, flex_weight: 1 } => "a..z"
///  AutoFlexedBeyond:{ min: 1,  max: Inf,     auto: LargestCell, flex_weight: 1 } => "abc... "
#[derive(Clone)]
pub struct ColumnWidth {
  /// Minimum column width
  pub min: ColumnWidthValue,
  /// Maximum column width
  pub max: ColumnWidthValue,
  /// Automatic column width adjustment
  pub auto_mode: ColumnWidthAuto,
  /// Flex column width to available space based on weight; zero is no flex.
  pub flex_weight: usize,
}

impl Default for ColumnWidth {
  fn default() -> Self {
    Self::new()
  }
}

impl ColumnWidth {
  pub fn new() -> Self {
    Self {
      min: ColumnWidthValue::Fixed(1),
      max: ColumnWidthValue::Auto,
      auto_mode: ColumnWidthAuto::AllCellsOrHeading,
      flex_weight: 1,
    }
  }

  pub fn min(mut self, min: ColumnWidthValue) -> Self {
    self.min = min;
    self
  }

  pub fn min_auto(mut self) -> Self {
    self.min = ColumnWidthValue::Auto;
    self
  }

  pub fn min_fixed(mut self, min: usize) -> Self {
    self.min = ColumnWidthValue::Fixed(min);
    self
  }

  pub fn max(mut self, max: ColumnWidthValue) -> Self {
    self.max = max;
    self
  }

  pub fn max_fixed(mut self, fixed: usize) -> Self {
    self.max = ColumnWidthValue::Fixed(fixed);
    self
  }

  pub fn max_auto(mut self) -> Self {
    self.max = ColumnWidthValue::Auto;
    self
  }

  pub fn auto_mode(mut self, auto_mode: ColumnWidthAuto) -> Self {
    self.auto_mode = auto_mode;
    self
  }

  pub fn flex_weight(mut self, weight: usize) -> Self {
    self.flex_weight = weight;
    self
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
  pub fn new(heading: Heading) -> Self {
    Self {
      heading,
      width: ColumnWidth::default(),
    }
  }
  pub fn width(mut self, width: ColumnWidth) -> Self {
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

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    self.heading.layout(parent_size)
  }

  fn render(&self, ctx: &mut RenderCtx) -> RenderResult {
    self.heading.render(ctx);
    Ok(())
  }
}

pub trait TableColumn: Widget {
  fn get_width(&self) -> Cow<ColumnWidth>;
}

impl<Heading> TableColumn for Column<Heading>
where
  Heading: Widget,
{
  fn get_width(&self) -> Cow<ColumnWidth> {
    Cow::Borrowed(&self.width)
  }
}

pub trait TableColumns {
  fn len(&self) -> usize;
  fn column(&self, idx: usize) -> Option<Scoped<dyn TableColumn>>;
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

  fn column(&self, idx: usize) -> Option<Scoped<dyn TableColumn>> {
    self.get(idx).and_then(|col| Some(Scoped::Ref(col as &dyn TableColumn)))
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

pub struct TableLayout {
  /// Number of fixed top rows
  pub fixed_rows: usize,
  /// Number of
  pub fixed_cols: usize,
  /// Whether to render column headings or not
  pub hide_headings: bool,
  /// Column separator
  pub column_separator: char,
  /// Rendering must fit all columns or render nothing at all (declare insufficient space)
  pub must_render_fit_all_columns: bool,
}

impl Default for TableLayout {
  fn default() -> Self {
    Self::new()
  }
}

impl TableLayout {
  pub fn new() -> Self {
    Self {
      fixed_rows: 0,
      fixed_cols: 0,
      hide_headings: false,
      column_separator: ' ',
      must_render_fit_all_columns: false,
    }
  }

  pub fn fixed_rows(mut self, fixed_rows: usize) -> Self {
    self.fixed_cols = fixed_rows;
    self
  }

  pub fn fixed_cols(mut self, fixed_cols: usize) -> Self {
    self.fixed_cols = fixed_cols;
    self
  }

  pub fn hide_headings(mut self, hide_headings: bool) -> Self {
    self.hide_headings = hide_headings;
    self
  }

  pub fn column_separator(mut self, column_separator: char) -> Self {
    self.column_separator = column_separator;
    self
  }

  pub fn must_render_fit_all_columns(mut self, must_render_fit_all_columns: bool) -> Self {
    self.must_render_fit_all_columns = must_render_fit_all_columns;
    self
  }
}

// TODO: how to merge cells
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
      layout: TableLayout::default(),
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

  pub fn layout(mut self, layout: TableLayout) -> Self {
    self.layout = layout;
    self
  }

  // TODO: pub fn theme() -> Self {}

  fn layout_flex(size: &Size2D<usize>, input_layout: Vec<ColumnLayoutWidth>) -> Result<Vec<usize>, LayoutError> {
    // let mut output_layout = Vec::new();
    // output_layout.reserve(input_layout.len());
    //
    // let mut fixed_width = 0;
    // let mut flex_total_weight = 0;
    // for layout in input_layout.iter() {
    //   match layout {
    //     ColumnLayoutWidth::Fixed(fixed) => {
    //       fixed_width += fixed;
    //       output_layout.push(*fixed);
    //     }
    //     ColumnLayoutWidth::Flex { min, max, weight } => {
    //       fixed_width += min;
    //       flex_total_weight += weight;
    //       output_layout.push(0 /* computed later */);
    //     }
    //   }
    // }
    //
    // if !size.contains(Size2D::new(fixed_width, 1)) {
    //   return Err(LayoutError::InsufficientSpace);
    // }
    //
    // let mut total_width = fixed_width;
    // let mut avail_flex_width = size.width - fixed_width;
    // let mut flex_unit = avail_flex_width as f32 / flex_total_weight as f32;
    //
    // for (idx, layout) in input_layout.iter().enumerate() {
    //   match layout {
    //     ColumnLayoutWidth::Fixed(_) => {}
    //     ColumnLayoutWidth::Flex { min, max, weight } => {
    //       let flex = (*weight as f32 * flex_unit).round() as usize;
    //       let add_width = std::cmp::min(flex, *max - min);
    //       let rest = flex - add_width;
    //       avail_flex_width -= add_width;
    //       flex_total_weight -= weight;
    //       flex_unit = avail_flex_width as f32 / flex_total_weight as f32;
    //       output_layout[idx] = min + add_width;
    //       total_width += add_width;
    //     }
    //   }
    // }
    //
    // if size.contains(Size2D::new(total_width, 1)) {
    //   Ok(output_layout)
    // } else {
    //   Err(LayoutError::InsufficientSpace)
    // }

    Err(LayoutError::InsufficientSpace)
  }

  fn layout_internal(&self, parent_size: &Size2D<usize>) -> Result<(LayoutSize, Vec<ColumnLayoutWidth>), LayoutError> {
    // let mut column_layout = Vec::new();
    //
    // let columns = self.columns_ref().unwrap();
    // let data = self.data_ref().unwrap();
    //
    // let mut avail_size = parent_size.clone();
    // let mut col_min_height = 0;
    // let mut col_max_height = 0;
    //
    // for col in 0..columns.len() {
    //   let column = columns.column(col).unwrap();
    //   let column_size = column.layout(&avail_size)?;
    //   let column_width = match column.get_width() {
    //     ColumnWidth::Fixed(fixed) => {
    //       column_layout.push(ColumnLayoutWidth::Fixed(fixed));
    //       MinMax::new(fixed, fixed)
    //     }
    //     ColumnWidth::Dynamic(constraints) => {
    //       let dynamic_width = {
    //         let auto_width = if let Some(auto) = constraints.auto {
    //           match auto {
    //             ColumnWidthAuto::Heading => column_size.max.width,
    //             ColumnWidthAuto::LargestCell => {
    //               let mut max_width = 0;
    //               for row in 0..data.rows_len() {
    //                 if let Some(cell) = data.cell(row, col) {
    //                   let cell_width = cell.layout(&avail_size)?.max.width;
    //                   if cell_width > max_width {
    //                     max_width = cell_width;
    //                   }
    //                 }
    //               }
    //               max_width
    //             }
    //           }
    //         } else {
    //           0
    //         };
    //         let width = match constraints.max {
    //           ColumnWidthValue::Fixed(max) => std::cmp::min(auto_width, max),
    //           ColumnWidthValue::Auto => auto_width,
    //         };
    //         let width = std::cmp::max(width, constraints.min);
    //         width
    //       };
    //
    //       if constraints.flex_weight > 0 {
    //         column_layout.push(ColumnLayoutWidth::Flex {
    //           min: constraints.min,
    //           max: match constraints.max {
    //             ColumnWidthValue::Fixed(max) => max,
    //             ColumnWidthValue::Auto => dynamic_width,
    //           },
    //           weight: constraints.flex_weight,
    //         });
    //         constraints.min
    //       } else {
    //         column_layout.push(ColumnLayoutWidth::Fixed(dynamic_width));
    //         dynamic_width
    //       }
    //     }
    //   };
    //   if !avail_size.contains(Size2D::new(column_width, column_size.height)) {
    //     return Err(LayoutError::InsufficientSpace);
    //   }
    //   avail_size.width -= column_width;
    //   col_min_height = std::cmp::max(col_min_height, column_size.height);
    // }
    //
    // let mut row_used_height = 0;
    // if data.rows_len() > 0 {
    //   let row_avail_size = Size2D::new(parent_size.width, parent_size.height - col_min_height);
    //   for col in 0..columns.len() {
    //     if let Some(cell) = data.cell(0, col) {
    //       let cell_height = cell.layout(&row_avail_size)?.height;
    //       row_used_height = std::cmp::max(row_used_height, cell_height);
    //     }
    //   }
    // }
    //
    // let used_width = parent_size.width - avail_size.width;
    // let used_height = col_min_height + row_used_height;
    // let min_size = Size2D::new(used_width, used_height);
    //
    // if parent_size.contains(min_size.clone()) {
    //   Ok((min_size, column_layout))
    // } else {
    //   Err(LayoutError::InsufficientSpace)
    // }

    Err(LayoutError::InsufficientSpace)
  }
}

enum ColumnLayoutWidth {
  Fixed(usize),
  Flex { min: usize, max: usize, weight: usize },
}

struct MinMax<T> {
  min: T,
  max: T,
}
impl<T> MinMax<T> {
  pub fn new(min: T, max: T) -> Self {
    Self { min, max }
  }
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

  fn layout(&self, parent_size: &Size2D<usize>) -> LayoutResult {
    self.layout_internal(parent_size).map(|ok| ok.0)
  }

  fn render(&self, ctx: &mut RenderCtx) -> RenderResult {
    // let (_, layout) = self
    //   .layout_internal(ctx.get_frame_size())
    //   .map_err(|err| RenderError::Layout(err))?;
    // let flexed_layout = Self::layout_flex(ctx.get_frame_size(), layout).map_err(|err| RenderError::Layout(err))?;
    //
    // let columns = self.columns_ref().unwrap();
    // let mut the_x = 0;
    // for c in 0..columns.len() {
    //   let column = columns.column(c).unwrap();
    //   // TODO: set render context, box constrains
    //   let prev_frame = ctx.get_frame();
    //   let mut child_ctx = ctx.child_ctx(Rect::new(
    //     Point2D::new(ctx.get_frame().min_x() + the_x, ctx.get_frame().min_y()),
    //     Size2D::new(flexed_layout[c], 1),
    //   ));
    //   column.render(&mut child_ctx);
    //   ctx.set_frame(prev_frame);
    //   the_x += flexed_layout[c];
    //   // ctx.renderer.move_to_column_relative((the_x + 1) as u16);
    // }
    //
    // // ctx.renderer.next_line();
    //
    // let data = self.data_ref().unwrap();
    // for r in 0..data.rows_len() {
    //   let mut the_x = 0;
    //   for c in 0..columns.len() {
    //     let column = columns.column(c).unwrap();
    //     if let Some(cell) = data.cell(r, c) {
    //       // TODO: set render context, box constrains
    //       let prev_frame = ctx.get_frame();
    //       let mut child_ctx = ctx.child_ctx(Rect::new(
    //         Point2D::new(
    //           ctx.get_frame().min_x() + the_x,
    //           ctx.get_frame().min_y() + r + 1, /*col*/
    //         ),
    //         Size2D::new(flexed_layout[c], 1),
    //       ));
    //       cell.render(&mut child_ctx);
    //       ctx.set_frame(prev_frame);
    //     }
    //     the_x += flexed_layout[c];
    //     // ctx.renderer.move_to_column_relative((the_x + 1) as u16);
    //   }
    //   // ctx.renderer.next_line();
    // }
    //
    // Ok(())

    Err(RenderError::Layout(LayoutError::InsufficientSpace))
  }
}
