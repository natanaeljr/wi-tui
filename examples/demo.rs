use cui::rect::Rect;
use cui::render::{RenderCtx, Renderer};
use cui::widgets::{Align, Button, Column, Padding, Table, Widget};
use std::io::Read;
use std::ops::{Deref, DerefMut};

fn main() {
  let mut render_ctx = RenderCtx {
    renderer: Renderer::new(),
  };

  let button = Button::new("Button");
  // button.render(&mut render_ctx);
  // render_ctx.renderer.next_line();

  let data = vec![
    vec!["A1", "B2", "C1"], //
    vec!["A2", "B2", "C2"], //
    vec!["A3", "B3", "C3"], //
  ];

  let table = Table::new().data(data);

  let aligned_data = vec![vec![
    Align::centered("Hello"),
    Align::centered("World"),
    Align::centered("Bye"),
    Align::centered("Heaven"),
  ]];

  let columns = vec![
    Column::heading(Align::centered("A")),
    Column::heading(Align::centered("B")),
    Column::heading(Align::centered("C")),
    Column::heading(Align::centered("D")),
  ];

  let table = Table::new().columns(columns).data(aligned_data);

  let table = Table::new()
    .columns(vec![
      Column::heading("A"), //
      Column::heading("B"), //
      Column::heading("C"), //
    ])
    .data(vec![
      vec!["A1", "B2", "C1"], //
      vec!["A2", "B2", "C2"], //
      vec!["A3", "B3", "C3"], //
      vec!["A4", "B4", "C4"], //
      vec!["A5", "B5", "C5"], //
      vec!["A6", "B6", "C6"], //
      vec!["A7", "B7", "C7"], //
      vec!["A8", "B8", "C8"], //
    ]);
  render_ctx
    .renderer
    .set_frame(Rect::from_size_unchecked((0, 0), (50, 10)));
  let table = Padding::around(table).left(5).top(0);
  table.render(&mut render_ctx);

  render_ctx
    .renderer
    .set_frame(Rect::from_size_unchecked((0, 0), (50, 10)));
  // render_ctx.renderer.next_line();
  // std::thread::sleep(std::time::Duration::from_secs(5));

  let table: Table<Box<dyn Widget>> = Table::new()
    .columns(vec![
      Column::heading(Box::new("Hi") as Box<dyn Widget>), //
      Column::heading(Box::new(Align::centered("Hey")) as Box<dyn Widget>), //
    ])
    .data(vec![vec![
      Box::new("Bye") as Box<dyn Widget>,
      Box::new("SeeYou!".to_string()) as Box<dyn Widget>,
    ]]);
  let mut table = table;
  let mut data = table.data_mut_as::<Vec<Vec<Box<dyn Widget>>>>().unwrap();
  data[0][1] = Box::new("Tschuss");

  let table = Padding::around(table).left(20).top(2);
  table.render(&mut render_ctx);
  // render_ctx.renderer.next_line();
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
}
