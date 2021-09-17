use crossterm::style::Stylize;
use witui::widgets::{Expanded, Flexible, Min, RowView};
use witui::Style;
use witui::WiTui;

fn main() {
  #[cfg(feature = "logging")]
  witui::enable_pretty_env_logging();

  let root = RowView::new()
    .child(Flexible::loose(0).child("HELLO".on_dark_grey()))
    .child(Expanded::child(Min::zero().child("World".on_dark_green())))
    .child(Flexible::loose(20).child(Min::zero().child("Welcome".on_dark_blue())));

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
