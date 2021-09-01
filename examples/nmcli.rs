use witui::widgets::table::{Column, Table};
use witui::WiTui;

/// Example to demonstrate the simple use of the Table Widget.
/// This reproduces the output of nmcli (a linux utility for network management).
/// output:
///
/// DEVICE          TYPE      STATE         CONNECTION
/// wlp8s0          wifi      connected     SuperRouter
/// p2p-dev-wlp8s0  wifi-p2p  disconnected  --
/// enp7s0          ethernet  unavailable   --
/// lo              loopback  unmanaged     --

fn main() {
  let table = Table::new()
    .columns(vec![
      Column::new("DEVICE"),
      Column::new("TYPE"),
      Column::new("STATE"),
      Column::new("CONNECTION"),
    ])
    .data(vec![
      vec!["wlp8s0", "wifi", "connected", "SuperRouter"],
      vec!["p2p-dev-wlp8s0", "wifi-p2p", "disconnected", "--"],
      vec!["enp7s0", "ethernet", "unavailable", "--"],
      vec!["lo", "loopback", "unmanaged", "--"],
    ]);

  let mut tui = WiTui::root_widget(table);
  tui.print().unwrap();
}
