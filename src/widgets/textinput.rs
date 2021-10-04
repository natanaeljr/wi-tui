use crate::render::RenderCtx;
use crate::widgets::{
  AnyEvent, Capability, EventResult, LayoutError, LayoutResult, LayoutSize, RenderResult, Text, Widget,
};
use crate::{Event, KeyCode, KeyModifiers};
use crossterm::event::KeyEvent;
use euclid::default::Size2D;

pub struct TextInput {
  pub text: Text,
}

impl TextInput {
  pub fn new() -> Self {
    Self {
      text: Text::new(String::new()),
    }
  }

  pub fn text(mut self, text: Text) -> Self {
    self.text = text;
    self
  }

  pub fn with_text(text: Text) -> Self {
    Self { text }
  }
}

impl Widget for TextInput {
  fn event(&mut self, event: &AnyEvent, size: &Size2D<usize>) -> EventResult {
    match event {
      AnyEvent::Input(input) => match input {
        Event::Key(key) => {
          match key.code {
            KeyCode::Backspace => {
              self.text.data.pop();
              return EventResult::Done;
            }
            KeyCode::Enter => {
              self.text.data.push('\n');
              return EventResult::Done;
            }
            KeyCode::Left => {}
            KeyCode::Right => {}
            KeyCode::Up => {}
            KeyCode::Down => {}
            KeyCode::Home => {}
            KeyCode::End => {}
            KeyCode::PageUp => {}
            KeyCode::PageDown => {}
            KeyCode::Tab => {}
            KeyCode::BackTab => {}
            KeyCode::Delete => {}
            KeyCode::Insert => {}
            KeyCode::F(_) => {}
            KeyCode::Char(char) if key.modifiers.is_empty() || key.modifiers == KeyModifiers::SHIFT => {
              self.text.data.push(char);
              return EventResult::Done;
            }
            KeyCode::Char(_) => {}
            KeyCode::Null => {}
            KeyCode::Esc => {}
          }
          EventResult::Unhandled
        }
        Event::Mouse(_) => EventResult::Unhandled,
        Event::Resize(_, _) => EventResult::Unhandled,
      },
    }
  }

  fn layout(&self, avail_size: &Size2D<usize>) -> LayoutSize {
    self.text.layout(avail_size)
  }

  fn render(&self, ctx: &RenderCtx) -> RenderResult {
    ctx.render_child_widget(ctx.get_frame().clone(), &self.text)
  }

  fn has_capability(&self, capability: &Capability) -> bool {
    todo!()
  }
}
