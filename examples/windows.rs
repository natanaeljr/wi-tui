use crossterm::style::Stylize;
use witui::widgets::align::{Align, HorizontalAlignment, HorizontalSide, VerticalAlignment, VerticalSide};
use witui::widgets::borders::Borders;
use witui::widgets::expand::Expand;
use witui::widgets::minimize::Minimize;
use witui::widgets::padding::Padding;
use witui::widgets::style::Style;
use witui::WiTui;

fn main() {
  let root = Borders::child(Expand::child(
    Minimize::zero().child(Align::center("Hello World!".white().bold())),
  ))
  .borders_rounded(Style::default());

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
