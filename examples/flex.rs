use crossterm::style::Stylize;
use witui::widgets::borderbox::BorderBox;
use witui::widgets::container::Container;
use witui::widgets::style::{Color, Style};
use witui::widgets::Padding;
use witui::WiTui;

fn main() {
  let mut root = Container::new().children(vec![
    Padding::new(BorderBox::preset_lined("Hello".white(), Style::default().green())),
    Padding::new(BorderBox::preset_lined("World".white(), Style::default().red())),
    Padding::new(BorderBox::preset_lined(
      "Ready to rock?".white().dim(),
      Style::default().dark_grey(),
    )),
    Padding::new(BorderBox::preset_double(
      "This is one true Text-based User Interface library"
        .blue()
        .on_dark_grey(),
      Style::new().blue().on_dark_grey(),
    ))
    .left(16)
    .top(3),
  ]);

  let mut root = BorderBox::preset_lined(root, Style::new().magenta());
  let mut root = Padding::new(root).top(1).bottom(1).left(3).right(3);
  let mut root = BorderBox::preset_dashed(root, Style::new().yellow().slow_blink());
  let mut root = Style::new().apply(root);

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
