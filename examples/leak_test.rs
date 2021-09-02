use witui::widgets::bordered::Bordered;
use witui::widgets::container::Container;
use witui::widgets::expanded::Expanded;
use witui::widgets::leak::Leak;
use witui::widgets::repeat::Repeat;
use witui::widgets::style::{Color, Style};
use witui::WiTui;

fn main() {
  let root =
    Bordered::child(
      Leak::child(
        Bordered::child(
          Bordered::child(
            ()
          ).borders_dash(Style::new().green())
        ).borders_dash(Style::new().blue())
      )
    ).borders_dash(Style::new().dim());

  WiTui::root_widget(root).alternate(true).run_loop().unwrap();
}
