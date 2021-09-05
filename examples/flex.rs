use crossterm::style::Stylize;
use euclid::default::Size2D;
use witui::widgets::borders::Borders;
use witui::widgets::container::Container;
use witui::widgets::leak::Leak;
use witui::widgets::min::Min;
use witui::widgets::padding::Padding;
use witui::widgets::style::{Color, Style};
use witui::widgets::Widget;
use witui::WiTui;

fn main() {
  #[cfg(feature = "logging")]
  witui::enable_pretty_env_logging();

  let root = Container::new()
    .child(Borders::child(()).borders_line(Style::default().dark_blue()))
    .child(Borders::child(()).borders_line(Style::default().dark_yellow()))
    .child(Borders::child("Hello".white()).borders_line(Style::default().green()))
    .child(Borders::child("World".white()).borders_line(Style::default().red()))
    .child(Borders::child("Ready to rock?".white().dim()).borders_line(Style::default().dark_grey()))
    .child(
      Padding::default().top(3).left(16).child(
        Borders::child("This is one true Text-based UI lib".blue().on_dark_grey())
          .borders_double(Style::new().blue().on_dark_grey()),
      ),
    );

  let root = Borders::child(root).borders_line(Style::new().magenta());
  let root = Padding::default().top(1).bottom(1).left(3).right(3).child(root);
  let root = Leak::child(root);
  let root = Min::zero().child(root);
  let root = Borders::child(root).borders_cross(Style::new().yellow());
  let root = Style::new().child(root);

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
