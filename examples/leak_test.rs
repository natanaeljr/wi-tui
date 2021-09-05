use witui::widgets::borders::Borders;
use witui::widgets::container::Container;
use witui::widgets::expand::Expanded;
use witui::widgets::leak::Leak;
use witui::widgets::minimize::Minimize;
use witui::widgets::repeat::Repeat;
use witui::widgets::style::{Color, Style};
use witui::WiTui;

fn main() {
  let root = Borders::child(Minimize::zero().child(Leak::child(
    Borders::child(Borders::child(()).borders_dash(Style::new().green())).borders_dash(Style::new().blue()),
  )))
  .borders_dash(Style::new().dim());

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
