extern crate euclid;

use crate::widgets::Widget;

pub mod widgets;
pub mod render;
pub mod util;

pub struct Tui {
  widgets: Vec<Box<dyn Widget>>,
}
