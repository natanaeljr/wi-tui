use crossterm::style::Stylize;
use euclid::default::Size2D;
use witui::widgets::container::Container;
use witui::widgets::frame::Frame;
use witui::widgets::leak::Leak;
use witui::widgets::minimize::Minimize;
use witui::widgets::padding::Padding;
use witui::widgets::style::{Color, Style};
use witui::widgets::Widget;
use witui::WiTui;

fn main() {
  #[cfg(feature = "logging")]
  witui::enable_pretty_env_logging();

  let root = Container::new()
    .child(Frame::child(()).borders_line(Style::default().dark_blue()))
    .child(Frame::child(()).borders_line(Style::default().dark_yellow()))
    .child(Frame::child("Hello".white()).borders_line(Style::default().green()))
    .child(Frame::child("World".white()).borders_line(Style::default().red()))
    .child(Frame::child("Ready to rock?".white().dim()).borders_line(Style::default().dark_grey()))
    .child(
      Padding::child(
        Frame::child("This is one true Text-based UI lib".blue().on_dark_grey())
          .borders_double(Style::new().blue().on_dark_grey()),
      )
      .left(16)
      .top(3),
    );

  let root = Frame::child(root).borders_line(Style::new().magenta());
  let root = Padding::child(root).top(1).bottom(1).left(3).right(3);
  let root = Leak::child(root);
  let root = Minimize::zero().child(root);
  let root = Frame::child(root).borders_cross(Style::new().yellow());
  let root = Style::new().child(root);

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
