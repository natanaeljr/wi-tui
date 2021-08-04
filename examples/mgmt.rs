use cui::rect::Rect;
use cui::render::{RenderCtx, Renderer};
use cui::widgets::{Button, Column, Container, Table, Widget};

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
    let mut render_ctx = RenderCtx {
      renderer: Renderer::new(),
    };
    render_ctx
      .renderer
      .set_frame(Rect::from_size_unchecked((0, 0), (60, 20)));
    self.root.as_ref().unwrap().render(&mut render_ctx);
  }
}

fn main() {
  let app = App::new().root_widget(
    Container::vertical()
      .child("Hello")
      .child(String::from("Welt"))
      .child(Button::new("CUI")),
  );

  // app.handle_input();
  app.render();
  // app.swap_buffers();
}
