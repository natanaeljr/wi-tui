use std::fmt::Display;

pub trait Renderer {
  fn print(&mut self, buf: &str) {
    print!("{}", buf);
  }
}

pub struct BufferedRenderer {}

impl BufferedRenderer {
  // TODO: configure alternate screens, plugins, etc.
}

impl Renderer for BufferedRenderer {}

pub struct RenderCtx {
  // widget constraints box
  pub renderer: Box<dyn Renderer>,
}
