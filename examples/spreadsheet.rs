use witui::util::Scoped;
use witui::widgets::table::column::{Column, ColumnWidth};
use witui::widgets::table::row::Row;
use witui::widgets::table::TableLayout;
use witui::widgets::{Align, Table};
use witui::WiTui;

/// Spreadsheet example to show off the Table Widget.
/// output:
/// ```
///  A  |  B  |  C  |  D  |  E  |  F  |  G  |  H  |  I  |  J  |  K  |  L  |  M  |  N  |  O  |  P  |  Q  |  R  |  S  |  T  |  U  |  V  |  W  |  X  |  Y  |  Z
///     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |
///     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |
///     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |
///     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |
///     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |
///     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |
///     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |
///     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |
///     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |
///     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |     |
/// ```

fn main() {
  let columns = (b'A'..b'Z' + 1)
    .map(|col| Column::new(Align::center(col as char)).width(ColumnWidth::new().min_fixed(5).flex_weight(0)))
    .collect::<Vec<_>>();

  let rows = (1..11).map(|idx| Row::new(idx)).collect::<Vec<Row<usize>>>();

  let data: Vec<Vec<String>> = Vec::new();

  let spreadsheet = Table::new()
    .columns(columns)
    .rows(rows)
    .data(data)
    .layout(TableLayout::default().column_separator('|'));

  WiTui::root_widget(spreadsheet).run_loop().unwrap();
}
