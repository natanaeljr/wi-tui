use std::fmt::Display;

pub trait Renderer {
  fn print(&mut self, buf: &str);
  fn next_line(&mut self);
}

pub struct BufferedRenderer {}

impl BufferedRenderer {
  // TODO: configure alternate screens, plugins, etc.
}

impl Renderer for BufferedRenderer {
  fn print(&mut self, buf: &str) {
    print!("{}", buf);
  }

  fn next_line(&mut self) {
    println!();
  }
}

pub struct RenderCtx {
  // widget constraints box
  pub renderer: Box<dyn Renderer>,
}
