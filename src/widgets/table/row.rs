use std::any::Any;
use std::borrow::Cow;

use euclid::default::Size2D;

use crate::render::RenderCtx;
use crate::util::Scoped;
use crate::widgets::{LayoutResult, RenderResult, Widget};

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
  pub flex_weight: usize,
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
      flex_weight: 1,
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

  pub fn flex_weight(mut self, weight: usize) -> Self {
    self.flex_weight = weight;
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
