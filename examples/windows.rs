use crossterm::style::Stylize;
use witui::widgets::align::{Align, HorizontalAlignment, HorizontalSide, VerticalAlignment, VerticalSide};
use witui::widgets::borders::Borders;
use witui::widgets::container::Container;
use witui::widgets::expand::Expand;
use witui::widgets::fillchar::FillChar;
use witui::widgets::minimize::Minimize;
use witui::widgets::padding::Padding;
use witui::widgets::stack::Stack;
use witui::widgets::style::Style;
use witui::widgets::Widget;
use witui::WiTui;

fn main() {
  let root = Borders::child(Expand::child(()))
    .borders_rounded(Style::default().dark_green())
    .top(Box::new(
      Stack::new()
        .child(Style::default().dark_green().child(FillChar::new('─')))
        .child(
          Container::new().child(
            Expand::child(
              Padding::default().left(1).right(1).child(
                Align::top_left(
                  Container::new()
                    .child('┤'.dark_green().dim())
                    .child("Title".white().bold().underlined())
                    .child('├'.dark_green().dim())
                    .must_fit_all_children(true),
                ), // Align
              ), // Padding
            ), // Expand
          ), // Container
        ), // Stack
    ) as Box<dyn Widget>);

  let root = Minimize::zero().child(root);

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
