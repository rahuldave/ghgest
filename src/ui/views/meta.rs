use std::fmt::{self, Display, Formatter};

use yansi::{Paint, Style};

/// Renders a metadata value with themed styling.
///
/// Scalars are displayed as styled text; compound types (arrays, tables)
/// are pretty-printed as JSON with the same style applied.
pub struct MetaValueView {
  text: String,
  style: Style,
}

impl MetaValueView {
  /// Create a view from a pre-formatted value string and a theme style.
  ///
  /// The `text` should come from `store::meta::format_toml_value` or
  /// `store::artifact_meta::format_yaml_value`.
  pub fn new(text: String, style: Style) -> Self {
    Self {
      text,
      style,
    }
  }
}

impl Display for MetaValueView {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let trimmed = self.text.trim_end_matches('\n');
    for (i, line) in trimmed.lines().enumerate() {
      if i > 0 {
        writeln!(f)?;
      }
      write!(f, "{}", line.paint(self.style))?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod meta_value_view {
    use super::*;

    #[test]
    fn it_renders_scalar_value() {
      yansi::disable();
      let view = MetaValueView::new("high\n".to_string(), Style::default());
      assert_eq!(view.to_string(), "high");
    }

    #[test]
    fn it_renders_multiline_json() {
      yansi::disable();
      let json = "{\n  \"key\": \"value\"\n}\n".to_string();
      let view = MetaValueView::new(json, Style::default());
      assert_eq!(view.to_string(), "{\n  \"key\": \"value\"\n}");
    }
  }
}
