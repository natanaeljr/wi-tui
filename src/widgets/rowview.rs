use std::any::Any;
use std::ops::Deref;

use euclid::default::{Point2D, Rect, Size2D};

use crate::log::info;
use crate::render::RenderCtx;
use crate::util::Scoped;
use crate::widgets::{
  AnyEvent, Capability, EventResult, LayoutError, LayoutResult, LayoutSize, RenderError, RenderResult, Widget,
};
use crate::FlexFit;
use crate::{compute_flex_layout, ChildrenStorage, MinMaxFlex};
use crossterm::style::ContentStyle;
use std::cmp::{max, min};

// TODO: Horizontal/Vertical
// TODO: Must fit N children

pub struct RowView<Children> {
  pub children: Children,
  pub must_fit_all_children: bool,
}

impl RowView<Vec<Box<dyn Widget>>> {
  pub fn new() -> Self {
    Self {
      children: Vec::new(),
      must_fit_all_children: false,
    }
  }

  pub fn child<Child2: Widget + 'static>(mut self, child: Child2) -> Self {
    self.children.push(Box::new(child));
    self
  }
}

impl<Children> RowView<Children>
where
  Children: ChildrenStorage,
{
  pub fn children<Children2: ChildrenStorage>(mut self, children: Children2) -> RowView<Children2> {
    RowView {
      children,
      must_fit_all_children: self.must_fit_all_children,
    }
  }

  pub fn must_fit_all_children(mut self, must_fit_all_children: bool) -> Self {
    self.must_fit_all_children = must_fit_all_children;
    self
  }

  fn layout_impl(&self, total_avail_size: &Size2D<usize>) -> Result<(LayoutSize, Vec<MinMaxFlex>), LayoutError> {
    let mut layout = LayoutSize::default();
    let mut avail_size = total_avail_size.clone();
    let mut flex_input_layouts = Vec::new();

    for idx in 0..self.children.len() {
      let child = self.children.child(idx).unwrap();
      let child_layout_result = child.layout(&avail_size);
      let child_layout = match child_layout_result {
        Ok(layout) => layout,
        Err(LayoutError::InsufficientSpace) => {
          if self.must_fit_all_children {
            return Err(LayoutError::InsufficientSpace);
          } else {
            break; // TODO: Maybe change this to just skip this child?
          }
        }
        Err(e) => return Err(e),
      };
      let child_fixed_size = if child_layout.flex == 0 { child_layout.max } else { child_layout.min };
      if !avail_size.contains(child_fixed_size.clone()) {
        if self.must_fit_all_children {
          return Err(LayoutError::InsufficientSpace);
        } else {
          break; // TODO: Maybe change this to just skip this child?
        }
      }
      // Take this child's size from available size for other children
      avail_size.width -= child_fixed_size.width;
      // Add up the widths
      layout.min.width = layout
        .min
        .width
        .checked_add(child_fixed_size.width)
        .unwrap_or(std::usize::MAX);
      layout.max.width = layout
        .max
        .width
        .checked_add(child_layout.max.width)
        .unwrap_or(std::usize::MAX);
      // Find the highest height
      layout.min.height = layout.min.height.max(child_fixed_size.height);
      layout.max.height = layout.max.height.max(child_layout.max.height);
      // Push layout for later flex computation
      flex_input_layouts.push(MinMaxFlex {
        min: child_layout.min.width,
        max: child_layout.max.width,
        flex: child_layout.flex,
        fit: child_layout.fit.clone(),
      });
    }

    Ok((layout, flex_input_layouts))
  }
}

impl<Children> Widget for RowView<Children>
where
  Children: ChildrenStorage,
{
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    todo!()
  }

  fn layout(&self, total_avail_size: &Size2D<usize>) -> LayoutResult {
    Ok(self.layout_impl(total_avail_size)?.0)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    let frame = ctx.get_frame().clone();
    let (layout, flex_input_layouts) = self.layout_impl(&frame.size).map_err(|e| RenderError::Layout(e))?;
    let (total_width, flexed_widths) =
      compute_flex_layout(frame.size.width, &flex_input_layouts).map_err(|e| RenderError::Layout(e))?;

    let mut walked_width = 0;
    for (idx, child_width) in flexed_widths.iter().enumerate() {
      let child = self.children.child(idx).unwrap();
      let child_frame = Rect::new(
        Point2D::new(frame.min_x() + walked_width, frame.min_y()),
        Size2D::new(*child_width, frame.height()),
      );
      ctx.render_child_dyn_widget(child_frame, child.deref())?;
      walked_width += *child_width;
    }

    Ok(())
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    todo!()
  }
}
