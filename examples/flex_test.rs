use crossterm::style::Stylize;
use witui::widgets::Container;
use witui::widgets::Expand;
use witui::widgets::Flexible;
use witui::widgets::Min;
use witui::Style;
use witui::WiTui;

fn main() {
  #[cfg(feature = "logging")]
  witui::enable_pretty_env_logging();

  let root = Container::new()
    .child(Flexible::loose(0).child(Style::new().on_dark_grey().child("HELLO")))
    .child(Expand::child(
      Min::zero().child(Style::new().on_dark_green().child("World")),
    ))
    .child(Flexible::loose(10).child(Min::zero().child(Style::new().on_dark_blue().child("Welcome"))));

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
