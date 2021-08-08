use std::any::Any;
use std::borrow::Cow;
use std::cmp::max;
use std::num::NonZeroUsize;
use std::ops::{Deref, DerefMut, Sub, SubAssign};

use crossterm::style::{ContentStyle, StyledContent, Stylize};
use euclid::default::{Point2D, Rect, Size2D};

use crate::render::RenderCtx;
use crate::util::{MinMax, Scoped};
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
  /// Use heading length
  /// TODO: Heading,
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
      auto_mode: ColumnWidthAuto::AllCells, // TODO: default AllCellsOrHeading when implemented
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
// TODO: Scoped<> columns and data
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

  fn layout_flex(
    render_size: &Size2D<usize>, input_layout: Vec<ColumnLayoutFlexInput>,
  ) -> Result<Vec<usize>, LayoutError> {
    let mut final_widths = Vec::new();
    final_widths.reserve(input_layout.len());

    let mut fixed_width = 0;
    let mut flex_total_weight = 0;
    // Compute fixed columns total width and accumulate the weight for flexible columns for later distribution
    for column in input_layout.iter() {
      fixed_width += column.min;
      final_widths.push(column.min);
      flex_total_weight += column.weight;
    }

    // TODO: factor in column_separator

    // Should not be needed, but just double check space for fixed columns
    if !render_size.contains(Size2D::new(fixed_width, 1)) {
      return Err(LayoutError::InsufficientSpace);
    }

    let mut total_width = fixed_width;
    let mut avail_flex_width = render_size.width - fixed_width;
    let mut flex_unit = avail_flex_width as f32 / flex_total_weight as f32;

    // Compute additional width for flexible columns
    for (idx, column) in input_layout.iter().enumerate() {
      // compute actual flex constrained to min/max
      let flex_width = (flex_unit * column.weight as f32).round() as usize;
      let avail_width = column.max - column.min;
      let add_width = std::cmp::min(avail_width, flex_width);
      // recalculate flex unit if there were remainder, to add more for other columns and fit the width
      let flex_width_remainder = flex_width - add_width;
      avail_flex_width -= add_width;
      flex_total_weight -= column.weight;
      flex_unit = avail_flex_width as f32 / flex_total_weight as f32;
      // add width to column minimum to flex it
      final_widths[idx] += add_width;
      total_width += add_width;
    }

    // Finally check again the space and return the final widths
    if render_size.contains(Size2D::new(total_width, 1)) {
      Ok(final_widths)
    } else {
      Err(LayoutError::InsufficientSpace)
    }
  }

  fn layout_table(&self, parent_size: &Size2D<usize>) -> Result<(LayoutSize, Vec<ColumnLayoutFlexInput>), LayoutError> {
    // Initial validation checks
    if self.columns.is_none() {
      return Ok((
        LayoutSize {
          min: Size2D::zero(),
          max: Size2D::zero(),
        },
        vec![],
      ));
    }

    // output container for flex
    let mut column_layouts_flex_input = Vec::new();
    // self fields shorthand
    let columns = self.columns_ref().unwrap();
    // local helpers
    let mut avail_table_size = parent_size.clone();
    let mut table_width = MinMax::default();
    let mut table_headings_height = MinMax::default();

    // Compute column min/max width and height of all columns.
    // After this we should know: 1) if there is enough space; 2) the table_headings_height min/max;
    // 3) how much width the columns used; 4) column values for flex computation.
    for col in 0..columns.len() {
      let column = columns.column(col).unwrap();

      // Get the underlying column layout
      let column_layout_result = column.layout(&avail_table_size);
      if let Err(err) = column_layout_result {
        match err {
          LayoutError::InsufficientSpace => {
            if self.layout.must_render_fit_all_columns {
              return Err(err);
            } else {
              // Do not consider Insufficient Space as error, we are just not going to render the remaining columns
              break;
            }
          }
          _ => return Err(err),
        }
      }
      let column_layout = column_layout_result.unwrap();

      // Figure out min/max column height
      let column_height = if self.layout.hide_headings {
        MinMax::new(0, 0)
      } else {
        MinMax::new(column_layout.min.height, column_layout.max.height)
      };

      // Compute automatic column width value, that should be the max value possible
      let column_width_settings = column.get_width();
      let column_auto_width = match column_width_settings.auto_mode {
        ColumnWidthAuto::Off => 0,
        ColumnWidthAuto::Heading => column_layout.max.width,
        ColumnWidthAuto::AllCells => {
          match self.data_ref() {
            None => 0, // no width if no data
            Some(data) => {
              let mut largest_cell_width = 0;
              for row in 0..data.rows_len() {
                if let Some(cell) = data.cell(row, col) {
                  // Get the underlying cell layout
                  let cell_layout_result = cell.layout(&avail_table_size);
                  if let Err(err) = cell_layout_result {
                    match err {
                      LayoutError::InsufficientSpace => {
                        if self.layout.must_render_fit_all_columns {
                          return Err(err);
                        } else {
                          // Do not consider Insufficient Space as error, we are just not going to render the remaining columns
                          continue;
                        }
                      }
                      _ => return Err(err),
                    }
                  }
                  let cell_layout = cell_layout_result.unwrap();
                  // Check for the largest cell width
                  largest_cell_width = std::cmp::max(largest_cell_width, cell_layout.max.width);
                } // if let Some(cell)
              } // for all rows
              largest_cell_width
            } // Some(data)
          } // match self.data_ref()
        }
        ColumnWidthAuto::VisibleCells => {
          todo!()
        }
        ColumnWidthAuto::AllCellsOrHeading => {
          todo!()
        }
        ColumnWidthAuto::VisibleCellsOrHeading => {
          todo!()
        }
      }; // let column_auto_width = match column_width_settings.auto_mode;

      // Constrain the auto width to the maximum and minimum values
      let column_auto_width = match column_width_settings.max {
        ColumnWidthValue::Fixed(max) => std::cmp::min(column_auto_width, max),
        ColumnWidthValue::Auto => column_auto_width,
      };
      let column_auto_width = match column_width_settings.min {
        ColumnWidthValue::Fixed(min) => std::cmp::max(column_auto_width, min),
        ColumnWidthValue::Auto => column_auto_width,
      };

      // Compute the final minimum and maximum widths for this column
      let mut column_width = MinMax::default();
      column_width.min = match column_width_settings.min {
        ColumnWidthValue::Fixed(min) => min,
        ColumnWidthValue::Auto => column_auto_width,
      };
      column_width.max = match column_width_settings.max {
        ColumnWidthValue::Fixed(max) => max,
        ColumnWidthValue::Auto => column_auto_width,
      };

      // 1) Check if we still have space for minimum column width/height
      if !avail_table_size.contains(Size2D::new(column_width.min, column_height.min)) {
        if self.layout.must_render_fit_all_columns {
          return Err(LayoutError::InsufficientSpace);
        } else {
          // Do not consider Insufficient Space as error, we are just not going to render the remaining columns
          break;
        }
      }

      // 2) Factor the this column height in the min/max table headings height
      table_headings_height.min = std::cmp::max(table_headings_height.min, column_height.min);
      table_headings_height.max = std::cmp::min(table_headings_height.max, column_height.max);

      // 3) Add column width to the overall table width values
      avail_table_size.width -= column_width.min;
      table_width.min += column_width.min;
      table_width.max += column_width.max;

      // 4) Add this column values for the flex calculation
      column_layouts_flex_input.push(ColumnLayoutFlexInput {
        auto: column_auto_width,
        min: column_width.min,
        max: column_width.max,
        weight: column_width_settings.flex_weight,
      });
    } // for all columns

    // We consider the minimum table height to be: { heading + at least 1 row }.
    // So let's compute the first row min/max height or leave a hard space of at least 1 unit.
    let avail_data_height = parent_size.height - table_headings_height.min;
    let mut first_row_height = MinMax::new(1, 1);

    // Compute the min/max height for the first row only
    if let Some(data) = self.data_ref() {
      if data.rows_len() > 1 {
        // Get min height for all *visible* columns
        for (col, column_flex) in column_layouts_flex_input.iter().enumerate() {
          if let Some(cell) = data.cell(0, col) {
            // Get the underlying cell layout
            let cell_avail_size = Size2D::new(column_flex.min, avail_data_height);
            let cell_layout_result = cell.layout(&cell_avail_size);
            if let Err(err) = cell_layout_result {
              match err {
                LayoutError::InsufficientSpace => {
                  if self.layout.must_render_fit_all_columns {
                    return Err(err);
                  } else {
                    // Do not consider Insufficient Space as error, we are just not going to render this cell
                    continue;
                  }
                }
                _ => return Err(err),
              }
            }
            let cell_layout = cell_layout_result.unwrap();
            // Check for the highest/lowest minimum/maximum cell height
            first_row_height.min = std::cmp::max(first_row_height.min, cell_layout.min.height);
            first_row_height.max = std::cmp::min(first_row_height.max, cell_layout.max.height);
          } // if let Some(cell)
        } // for all visible cols
      } // if data.rows_len() > 1
    } // if let Some(data)

    // TODO: compute sizes for all visible rows in order to support row heights > 1 for when rendering with context
    // TODO: constrain max layout in parent scope too

    // Generate final table min/max layout sizes
    let table_height_min = table_headings_height.min + first_row_height.min;
    let table_height_max = table_headings_height.max + first_row_height.max;
    let table_layout_size = LayoutSize {
      min: Size2D::new(table_width.min, table_height_min),
      max: Size2D::new(table_width.max, table_height_max),
    };

    // Finally check if we still have space for the final table size and return result
    if parent_size.contains(table_layout_size.min) {
      Ok((table_layout_size, column_layouts_flex_input))
    } else {
      Err(LayoutError::InsufficientSpace)
    }
  }
}

#[derive(Debug)]
struct ColumnLayoutFlexInput {
  auto: usize,
  min: usize,
  max: usize,
  weight: usize,
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
    self.layout_table(parent_size).map(|ok| ok.0)
  }

  fn render(&self, ctx: &mut RenderCtx) -> RenderResult {
    let (_, layout) = self
      .layout_table(ctx.get_frame_size())
      .map_err(|err| RenderError::Layout(err))?;
    let flexed_widths = Self::layout_flex(ctx.get_frame_size(), layout).map_err(|err| RenderError::Layout(err))?;

    let columns = self.columns_ref().unwrap();
    let mut the_x = 0;
    for c in 0..flexed_widths.len() {
      let column = columns.column(c).unwrap();
      // TODO: set render context, box constrains
      let prev_frame = ctx.get_frame();
      let mut child_ctx = ctx.child_ctx(Rect::new(
        Point2D::new(ctx.get_frame().min_x() + the_x, ctx.get_frame().min_y()),
        Size2D::new(flexed_widths[c], 1),
      ));
      column.render(&mut child_ctx);
      ctx.set_frame(prev_frame);
      the_x += flexed_widths[c];
      // ctx.renderer.move_to_column_relative((the_x + 1) as u16);
    }

    // ctx.renderer.next_line();

    let data = self.data_ref().unwrap();
    for r in 0..data.rows_len() {
      let mut the_x = 0;
      for c in 0..flexed_widths.len() {
        let column = columns.column(c).unwrap();
        if let Some(cell) = data.cell(r, c) {
          // TODO: set render context, box constrains
          let prev_frame = ctx.get_frame();
          let mut child_ctx = ctx.child_ctx(Rect::new(
            Point2D::new(
              ctx.get_frame().min_x() + the_x,
              ctx.get_frame().min_y() + r + 1, /*col*/
            ),
            Size2D::new(flexed_widths[c], 1),
          ));
          cell.render(&mut child_ctx);
          ctx.set_frame(prev_frame);
        }
        the_x += flexed_widths[c];
        // ctx.renderer.move_to_column_relative((the_x + 1) as u16);
      }
      // ctx.renderer.next_line();
    }

    Ok(())
  }
}
