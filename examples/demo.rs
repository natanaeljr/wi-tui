use cui::render::{BufferedRenderer, RenderCtx};
use cui::widgets::{Align, Button, Column, Table, Widget};

fn main() {
  let mut render_ctx = RenderCtx {
    renderer: Box::new(BufferedRenderer {}),
  };

  let button = Button::new("Button");
  button.render(&mut render_ctx);

  let data = vec![
    vec!["A1", "B2", "C1"], //
    vec!["A2", "B2", "C2"], //
    vec!["A3", "B3", "C3"], //
  ];

  let table = Table::new().data(data);

  let aligned_data = vec![
    Align::centered("Hello"),
    Align::centered("World"),
    Align::centered("Bye"),
    Align::centered("Heaven"),
  ];

  let columns = vec![
    Column::heading(Align::centered("A")),
    Column::heading(Align::centered("B")),
    Column::heading(Align::centered("C")),
    Column::heading(Align::centered("D")),
  ];

  let table = Table::new().columns(columns).data(aligned_data);

  // TODO: How to mutate the columns and data
  let table = Table::new()
    .columns(vec![
      Column::heading("A"),
      Column::heading("B"),
      Column::heading("C"),
      Column::heading("D"),
    ])
    .data(vec![
      vec!["A1", "B2", "C1"], //
      vec!["A2", "B2", "C2"], //
      vec!["A3", "B3", "C3"], //
    ]);

  let col: Box<dyn Widget> = Box::new("Hi");
  let col2: Box<dyn Widget> = Box::new(Align::centered("Hey"));
  let cell1: Box<dyn Widget> = Box::new("Bye");
  let cell2: Box<dyn Widget> = Box::new("See you!".to_string());
  let table: Table<Box<dyn Widget>> = Table::new()
    .columns(vec![
      Column::heading(col),  //
      Column::heading(col2), //
    ])
    .data(vec![cell1, cell2]);
}
