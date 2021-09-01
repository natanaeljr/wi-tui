use crossterm::style::Stylize;
use witui::widgets::bordered::Bordered;
use witui::widgets::container::Container;
use witui::widgets::padding::Padding;
use witui::widgets::style::{Color, Style};
use witui::widgets::Widget;
use witui::WiTui;

fn main() {
  let root = Container::new()
    .child(Bordered::child(()).borders_line(Style::default().dark_blue()))
    .child(Bordered::child(()).borders_line(Style::default().dark_yellow()))
    .child(Padding::new(
      Bordered::child("Hello".white()).borders_line(Style::default().green()),
    ))
    .child(Padding::new(
      Bordered::child("World".white()).borders_line(Style::default().red()),
    ))
    .child(Padding::new(
      Bordered::child("Ready to rock?".white().dim()).borders_line(Style::default().dark_grey()),
    ))
    .child(
      Padding::new(
        Bordered::child(
          "This is one true Text-based User Interface library"
            .blue()
            .on_dark_grey(),
        )
        .borders_double(Style::new().blue().on_dark_grey()),
      )
      .left(16)
      .top(3),
    );

  let root = Bordered::child(root).borders_line(Style::new().magenta());
  let root = Padding::new(root).top(1).bottom(1).left(3).right(3);
  let root = Bordered::child(root).borders_cross(Style::new().yellow().slow_blink());
  let root = Style::new().apply(root);

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
