use crossterm::style::Stylize;
use witui::widgets::align::{Align, HorizontalAlignment, HorizontalSide, VerticalAlignment, VerticalSide};
use witui::widgets::borders::Borders;
use witui::widgets::container::Container;
use witui::widgets::expand::Expand;
use witui::widgets::fillchar::FillChar;
use witui::widgets::hook::Hook;
use witui::widgets::leak::Leak;
use witui::widgets::minimize::Minimize;
use witui::widgets::padding::Padding;
use witui::widgets::stack::Stack;
use witui::widgets::style::{Color, Style};
use witui::widgets::Widget;
use witui::WiTui;

fn main() {
  let root = Borders::child(Expand::child(()))
    .borders_rounded(Style::default().dark_green())
    .top(Box::new(
      Minimize::zero().child(
        Stack::new()
          .child(Style::default().dark_green().child(FillChar::new('─')))
          .child(
            Container::new()
              .child(
                Padding::default().left(1).child(
                  Align::top_left(
                    Container::new()
                      .child('┤'.dark_green())
                      .child("Title".white().bold().underlined().dim())
                      .child('├'.dark_green())
                      .must_fit_all_children(true),
                  ), // Align
                ), // Padding
              ) // Container
              .child(
                Align::top_left(
                  Container::new()
                    .child('┤'.dark_green())
                    .child("Bread".white().bold().underlined().dim())
                    .child('├'.dark_green())
                    .must_fit_all_children(true),
                ), // Align
              ) // Container
              .child(
                Expand::child(
                  Padding::default().right(1).child(
                    Align::top_right(
                      Container::new()
                        .child('┤'.dark_green())
                        .child("Header".white().bold().underlined().dim())
                        .child('├'.dark_green())
                        .must_fit_all_children(true),
                    ), // Align
                  ), // Padding
                ), // Expand
              ) // Container
              .must_fit_all_children(false),
          ), // Stack
      ), // Minimize
    ) as Box<dyn Widget>);

  let root = Minimize::zero().child(root);
  let root = Style::new()
    .bg(Color::Rgb { r: 20, g: 20, b: 20 })
    .fg(Color::White)
    .child(root);

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
