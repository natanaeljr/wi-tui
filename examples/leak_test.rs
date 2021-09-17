use witui::widgets::Borders;
use witui::widgets::RowView;
use witui::widgets::Expanded;
use witui::widgets::Leak;
use witui::widgets::Min;
use witui::widgets::Repeat;
use witui::{Color, Style, WiTui};

fn main() {
  let root = Borders::with_child(Min::zero().child(Leak::child(
    Borders::with_child(Borders::with_child(()).borders_dash(Style::new().green())).borders_dash(Style::new().blue()),
  )))
  .borders_dash(Style::new().dim());

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
