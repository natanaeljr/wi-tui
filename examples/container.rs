use witui::widgets::bordered::Bordered;
use witui::widgets::container::Container;
use witui::widgets::expanded::Expanded;
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
                  Bordered::child(
                    Container::new()
                      .child(Bordered::child("~").borders_line(Style::new().dark_magenta()))
                      .child(Bordered::child("!").borders_line(Style::new().dark_cyan()))
                      .child(Bordered::child(".").borders_line(Style::new().dark_green())),
                  ) // Bordered
                  .borders_dashes(Style::default()),
                ), // Repeat
              ) // Bordered
              .borders_line(Style::new().dark_red()),
            ), // Container
        ), // Expanded
      ) // Bordered
      .borders_dashes(Style::default()),
    ); // Style

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
