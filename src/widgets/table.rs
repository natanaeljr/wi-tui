use std::any::Any;
use std::borrow::Cow;
use std::ops::{Deref, DerefMut, Sub, SubAssign};

use crossterm::style::{Attribute, Attributes, Color, ContentStyle, StyledContent, Stylize};
use euclid::default::{Point2D, Rect, Size2D};

use crate::render::RenderCtx;
use crate::util::{MinMax, Scoped, ScopedMut};
use crate::widgets::flexible::FlexFit;
use crate::widgets::{AnyEvent, EventResult, LayoutError, LayoutResult, LayoutSize, RenderError, RenderResult, Widget};
use crossterm::event::{Event, MouseButton, MouseEventKind};

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
  Heading,
}

/// Combinations for "abcdefghijklmnopqrswxyz"
///  Fixed:           { min: 10, max: 10,      auto: None,        flex: 0 } => "abcdefghij"
///  Min:             { min: 5,  max: 10,      auto: None,        flex: 0 } => "abcde"
///  Distributed:     { min: 5,  max: 10|Auto, auto: None,        flex: 1 } => "abcdef...j"
///  AutoFixed:       { min: 1,  max: Auto,    auto: LargestCell, flex: 0 } => "abcdefghijklmnopqrswxyz"
///  AutoFlexed:      { min: 1,  max: Auto,    auto: LargestCell, flex: 1 } => "a..z"
///  AutoFlexedBeyond:{ min: 1,  max: Inf,     auto: LargestCell, flex: 1 } => "abc... "
#[derive(Clone)]
pub struct ColumnWidth {
  /// Minimum column width
  pub min: ColumnWidthValue,
  /// Maximum column width
  pub max: ColumnWidthValue,
  /// Automatic column width adjustment
  pub auto_mode: ColumnWidthAuto,
  /// Flex column width to available space based on weight; zero is no flex.
  pub flex: usize,
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
      flex: 1,
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

  pub fn min_heading(mut self) -> Self {
    self.min = ColumnWidthValue::Heading;
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

  pub fn max_heading(mut self) -> Self {
    self.max = ColumnWidthValue::Heading;
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

  pub fn flex(mut self, weight: usize) -> Self {
    self.flex = weight;
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
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    match event {
      AnyEvent::Input(input) => match input {
        Event::Key(_) => EventResult::Unhandled,
        Event::Mouse(mouse) => {
          if mouse.modifiers.is_empty() {
            match mouse.kind {
              MouseEventKind::Down(button) => match button {
                MouseButton::Left => EventResult::Unhandled,
                MouseButton::Right => EventResult::Unhandled,
                MouseButton::Middle => {
                  self.width.max = match self.width.max {
                    ColumnWidthValue::Fixed(_) => ColumnWidthValue::Auto,
                    ColumnWidthValue::Auto => ColumnWidthValue::Fixed(1),
                    ColumnWidthValue::Heading => ColumnWidthValue::Fixed(1),
                  };
                  EventResult::Done
                }
              },
              _ => EventResult::Unhandled,
            }
          } else {
            EventResult::Unhandled
          }
        }
        Event::Resize(_, _) => EventResult::Unhandled,
      },
    }
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    self.heading.layout(avail_size)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    // ctx.renderer().set_background(&Color::Grey);
    // ctx.renderer().set_foreground(&Color::Black);
    // ctx
    //   .renderer()
    //   .set_attributes(Attributes::default() | Attribute::Reverse);
    self.heading.render(ctx)
  }
}

pub trait TableColumn: Widget {
  fn get_width(&self) -> Cow<ColumnWidth>;
  fn as_widget(&self) -> &dyn Widget;
  fn as_mut_widget(&mut self) -> &mut dyn Widget;
}

impl<Heading> TableColumn for Column<Heading>
where
  Heading: Widget,
{
  fn get_width(&self) -> Cow<ColumnWidth> {
    Cow::Borrowed(&self.width)
  }

  fn as_widget(&self) -> &dyn Widget {
    self
  }

  fn as_mut_widget(&mut self) -> &mut dyn Widget {
    self
  }
}

pub trait TableColumns: 'static {
  fn len(&self) -> usize;
  fn column(&self, idx: usize) -> Option<Scoped<dyn TableColumn>>;
  fn column_mut(&mut self, idx: usize) -> Option<ScopedMut<dyn TableColumn>>;
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

  fn column_mut(&mut self, idx: usize) -> Option<ScopedMut<dyn TableColumn>> {
    self
      .get_mut(idx)
      .and_then(|col| Some(ScopedMut::Ref(col as &mut dyn TableColumn)))
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}

pub struct TableColumnsFn {
  pub len: usize,
  pub fn_column: Box<dyn Fn(usize) -> Option<Scoped<'static, dyn TableColumn>>>,
}

impl TableColumnsFn {
  pub fn generator<F: 'static>(len: usize, fn_column: F) -> Self
  where
    F: Fn(usize) -> Option<Scoped<'static, dyn TableColumn>>,
  {
    Self {
      len,
      fn_column: Box::new(fn_column),
    }
  }
}

impl TableColumns for TableColumnsFn {
  fn len(&self) -> usize {
    self.len
  }

  fn column(&self, idx: usize) -> Option<Scoped<dyn TableColumn>> {
    (self.fn_column)(idx)
  }

  fn column_mut(&mut self, idx: usize) -> Option<ScopedMut<dyn TableColumn>> {
    todo!()
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}

#[derive(Clone)]
pub enum RowHeightAuto {
  /// Do not adjust column width automatically
  Off,
  /// Adjust column width to the width of the largest cell in that column
  Cells,
  /// Adjust column width to the width of the heading
  Heading,
  /// Adjust column width to the width of the largest cell in that column or the heading
  CellsOrHeading,
}

#[derive(Clone)]
pub enum RowHeightValue {
  /// Use a fixed value
  Fixed(usize),
  /// Use value computed from the auto setting
  Auto,
  /// Use heading height
  Heading,
}

#[derive(Clone)]
pub struct RowHeight {
  /// Minimum row height
  pub min: RowHeightValue,
  /// Maximum row height
  pub max: RowHeightValue,
  /// Automatic row height adjustment
  pub auto_mode: RowHeightAuto,
  /// Flex row height to available space based on weight; zero is no flex.
  pub flex: usize,
}

impl Default for RowHeight {
  fn default() -> Self {
    Self::new()
  }
}

impl RowHeight {
  pub fn new() -> Self {
    Self {
      min: RowHeightValue::Fixed(1),
      max: RowHeightValue::Fixed(1),
      auto_mode: RowHeightAuto::CellsOrHeading,
      flex: 1,
    }
  }

  pub fn min(mut self, min: RowHeightValue) -> Self {
    self.min = min;
    self
  }

  pub fn min_auto(mut self) -> Self {
    self.min = RowHeightValue::Auto;
    self
  }

  pub fn min_heading(mut self) -> Self {
    self.min = RowHeightValue::Heading;
    self
  }

  pub fn min_fixed(mut self, min: usize) -> Self {
    self.min = RowHeightValue::Fixed(min);
    self
  }

  pub fn max(mut self, max: RowHeightValue) -> Self {
    self.max = max;
    self
  }

  pub fn max_heading(mut self) -> Self {
    self.max = RowHeightValue::Heading;
    self
  }

  pub fn max_fixed(mut self, fixed: usize) -> Self {
    self.max = RowHeightValue::Fixed(fixed);
    self
  }

  pub fn max_auto(mut self) -> Self {
    self.max = RowHeightValue::Auto;
    self
  }

  pub fn auto_mode(mut self, auto_mode: RowHeightAuto) -> Self {
    self.auto_mode = auto_mode;
    self
  }

  pub fn flex(mut self, weight: usize) -> Self {
    self.flex = weight;
    self
  }
}

pub struct Row<Heading> {
  pub heading: Heading,
  pub height: RowHeight,
}

impl<Heading> Row<Heading>
where
  Heading: Widget,
{
  pub fn new(heading: Heading) -> Self {
    Self {
      heading,
      height: RowHeight::default(),
    }
  }
  pub fn height(mut self, height: RowHeight) -> Self {
    self.height = height;
    self
  }
}

impl<Heading> Widget for Row<Heading>
where
  Heading: Widget,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    todo!()
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    self.heading.layout(avail_size)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    self.heading.render(ctx)
  }
}

pub trait TableRow: Widget {
  fn get_height(&self) -> Cow<RowHeight>;
}

impl<Heading> TableRow for Row<Heading>
where
  Heading: Widget,
{
  fn get_height(&self) -> Cow<RowHeight> {
    Cow::Borrowed(&self.height)
  }
}

pub trait TableRows: 'static {
  fn len(&self) -> usize;
  fn row(&self, idx: usize) -> Option<Scoped<dyn TableRow>>;
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<R> TableRows for Vec<R>
where
  R: TableRow + 'static,
{
  fn len(&self) -> usize {
    Self::len(self)
  }

  fn row(&self, idx: usize) -> Option<Scoped<dyn TableRow>> {
    self.get(idx).and_then(|row| Some(Scoped::Ref(row as &dyn TableRow)))
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}

pub struct TableRowsFn {
  pub len: usize,
  pub fn_row: Box<dyn Fn(usize) -> Option<Scoped<'static, dyn TableRow>>>,
}

impl TableRowsFn {
  pub fn generator<F: 'static>(len: usize, fn_row: F) -> Self
  where
    F: Fn(usize) -> Option<Scoped<'static, dyn TableRow>>,
  {
    Self {
      len,
      fn_row: Box::new(fn_row),
    }
  }
}

impl TableRows for TableRowsFn {
  fn len(&self) -> usize {
    self.len
  }

  fn row(&self, idx: usize) -> Option<Scoped<dyn TableRow>> {
    (self.fn_row)(idx)
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

  fn layout_table(&self, avail_size: &Size2D<usize>) -> Result<(LayoutSize, Vec<ColumnLayoutFlexInput>), LayoutError> {
    // Initial validation checks
    if self.columns.is_none() {
      return Ok((LayoutSize::min_max(Size2D::zero(), Size2D::zero()), vec![]));
    }

    // output container for flex
    let mut column_layouts_flex_input = Vec::new();
    // self fields shorthand
    let columns = self.columns_ref().unwrap();
    // local helpers
    let mut avail_table_size = avail_size.clone();
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
        weight: column_width_settings.flex,
      });
    } // for all columns

    // We consider the minimum table height to be: { heading + at least 1 row }.
    table_headings_height = if self.layout.show_column_headings {
      table_headings_height
    } else {
      MinMax::default() // zero
    };
    // So let's compute the first row min/max height or leave a hard space of at least 1 unit.
    let avail_data_height = avail_size.height - table_headings_height.min;
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
    let table_height_max = table_headings_height.max
      + first_row_height.max /*TODO: compute actual rows size */* self.data.as_ref().unwrap().rows_len()
      + 1;
    let table_layout_size = LayoutSize::min_max(
      Size2D::new(table_width.min, table_height_min),
      Size2D::new(table_width.max, table_height_max),
    );

    // Finally check if we still have space for the final table size and return result
    if avail_size.contains(table_layout_size.min) {
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
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    // NOTE: pass down to the column, make it possible for the column to spawn a Popup Menu with filled options,
    // as we go back up the hierarchy the Popup can be filled up.

    let (_, layout) = self.layout_table(size).unwrap();
    let flexed_widths = self.layout_flex(size, layout).unwrap();
    let mut column_starts = Vec::new();
    for (idx, width) in flexed_widths.iter().enumerate() {
      let prev = *column_starts.get(idx.checked_sub(1).unwrap_or(0)).unwrap_or(&0);
      let new = prev + 1 + *width;
      column_starts.push(new);
    }

    match event {
      AnyEvent::Input(input) => match input {
        Event::Key(_) => EventResult::Unhandled,
        Event::Mouse(mouse) => {
          let mut col = None;
          for idx in 0..column_starts.len() {
            if mouse.column >= *column_starts.get(idx.checked_sub(1).unwrap_or(0)).unwrap_or(&0) as u16
              && mouse.column < *column_starts.get(idx).unwrap_or(&std::usize::MAX) as u16
            {
              col = Some(idx);
              break;
            }
          }

          if mouse.modifiers.is_empty() {
            match mouse.kind {
              MouseEventKind::Down(button) => match button {
                MouseButton::Left => EventResult::Unhandled,
                MouseButton::Right => EventResult::Unhandled,
                MouseButton::Middle => {
                  if let Some(col) = col {
                    let mut column = self.columns.as_mut().unwrap().column_mut(col).unwrap();
                    column.as_mut_widget().event(event, size);
                  }
                  EventResult::Done
                }
              },
              _ => EventResult::Unhandled,
            }
          } else {
            EventResult::Unhandled
          }
        }
        Event::Resize(_, _) => EventResult::Unhandled,
      },
    }
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutResult {
    self.layout_table(avail_size).map(|ok| ok.0)
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
          let child_frame = Rect::new(
            Point2D::new(ctx.get_frame().min_x() + the_x, ctx.get_frame().min_y()),
            Size2D::new(1, 1 /* TODO: height */),
          );
          ctx.render_child_widget(child_frame, &self.layout.column_separator);
          1
        } else {
          0
        };
        // render column heading
        let column = columns.column(col).unwrap();
        let child_frame = Rect::new(
          Point2D::new(ctx.get_frame().min_x() + the_x, ctx.get_frame().min_y()),
          Size2D::new(flexed_widths[col], 1 /* TODO: height */),
        );
        // ctx.renderer().set_background(&Color::Black);
        ctx.render_child_dyn_widget(child_frame, column.as_widget());
        the_x += flexed_widths[col];
      }
    }

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
          let child_frame = Rect::new(
            Point2D::new(
              ctx.get_frame().min_x() + the_x,
              ctx.get_frame().min_y() + row + column_heading_height,
            ),
            Size2D::new(flexed_widths[col], 1 /* TODO: height */),
          );
          ctx.render_child_widget(child_frame, &self.layout.column_separator);
          1
        } else {
          0
        };
        // render cell
        if let Some(cell) = data.cell(row, col) {
          let child_frame = Rect::new(
            Point2D::new(
              ctx.get_frame().min_x() + the_x,
              ctx.get_frame().min_y() + row + column_heading_height,
            ),
            Size2D::new(flexed_widths[col], 1 /* TODO: height */),
          );
          ctx.render_child_dyn_widget(child_frame, cell.deref());
        }
        the_x += flexed_widths[col];
        // ctx.renderer.move_to_column_relative((the_x + 1) as u16);
      }
      // ctx.renderer.next_line();
    }

    Ok(())
  }
}
