use std::any::Any;
use std::borrow::Cow;
use std::cmp::max;
use std::num::NonZeroUsize;
use std::ops::{Deref, DerefMut, Sub, SubAssign};

use crossterm::style::{ContentStyle, StyledContent, Stylize};
use euclid::default::{Point2D, Rect, Size2D};

use column::{ColumnWidthAuto, ColumnWidthValue, TableColumn, TableColumns};
use data::TableData;
use row::TableRows;

use crate::render::RenderCtx;
use crate::util::{MinMax, Scoped};
use crate::widgets::{LayoutError, LayoutResult, LayoutSize, RenderError, RenderResult, Widget};

pub mod column;
pub mod data;
pub mod row;

pub struct TableLayout {
  // /// Number of fixed left rows
  // TODO: pub fixed_cols: usize,
  // /// Number of fixed top rows
  // TODO: pub fixed_rows: usize,
  /// Whether to render column headings or not
  pub show_column_headings: bool,
  // /// Whether to render row headings or not
  // TODO: pub hide_row_headings: bool,
  /// Column separator
  pub column_separator: char,
  // /// Row separator
  // TODO: pub row_separator: char,
  /// Rendering must fit all columns or render nothing at all (declare insufficient space)
  pub must_render_fit_all_columns: bool,
  // TODO: Global header style (Container Widget?) for applying to the entire header box
  // TODO: Headers, Rows and Column (underlay) Container Widgets for default appliance?
}

impl Default for TableLayout {
  fn default() -> Self {
    Self {
      show_column_headings: true,
      column_separator: ' ',
      must_render_fit_all_columns: false,
    }
  }
}

impl TableLayout {
  pub fn show_column_headings(mut self, show_column_headings: bool) -> Self {
    self.show_column_headings = show_column_headings;
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

// https://api.flutter.dev/flutter/widgets/Table-class.html
// TODO: How to merge cells?
// TODO: How to serialize?
// TODO: ScrollView widget on table, ref: https://api.flutter.dev/flutter/widgets/Table/defaultColumnWidth.html
// TODO: Action properties, ref: https://www.activestate.com/resources/quick-reads/how-to-display-data-in-a-table-using-tkinter/
pub struct Table {
  columns: Option<Box<dyn TableColumns>>,
  rows: Option<Box<dyn TableRows>>,
  data: Option<Box<dyn TableData>>,
  layout: TableLayout,
}

impl Table {
  pub fn new() -> Self {
    Self {
      columns: None,
      rows: None,
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

  pub fn rows<C: TableRows + 'static>(mut self, rows: C) -> Self {
    self.rows = Some(Box::new(rows));
    self
  }

  pub fn rows_ref(&self) -> Option<&dyn TableRows> {
    self.rows.as_ref().and_then(|cols| Some(cols.deref()))
  }

  pub fn rows_mut(&mut self) -> Option<&mut (dyn TableRows + 'static)> {
    self.rows.as_mut().and_then(|cols| Some(cols.deref_mut()))
  }

  pub fn rows_ref_as<C: TableRows + 'static>(&self) -> Option<&C> {
    self
      .rows
      .as_ref()
      .and_then(|cols| cols.deref().as_any().downcast_ref::<C>())
  }

  pub fn rows_mut_as<C: TableRows + 'static>(&mut self) -> Option<&mut C> {
    self
      .rows
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
    &self, render_size: &Size2D<usize>, input_layout: Vec<ColumnLayoutFlexInput>,
  ) -> Result<Vec<usize>, LayoutError> {
    let mut final_widths = Vec::new();
    final_widths.reserve(input_layout.len());

    let mut fixed_width = 0;
    let mut flex_total_weight = 0;
    // Compute fixed columns total width and accumulate the weight for flexible columns for later distribution
    for (col, column) in input_layout.iter().enumerate() {
      // Prepend the column separator, starting from the second column on
      let separator_len = if col > 0 { 1 } else { 0 };
      fixed_width += column.min + separator_len;
      final_widths.push(column.min);
      flex_total_weight += column.weight;
    }

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

  /// Calculate the largest width of cells in a column
  fn layout_column_width_auto_cells(
    &self, col: usize, avail_size: &Size2D<usize>, visible_only: bool,
  ) -> Result<usize, LayoutError> {
    let mut avail_size = avail_size.clone();
    // Remove heading height from cells available height
    if self.layout.show_column_headings {
      // BUG: we are hardcoding the table headings height to 1 here,
      // because at this point we cannot know yet the actual final height
      avail_size.height = avail_size.height.checked_sub(1).unwrap_or(0);
    }
    // Figure out the largest possible width by looping through the rows/cells of this column
    match self.data_ref() {
      None => Ok(0), // no width if no data
      Some(data) => {
        let mut largest_cell_width = 0;
        for row in 0..self.rows.as_ref().map(|rows| rows.len()).unwrap_or(data.rows_len()) {
          // Treatment for VisibleCells variant
          if visible_only && avail_size.height == 0 {
            break;
          }
          if let Some(cell) = data.cell(row, col) {
            // Get the underlying cell layout
            let cell_layout_result = cell.layout(&avail_size);
            if let Err(err) = cell_layout_result {
              match err {
                LayoutError::InsufficientSpace => {
                  if self.layout.must_render_fit_all_columns {
                    return Err(err);
                  } else {
                    // BUG: hardcoded height in 1, because we do not know the final row height
                    if visible_only {
                      avail_size.height = avail_size.height.checked_sub(1).unwrap_or(0);
                    }
                    // Do not consider Insufficient Space as error, we are just not going to render this cell
                    continue;
                  }
                }
                _ => return Err(err),
              }
            }
            let cell_layout = cell_layout_result.unwrap();
            if visible_only {
              avail_size.height = avail_size.height.checked_sub(cell_layout.min.height).unwrap_or(0);
            }
            // Update largest cell width
            largest_cell_width = std::cmp::max(largest_cell_width, cell_layout.max.width);
          } else {
            // BUG: hardcoded height in 1, because we do not know the final row height
            if visible_only {
              avail_size.height = avail_size.height.checked_sub(1).unwrap_or(0);
            }
          }
        } // for all rows
        Ok(largest_cell_width)
      } // Some(data)
    } // match self.data_ref()
  }

  /// Calculate the Auto width of a column based on the configured mode.
  fn layout_column_width_auto(
    &self, col: usize, column_width_auto_mode: &ColumnWidthAuto, column_layout: &LayoutSize, avail_size: &Size2D<usize>,
  ) -> Result<usize, LayoutError> {
    match column_width_auto_mode {
      ColumnWidthAuto::Off => Ok(0),
      ColumnWidthAuto::Heading => Ok(column_layout.max.width),
      ColumnWidthAuto::AllCells => self.layout_column_width_auto_cells(col, &avail_size, false),
      ColumnWidthAuto::VisibleCells => self.layout_column_width_auto_cells(col, &avail_size, true),
      ColumnWidthAuto::AllCellsOrHeading => {
        let cells_width = self.layout_column_width_auto_cells(col, &avail_size, false)?;
        let heading_width = column_layout.max.width;
        Ok(std::cmp::max(cells_width, heading_width))
      }
      ColumnWidthAuto::VisibleCellsOrHeading => {
        let cells_width = self.layout_column_width_auto_cells(col, &avail_size, true)?;
        let heading_width = column_layout.max.width;
        Ok(std::cmp::max(cells_width, heading_width))
      }
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
    let mut table_width = MinMax::<usize>::default();
    let mut table_headings_height = MinMax::<usize>::default();

    // Compute column min/max width and height of all columns.
    // After this we should know: 1) if there is enough space; 2) the table_headings_height min/max;
    // 3) how much width the columns used; 4) column values for flex computation.
    for col in 0..columns.len() {
      let column = columns.column(col).unwrap();

      // Prepend the column separator, starting from the second column on
      let separator_len = if col > 0 { 1 } else { 0 };

      // Check if we still have space for minimum column separator
      if !avail_table_size.contains(Size2D::new(separator_len, 1)) {
        if self.layout.must_render_fit_all_columns {
          return Err(LayoutError::InsufficientSpace);
        } else {
          // Do not consider Insufficient Space as error, we are just not going to render the remaining columns
          break;
        }
      }

      // Update overall table width values with the separator here, so the column width calculation knows the separator took space
      avail_table_size.width = avail_table_size.width.checked_sub(separator_len).unwrap_or(0);
      table_width.min = table_width.min.checked_add(separator_len).unwrap_or(table_width.min);
      table_width.max = table_width.max.checked_add(separator_len).unwrap_or(table_width.max);

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
      let column_height = if self.layout.show_column_headings {
        MinMax::new(column_layout.min.height, column_layout.max.height)
      } else {
        MinMax::new(0, 0)
      };

      // Compute automatic column width value, that should be the max value possible
      let column_width_settings = column.get_width();
      let column_auto_width =
        self.layout_column_width_auto(col, &column_width_settings.auto_mode, &column_layout, &avail_table_size)?;

      // Constrain the auto width to the maximum and minimum values
      let column_auto_width = match column_width_settings.max {
        ColumnWidthValue::Fixed(max) => std::cmp::min(column_auto_width, max),
        ColumnWidthValue::Auto => column_auto_width,
        ColumnWidthValue::Heading => std::cmp::min(column_auto_width, column_layout.max.width),
      };
      let column_auto_width = match column_width_settings.min {
        ColumnWidthValue::Fixed(min) => std::cmp::max(column_auto_width, min),
        ColumnWidthValue::Auto => column_auto_width,
        ColumnWidthValue::Heading => std::cmp::max(column_auto_width, column_layout.max.width),
      };

      // Compute the final minimum and maximum widths for this column
      let mut column_width = MinMax::default();
      column_width.min = match column_width_settings.min {
        ColumnWidthValue::Fixed(min) => min,
        ColumnWidthValue::Auto => column_auto_width,
        ColumnWidthValue::Heading => column_layout.max.width,
      };
      column_width.max = match column_width_settings.max {
        ColumnWidthValue::Fixed(max) => max,
        ColumnWidthValue::Auto => column_auto_width,
        ColumnWidthValue::Heading => column_layout.max.width,
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
      table_width.min = table_width.min.checked_add(column_width.min).unwrap_or(std::usize::MAX);
      table_width.max = table_width.max.checked_add(column_width.max).unwrap_or(std::usize::MAX);

      // 4) Add this column values for the flex calculation
      column_layouts_flex_input.push(ColumnLayoutFlexInput {
        min: column_width.min,
        max: column_width.max,
        weight: column_width_settings.flex_weight,
      });
    } // for all columns

    // We consider the minimum table height to be: { heading + at least 1 row }.
    table_headings_height = if self.layout.show_column_headings {
      table_headings_height
    } else {
      MinMax::default() // zero
    };
    // So let's compute the first row min/max height or leave a hard space of at least 1 unit.
    let avail_data_height = parent_size.height - table_headings_height.min;
    let mut first_row_height = MinMax::new(1, 1);

    // Compute the min/max height for the first row only
    if let Some(data) = self.data_ref() {
      if self.rows.as_ref().map(|rows| rows.len()).unwrap_or(data.rows_len()) > 1 {
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
      } // if rows len > 1
    } // if let Some(data)

    // TODO: compute sizes for all visible rows in order to support row heights > 1 for when rendering with context

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

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let (_, layout) = self
      .layout_table(&ctx.get_frame().size)
      .map_err(|err| RenderError::Layout(err))?;
    let flexed_widths = self
      .layout_flex(&ctx.get_frame().size, layout)
      .map_err(|err| RenderError::Layout(err))?;

    let columns = self.columns_ref().unwrap();

    // render table headings
    let mut column_heading_height = 0;
    if self.layout.show_column_headings {
      column_heading_height = 1 /*TODO: other heights */;
      let mut the_x = 0;
      for col in 0..flexed_widths.len() {
        // factor-in the column separator
        the_x += if col > 0 {
          // let mut child_ctx = ctx.push_ctx(rect);
          // let mut parent_ctx = child_ctx.pop_ctx();

          let mut child_ctx = ctx.into_child_ctx(Rect::new(
            Point2D::new(ctx.get_frame().min_x() + the_x, ctx.get_frame().min_y()),
            Size2D::new(1, 1 /* TODO: height */),
          ));
          ctx.renderer().write(self.layout.column_separator.to_string().as_str());
          1
        } else {
          0
        };
        // render column heading
        let column = columns.column(col).unwrap();
        let mut child_ctx = ctx.into_child_ctx(Rect::new(
          Point2D::new(ctx.get_frame().min_x() + the_x, ctx.get_frame().min_y()),
          Size2D::new(flexed_widths[col], 1 /* TODO: height */),
        ));
        column.render(&mut child_ctx);
        the_x += flexed_widths[col];
        // ctx.renderer.move_to_column_relative((the_x + 1) as u16);
      }
    }

    // ctx.renderer.next_line();

    let data = self.data_ref().unwrap();
    for row in 0..self
      .rows
      .as_ref()
      .map(|rows| rows.len())
      .unwrap_or(data.rows_len())
      .min(ctx.get_frame().size.height - column_heading_height)
    {
      let mut the_x = 0;
      for col in 0..flexed_widths.len() {
        // factor-in the column separator
        the_x += if col > 0 {
          let mut child_ctx = ctx.into_child_ctx(Rect::new(
            Point2D::new(
              ctx.get_frame().min_x() + the_x,
              ctx.get_frame().min_y() + row + column_heading_height,
            ),
            Size2D::new(flexed_widths[col], 1 /* TODO: height */),
          ));
          ctx.renderer().write(self.layout.column_separator.to_string().as_str());
          1
        } else {
          0
        };
        // render cell
        if let Some(cell) = data.cell(row, col) {
          let mut child_ctx = ctx.into_child_ctx(Rect::new(
            Point2D::new(
              ctx.get_frame().min_x() + the_x,
              ctx.get_frame().min_y() + row + column_heading_height,
            ),
            Size2D::new(flexed_widths[col], 1 /* TODO: height */),
          ));
          cell.render(&mut child_ctx);
        }
        the_x += flexed_widths[col];
        // ctx.renderer.move_to_column_relative((the_x + 1) as u16);
      }
      // ctx.renderer.next_line();
    }

    Ok(())
  }
}
