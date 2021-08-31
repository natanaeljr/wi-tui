use std::io::Read;
use std::num::NonZeroUsize;
use std::ops::{Deref, DerefMut};

use euclid::default::{Rect, Size2D};

use witui::render::{RenderCtx, Renderer};
use witui::widgets::table::{Column, ColumnWidth};
use witui::widgets::{Align, Button, Padding, Table, Widget};

fn main() {
  let button = Button::new("Button");

  let data = vec![
    vec!["A1", "B2", "C1"], //
    vec!["A2", "B2", "C2"], //
    vec!["A3", "B3", "C3"], //
  ];

  let table = Table::new().data(data);

  let aligned_data = vec![vec![
    Align::center("Hello"),
    Align::center("World"),
    Align::center("Bye"),
    Align::center("Heaven"),
  ]];

  let columns = vec![
    Column::new(Align::center("A")),
    Column::new(Align::center("B")),
    Column::new(Align::center("C")),
    Column::new(Align::center("D")),
  ];

  let table = Table::new().columns(columns).data(aligned_data);

  let table = Table::new()
    .columns(vec![
      Column::new("A"), //
      Column::new("B"), //
      Column::new("C"), //
    ])
    .data(vec![
      vec!["A1", "B2", "C1"],     //
      vec!["A2", "B2", "C2"],     //
      vec!["A3", "B3", "C3"],     //
      vec!["A4", "B44444", "C4"], //
      vec!["A5", "B5", "C5"],     //
      vec!["A6", "B6", "C6"],     //
      vec!["A72", "B7", "C7"],    //
    ]);
  let table = Padding::new(table).left(5).top(0);
  // table.render(&mut render_ctx).unwrap();

  let table: Table = Table::new()
    .columns(vec![
      Column::new(Box::new("Hi") as Box<dyn Widget>).width(ColumnWidth::new().max_fixed(3)),
      Column::new(Box::new(Align::center("Hey")) as Box<dyn Widget>), //
    ])
    .data(vec![vec![
      Box::new("Bye") as Box<dyn Widget>,
      Box::new("SeeYou!".to_string()) as Box<dyn Widget>,
    ]]);
  let mut table = table;
  let mut data = table.data_mut_as::<Vec<Vec<Box<dyn Widget>>>>().unwrap();
  data[0][1] = Box::new("Tschuss");

  let table = Padding::new(table).left(20).top(2);
  // table.render(&mut render_ctx).unwrap();
  let table = table.child;

  let table = table;
  let columns = table.columns_ref().unwrap();
  let columns = table.columns_ref_as::<Vec<Column<Box<dyn Widget>>>>().unwrap();
  let data = table.data_ref().unwrap();
  let data = table.data_ref_as::<Vec<Vec<Box<dyn Widget>>>>().unwrap();

  let mut table = table;
  let mut columns = table
    .columns_mut()
    .unwrap()
    .as_any_mut()
    .downcast_mut::<Vec<Column<Box<dyn Widget>>>>()
    .unwrap();
  let mut columns = table.columns_mut_as::<Vec<Column<Box<dyn Widget>>>>().unwrap();
  let mut data = table
    .data_mut()
    .unwrap()
    .as_any_mut()
    .downcast_mut::<Vec<Vec<Box<dyn Widget>>>>()
    .unwrap();
  let mut data = table.data_mut_as::<Vec<Vec<Box<dyn Widget>>>>().unwrap();

  // // $ nmcli d
  // // DEVICE          TYPE      STATE         CONNECTION
  // // wlp8s0          wifi      connected     SUPERROUTER
  // // p2p-dev-wlp8s0  wifi-p2p  disconnected  --
  // // enp7s0          ethernet  unavailable   --
  // // lo              loopback  unmanaged     --
  let table = Table::new()
    .columns(vec![
      Column::new("DEVICE"),     //
      Column::new("TYPE"),       //
      Column::new("STATE"),      //
      Column::new("CONNECTION"), //
    ])
    .data(vec![
      vec!["wlp8s0", "wifi", "connected", "SUPERROUTER"],       //
      vec!["p2p-dev-wlp8s0", "wifi-p2p", "disconnected", "--"], //
      vec!["enp7s0", "ethernet", "unavailable", "--"],          //
      vec!["lo", "loopback", "unmanaged", "--"],                //
    ])
    // .number_rows()
    ;

  let table = Padding::new(table).left(0).top(10);
  // table.render(&mut render_ctx).unwrap();
}
