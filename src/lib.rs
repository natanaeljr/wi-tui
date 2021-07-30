use crate::widgets::Widget;

pub mod widgets;
pub mod render;

struct Tui {
  widgets: Vec<Box<dyn Widget>>,
}
