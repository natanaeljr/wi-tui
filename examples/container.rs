use euclid::default::Size2D;

use witui::widgets::Borders;
use witui::widgets::RowView;
use witui::widgets::Expanded;
use witui::widgets::Leak;
use witui::widgets::Min;
use witui::widgets::Repeat;
use witui::{Color, Style, WiTui};

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
      Borders::with_child(
        Expanded::child(
          RowView::new()
            .child(Borders::with_child(()).borders_line(Style::new().dark_blue()))
            .child(Borders::with_child(()).borders_line(Style::new().dark_yellow()))
            .child(
              Borders::with_child(
                Repeat::child(
                  Min::zero().child(
                    Leak::child(
                      Borders::with_child(
                        RowView::new()
                          .child(Borders::with_child("~").borders_line(Style::new().dark_magenta()))
                          .child(Borders::with_child("!").borders_line(Style::new().dark_cyan()))
                          .child(Borders::with_child(".").borders_line(Style::new().dark_green()))
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
