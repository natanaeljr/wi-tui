use crate::widgets::Widget;

pub mod widgets;
pub mod render;
pub mod rect;

struct Tui {
  widgets: Vec<Box<dyn Widget>>,
}
