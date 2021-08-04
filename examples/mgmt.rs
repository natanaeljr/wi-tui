use cui::rect::Rect;
use cui::render::{RenderCtx, Renderer};
use cui::widgets::{Align, Button, Column, Table, VerticalContainer, Widget};

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
  let app = App::new().root_widget(Align::centered(
    VerticalContainer::new()
      .child("Hello")
      .child(String::from("Welt"))
      .child(Button::new("CUI")),
  ));

  // app.handle_input();
  app.render();
  // app.swap_buffers();
}

mod tmp {
  use euclid::default::{Size2D, Rect, Point2D};

  fn myfn() {
    let rect = Rect::new(Point2D::zero(), Size2D::new(100, 40));
    let s = Size2D::new(10usize, 20usize);
    let u = Size2D::new(10usize, 20usize);
    let x = u * s;
  }
}
