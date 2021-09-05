use euclid::default::Size2D;
use witui::widgets::borders::Borders;
use witui::widgets::container::Container;
use witui::widgets::expand::Expand;
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
      Borders::child(
        Expand::child(
          Container::new()
            .child(Borders::child(()).borders_line(Style::new().dark_blue()))
            .child(Borders::child(()).borders_line(Style::new().dark_yellow()))
            .child(
              Borders::child(
                Repeat::child(
                  Minimize::zero().child(
                    Leak::child(
                      Borders::child(
                        Container::new()
                          .child(Borders::child("~").borders_line(Style::new().dark_magenta()))
                          .child(Borders::child("!").borders_line(Style::new().dark_cyan()))
                          .child(Borders::child(".").borders_line(Style::new().dark_green()))
                          .must_fit_all_children(true),
                      ) // Borders
                      .borders_dash(Style::default()),
                    ), // Leak
                  ), // Minimize
                ), // Repeat
              ) // Borders
              .borders_line(Style::new().dark_red()),
            ), // Container
        ), // Expanded
      ) // Borders
      .borders_rounded(Style::default()),
    ); // Style

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
