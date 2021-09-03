use euclid::default::Size2D;
use witui::widgets::container::Container;
use witui::widgets::expand::Expanded;
use witui::widgets::frame::Frame;
use witui::widgets::leak::Leak;
use witui::widgets::minimize::Minimize;
use witui::widgets::repeat::Repeat;
use witui::widgets::style::{Color, Style};
use witui::WiTui;

// ┌---------------------------------------┐
// |┌┐┌┐┌─────────────────────────────────┐|
// |└┘└┘│┌---------┐┌---------┐┌---------┐│|
// |    │|┌─┐┌─┐┌─┐||┌─┐┌─┐┌─┐||┌─┐┌─┐┌─┐|│|
// |    │|│~││!││.│||│~││!││.│||│~││!││.│|│|
// |    │|└─┘└─┘└─┘||└─┘└─┘└─┘||└─┘└─┘└─┘|│|
// |    │└---------┘└---------┘└---------┘│|
// |    │┌---------┐┌---------┐┌---------┐│|
// |    │|┌─┐┌─┐┌─┐||┌─┐┌─┐┌─┐||┌─┐┌─┐┌─┐|│|
// |    │|│~││!││.│||│~││!││.│||│~││!││.│|│|
// |    │|└─┘└─┘└─┘||└─┘└─┘└─┘||└─┘└─┘└─┘|│|
// |    │└---------┘└---------┘└---------┘│|
// |    └─────────────────────────────────┘|
// └---------------------------------------┘

fn main() {
  #[cfg(feature = "logging")]
  witui::enable_pretty_env_logging();

  let root = Style::new()
    .bg(Color::Rgb { r: 20, g: 20, b: 20 })
    .fg(Color::White)
    .child(
      Frame::child(
        Expanded::child(
          Container::new()
            .child(Frame::child(()).borders_line(Style::new().dark_blue()))
            .child(Frame::child(()).borders_line(Style::new().dark_yellow()))
            .child(
              Frame::child(
                Repeat::child(
                  Minimize::zero().child(
                    Leak::child(
                      Frame::child(
                        Container::new()
                          .child(Frame::child("~").borders_line(Style::new().dark_magenta()))
                          .child(Frame::child("!").borders_line(Style::new().dark_cyan()))
                          .child(Frame::child(".").borders_line(Style::new().dark_green()))
                          .must_fit_all_children(true),
                      ) // Frame
                      .borders_dash(Style::default()),
                    ), // Leak
                  ), // Minimize
                ), // Repeat
              ) // Frame
              .borders_line(Style::new().dark_red()),
            ), // Container
        ), // Expanded
      ) // Frame
      .borders_line(Style::default()),
    ); // Style

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
