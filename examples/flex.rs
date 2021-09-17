use crossterm::style::Stylize;
use euclid::default::Size2D;

use witui::widgets::Borders;
use witui::widgets::Row;
use witui::widgets::Leak;
use witui::widgets::Min;
use witui::widgets::Padding;
use witui::widgets::Widget;
use witui::Color;
use witui::{Style, WiTui};

fn main() {
  #[cfg(feature = "logging")]
  witui::enable_pretty_env_logging();

  let root = Row::new()
    .child(Borders::with_child(()).borders_line(Style::default().dark_blue()))
    .child(Borders::with_child(()).borders_line(Style::default().dark_yellow()))
    .child(Borders::with_child(Style::new().white().child("Hello")).borders_line(Style::default().green()))
    .child(Borders::with_child(Style::new().white().child("World")).borders_line(Style::default().red()))
    .child(
      Borders::with_child(Style::new().white().dim().child("Ready to rock?"))
        .borders_line(Style::default().dark_grey()),
    )
    .child(
      Padding::default().top(3).left(16).child(
        Borders::with_child(
          Style::new()
            .blue()
            .on_dark_grey()
            .child("This is one true Text-based UI lib"),
        )
        .borders_double(Style::new().blue().on_dark_grey()),
      ),
    );

  let root = Borders::with_child(root).borders_line(Style::new().magenta());
  let root = Padding::default().top(1).bottom(1).left(3).right(3).child(root);
  let root = Leak::child(root);
  let root = Min::zero().child(root);
  let root = Borders::with_child(root).borders_cross(Style::new().yellow());
  let root = Style::new().child(root);

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
