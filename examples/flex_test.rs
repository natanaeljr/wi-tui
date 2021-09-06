use witui::widgets::container::Container;
use witui::widgets::expand::Expand;
use witui::widgets::flexible::Flexible;
use witui::widgets::style::Style;
use witui::WiTui;

fn main() {
  let root = Container::new()
    .child(Style::new().on_dark_grey().child(Flexible::new(1, Expand::child(()))))
    .child(Style::new().on_dark_green().child(Flexible::new(1, Expand::child(()))))
    .child(Style::new().on_dark_blue().child(Flexible::new(1, Expand::child(()))));

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
