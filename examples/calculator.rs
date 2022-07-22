use witui::widgets::{Borders, CheckBox, Expanded};
use witui::{WiTui, Style};

fn main() {
  let root = Borders::new().borders_rounded(Style::default()).child(Expanded::child(CheckBox::new()));

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
