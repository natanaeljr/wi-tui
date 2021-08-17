use std::any::Any;
use std::borrow::Cow;

use euclid::default::Size2D;

use crate::render::RenderCtx;
use crate::util::Scoped;
use crate::widgets::{LayoutResult, RenderResult, Widget};

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

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    self.heading.render(ctx)
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

pub trait TableColumns: 'static {
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

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}
