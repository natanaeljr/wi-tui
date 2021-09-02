use witui::widgets::bordered::Bordered;
use witui::widgets::container::Container;
use witui::widgets::expanded::Expanded;
use witui::widgets::repeat::Repeat;
use witui::widgets::style::{Color, Style};
use witui::WiTui;
use witui::widgets::leak::Leak;

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
  let root = Style::new()
    .bg(Color::Rgb { r: 20, g: 20, b: 20 })
    .fg(Color::White)
    .child(
      Bordered::child(
        Expanded::child(
          Container::new()
            .child(Bordered::child(()).borders_line(Style::new().dark_blue()))
            .child(Bordered::child(()).borders_line(Style::new().dark_yellow()))
            .child(
              Bordered::child(
                Repeat::child(
                  Leak::child(
                  Bordered::child(
                    Container::new()
                      .child(Bordered::child("~").borders_line(Style::new().dark_magenta()))
                      .child(Bordered::child("!").borders_line(Style::new().dark_cyan()))
                      .child(Bordered::child(".").borders_line(Style::new().dark_green()))
                      .must_fit_all_children(true),
                  ) // Bordered
                  .borders_dash(Style::default()),
                  ) // Leak
                ), // Repeat
              ) // Bordered
              .borders_line(Style::new().dark_red()),
            ), // Container
        ), // Expanded
      ) // Bordered
      .borders_dash(Style::default()),
    ); // Style

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
