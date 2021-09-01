use witui::widgets::bordered::Bordered;
use witui::widgets::container::Container;
use witui::widgets::style::Style;
use witui::WiTui;

fn main() {
  let root = Container::new()
    .child(Bordered::child(()).borders_line(Style::default().dark_blue()))
    .child(Bordered::child(()).borders_line(Style::default().dark_yellow()))
    .child(Bordered::child(()).borders_line(Style::default().dark_red()));

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
