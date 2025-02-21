use super::widget::Widget;
use super::label::Label;

pub struct Button {
  label: Label,
}

impl Button {
  pub fn new(label: &str) -> Button {
      Button { label: Label::new(label) }
  }
}

impl Widget for Button {
  fn width(&self) -> usize {
      self.label.width() + 8 // add a bit of padding
  }

  fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
      let width = self.width();
      let mut label = String::new();
      self.label.draw_into(&mut label);

      writeln!(buffer, "+{:-<width$}+", "").unwrap();
      for line in label.lines() {
          writeln!(buffer, "|{:^width$}|", &line).unwrap();
      }
      writeln!(buffer, "+{:-<width$}+", "").unwrap();
  }
}