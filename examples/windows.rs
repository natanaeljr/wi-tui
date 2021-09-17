use crossterm::style::Stylize;
use euclid::size2;

use witui::widgets::Align;
use witui::widgets::Borders;
use witui::widgets::RowView;
use witui::widgets::Expanded;
use witui::widgets::FillChar;
use witui::widgets::Flexible;
use witui::widgets::Hook;
use witui::widgets::Leak;
use witui::widgets::Min;
use witui::widgets::Padding;
use witui::widgets::Stack;
use witui::widgets::Widget;
use witui::{Color, HorizontalAlignment, HorizontalSide, Style, VerticalAlignment, VerticalSide, WiTui};

// ╭─┤Title├┤Bar├───────────┤_ x├─╮
// │                              │
// │                              │
// │                              │
// │                              │
// │                              │
// │                              │
// ╰──────────────────────────────╯

fn main() {
  let root = Borders::with_child(Expanded::child(()))
    .borders_rounded(Style::default().dark_green())
    .top(Box::new(
      Min::zero().child(
        Stack::new()
          .child(Style::default().dark_green().child(FillChar::new('─')))
          .child(
            Padding::default().left(1).right(1).child(
              RowView::new()
                .child(
                  Flexible::loose(10).child(
                    RowView::new()
                      .child(Style::new().dark_green().child('┤'))
                      .child(
                        Style::new()
                          .white()
                          .on_dark_green()
                          .bold()
                          .child(Flexible::loose(1).child("Title")),
                      ) // Container
                      .child(Style::new().dark_green().child('├'))
                      .must_fit_all_children(true),
                  ), // Flexible
                ) // Container
                .child(
                  Expanded::child(
                    RowView::new()
                      .child(Style::new().dark_green().child('┤'))
                      .child(Style::new().white().bold().underlined().dim().child("Bar"))
                      .child(Style::new().dark_green().child('├'))
                      .must_fit_all_children(true),
                  ), //Expand
                ) // Container
                .child(
                  Flexible::loose(5).child(
                    RowView::new()
                      .child(Style::new().dark_green().child('┤'))
                      .child(
                        RowView::new()
                          .child(Min::zero().child(Style::new().white().dim().reverse().bold().child("_")))
                          .child(' ')
                          .child(Style::new().white().bg(Color::AnsiValue(88)).bold().child("x"))
                          .must_fit_all_children(false),
                      ) // Container
                      .child(Style::new().dark_green().child('├'))
                      .must_fit_all_children(true),
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
