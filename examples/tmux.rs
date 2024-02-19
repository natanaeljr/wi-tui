use witui::render::RenderCtx;
use witui::widgets::{Table, Text, Widget};
use witui::widgets::table::Column;

fn main() {
  witui::enable_pretty_env_logging();

  let table = Table::new()
    .columns(vec![
      Column::new("DEVICE"),
      Column::new("TYPE"),
      Column::new("STATE"),
      Column::new("CONNECTION"),
    ])
    .data(vec![
      vec!["wlp8s0", "wifi", "connected", "SUPERROUTER"],
      vec!["p2p-dev-wlp8s0", "wifi-p2p", "disconnected", "--"],
      vec!["enp7s0", "ethernet", "unavailable", "--"],
      vec!["lo", "loopback", "unmanaged", "--"],
    ]);

  let root = table;
  let (cols, rows) = crossterm::terminal::size().unwrap();
  let mut render_ctx = RenderCtx::new(false);
  root.render(&render_ctx).unwrap();
  render_ctx.renderer().flush();
}