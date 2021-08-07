use cui::render::{RenderCtx, Renderer};
use cui::widgets::{Align, Button, Table, VerticalContainer, Widget};
use euclid::default::{Rect, Size2D};

struct App {
  root: Option<Box<dyn Widget>>,
}

impl App {
  pub fn new() -> Self {
    Self { root: None }
  }

  pub fn root_widget<W: Widget + 'static>(mut self, root: W) -> Self {
    self.root = Some(Box::new(root));
    self
  }

  pub fn render(&self) {
    let mut render_ctx = RenderCtx::new();
    // render_ctx.set_frame(Rect::from_size(Size2D::new(60, 20)));
    self.root.as_ref().unwrap().render(&mut render_ctx);
  }
}

fn main() {
  let app = App::new().root_widget(Align::centered(
    VerticalContainer::new()
      .child("Hello")
      .child(String::from("Welt"))
      .child(Button::new("CUI")),
  ));

  let other = VerticalContainer::children(vec![
    "Hello Hello Hello",
    "how how how",
    "are are are",
    "you you you you you you",
  ]);

  // app.handle_input();
  app.render();
  // app.swap_buffers();
}
