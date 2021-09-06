use crossterm::style::Stylize;
use euclid::size2;
use witui::widgets::align::{Align, HorizontalAlignment, HorizontalSide, VerticalAlignment, VerticalSide};
use witui::widgets::borders::Borders;
use witui::widgets::container::Container;
use witui::widgets::expand::Expand;
use witui::widgets::fillchar::FillChar;
use witui::widgets::flexible::Flexible;
use witui::widgets::hook::Hook;
use witui::widgets::leak::Leak;
use witui::widgets::min::Min;
use witui::widgets::padding::Padding;
use witui::widgets::stack::Stack;
use witui::widgets::style::{Color, Style};
use witui::widgets::Widget;
use witui::WiTui;

// ╭─┤Title├┤Bar├───────────┤_ x├─╮
// │                              │
// │                              │
// │                              │
// │                              │
// │                              │
// │                              │
// ╰──────────────────────────────╯

fn main() {
  let root = Borders::child(Expand::child(()))
    .borders_rounded(Style::default().dark_green())
    .top(Box::new(
      Min::zero().child(
        Stack::new()
          .child(Style::default().dark_green().child(FillChar::new('─')))
          .child(
            Padding::default().left(1).right(1).child(
              Container::new()
                .child(
                  Flexible::tight(6).child(
                    Container::new()
                      .child(Style::new().dark_green().child('┤'))
                      .child(Style::new().dark_green().bold().reverse().child("Title"))
                      .child(Style::new().dark_green().child('├'))
                      .must_fit_all_children(true),
                  ), // Flexible
                ) // Container
                .child(
                  Flexible::tight(1).child(
                    Container::new()
                      .child(Style::new().dark_green().child('┤'))
                      .child(Style::new().white().bold().underlined().dim().child("Bar"))
                      .child(Style::new().dark_green().child('├'))
                      .must_fit_all_children(true),
                  ), // Flexible
                ) // Container
                .child(
                  Flexible::loose(2).child(
                    Align::top_right(
                      Container::new()
                        .child(Style::new().dark_green().child('┤'))
                        .child(
                          Container::new()
                            .child(Min::zero().child(Style::new().white().dim().reverse().bold().child("_")))
                            .child(' ')
                            .child(Style::new().white().bg(Color::AnsiValue(88)).bold().child("x"))
                            .must_fit_all_children(false),
                        )
                        .child(Style::new().dark_green().child('├'))
                        .must_fit_all_children(true),
                    ), // Align
                  ), // Flexible
                ), // Container
            ), // Padding
          ) // Stack
          .must_fit_all_children(false),
      ), // Minimize
    ) as Box<dyn Widget>);

  let root = Min::zero().child(root);
  let root = Style::new()
    .bg(Color::Rgb { r: 20, g: 20, b: 20 })
    .fg(Color::White)
    .child(root);

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
