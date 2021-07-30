use cui::render::{BufferedRenderer, RenderCtx};
use cui::widgets::{Align, Button, Column, Table, Widget};
use std::ops::{Deref, DerefMut};

fn main() {
  let mut render_ctx = RenderCtx {
    renderer: Box::new(BufferedRenderer {}),
  };

  let button = Button::new("Button");
  button.render(&mut render_ctx);
  render_ctx.renderer.next_line();

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
    ]);

  table.render(&mut render_ctx);
  render_ctx.renderer.next_line();

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

  table.render(&mut render_ctx);
  render_ctx.renderer.next_line();

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
