//! User-configurable color and text style definitions.
//!
//! Colors can be specified as named colors, hex strings (`#RRGGBB`), or
//! tables with foreground, background, and modifier fields.

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Deserializer, Serialize};
use yansi::{Color, Style};

/// Two-tier color configuration: semantic palette slots and per-token overrides.
///
/// The `palette` map accepts the 11 [`PaletteColor`] keys with color-only values.
/// The `overrides` map accepts the ~95 theme token keys with full [`ColorValue`] format.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Settings {
  /// Per-token style overrides keyed by dot-separated token name.
  pub overrides: HashMap<String, ColorValue>,
  /// Semantic palette color overrides keyed by palette slot name.
  pub palette: HashMap<String, Color>,
}

impl Settings {
  /// Returns `true` if neither palette nor overrides are configured.
  pub fn is_empty(&self) -> bool {
    self.overrides.is_empty() && self.palette.is_empty()
  }

  /// Iterates over all configured token override entries.
  pub fn iter(&self) -> impl Iterator<Item = (&String, &ColorValue)> {
    self.overrides.iter()
  }
}

impl<'de> Deserialize<'de> for Settings {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    #[derive(Deserialize)]
    #[serde(default)]
    #[derive(Default)]
    struct RawSettings {
      #[serde(default)]
      overrides: HashMap<String, ColorValue>,
      #[serde(default)]
      palette: HashMap<String, String>,
    }

    let raw = RawSettings::deserialize(deserializer)?;

    let valid_keys: HashSet<&str> = crate::ui::theming::palette::PaletteColor::ALL
      .iter()
      .map(|p| p.key())
      .collect();

    let mut palette = HashMap::new();
    for (key, value) in raw.palette {
      if !valid_keys.contains(key.as_str()) {
        log::warn!("unknown palette key  key={key:?}");
        continue;
      }
      let color = parse_color(&value).map_err(serde::de::Error::custom)?;
      palette.insert(key, color);
    }

    Ok(Settings {
      overrides: raw.overrides,
      palette,
    })
  }
}

impl Serialize for Settings {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    use serde::ser::SerializeMap;

    let mut map = serializer.serialize_map(None)?;
    if !self.overrides.is_empty() {
      map.serialize_entry("overrides", &self.overrides)?;
    }
    if !self.palette.is_empty() {
      let palette_strings: HashMap<&str, String> = self
        .palette
        .iter()
        .map(|(k, c)| (k.as_str(), color_to_string(*c)))
        .collect();
      map.serialize_entry("palette", &palette_strings)?;
    }
    map.end()
  }
}

/// A resolved color and text modifier specification.
#[derive(Clone, Debug, PartialEq)]
pub struct ColorValue {
  /// Background color, if set.
  pub bg: Option<Color>,
  /// Whether bold weight is enabled.
  pub bold: bool,
  /// Whether dim/faint rendering is enabled.
  pub dim: bool,
  /// Foreground color, if set.
  pub fg: Option<Color>,
  /// Whether italic style is enabled.
  pub italic: bool,
  /// Whether underline decoration is enabled.
  pub underline: bool,
}

impl ColorValue {
  /// Applies this value's colors and modifiers to the given [`Style`].
  pub fn apply_to(&self, mut style: Style) -> Style {
    if let Some(fg) = self.fg {
      style = style.fg(fg);
    }
    if let Some(bg) = self.bg {
      style = style.bg(bg);
    }
    if self.bold {
      style = style.bold();
    }
    if self.dim {
      style = style.dim();
    }
    if self.italic {
      style = style.italic();
    }
    if self.underline {
      style = style.underline();
    }
    style
  }
}

impl<'de> Deserialize<'de> for ColorValue {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum RawColorValue {
      String(String),
      Table(RawColorTable),
    }

    #[derive(Deserialize)]
    struct RawColorTable {
      #[serde(default)]
      bg: Option<String>,
      #[serde(default)]
      bold: bool,
      #[serde(default)]
      dim: bool,
      #[serde(default)]
      fg: Option<String>,
      #[serde(default)]
      italic: bool,
      #[serde(default)]
      underline: bool,
    }

    let raw = RawColorValue::deserialize(deserializer)?;
    match raw {
      RawColorValue::String(s) => {
        let color = parse_color(&s).map_err(serde::de::Error::custom)?;
        Ok(ColorValue {
          bg: None,
          bold: false,
          dim: false,
          fg: Some(color),
          italic: false,
          underline: false,
        })
      }
      RawColorValue::Table(table) => {
        let fg = table
          .fg
          .as_deref()
          .map(parse_color)
          .transpose()
          .map_err(serde::de::Error::custom)?;
        let bg = table
          .bg
          .as_deref()
          .map(parse_color)
          .transpose()
          .map_err(serde::de::Error::custom)?;
        Ok(ColorValue {
          bg,
          bold: table.bold,
          dim: table.dim,
          fg,
          italic: table.italic,
          underline: table.underline,
        })
      }
    }
  }
}

impl Serialize for ColorValue {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    use serde::ser::SerializeMap;

    if self.bg.is_none()
      && !self.bold
      && !self.dim
      && !self.italic
      && !self.underline
      && let Some(fg) = self.fg
    {
      return serializer.serialize_str(&color_to_string(fg));
    }

    let mut map = serializer.serialize_map(None)?;
    if let Some(fg) = self.fg {
      map.serialize_entry("fg", &color_to_string(fg))?;
    }
    if let Some(bg) = self.bg {
      map.serialize_entry("bg", &color_to_string(bg))?;
    }
    if self.bold {
      map.serialize_entry("bold", &true)?;
    }
    if self.dim {
      map.serialize_entry("dim", &true)?;
    }
    if self.italic {
      map.serialize_entry("italic", &true)?;
    }
    if self.underline {
      map.serialize_entry("underline", &true)?;
    }
    map.end()
  }
}

/// Converts a [`Color`] back to its canonical string representation.
fn color_to_string(color: Color) -> String {
  match color {
    Color::Black => "black".to_string(),
    Color::Blue => "blue".to_string(),
    Color::BrightBlack => "bright black".to_string(),
    Color::BrightBlue => "bright blue".to_string(),
    Color::BrightCyan => "bright cyan".to_string(),
    Color::BrightGreen => "bright green".to_string(),
    Color::BrightMagenta => "bright magenta".to_string(),
    Color::BrightRed => "bright red".to_string(),
    Color::BrightWhite => "bright white".to_string(),
    Color::BrightYellow => "bright yellow".to_string(),
    Color::Cyan => "cyan".to_string(),
    Color::Green => "green".to_string(),
    Color::Magenta => "magenta".to_string(),
    Color::Red => "red".to_string(),
    Color::Rgb(r, g, b) => format!("#{r:02X}{g:02X}{b:02X}"),
    Color::White => "white".to_string(),
    Color::Yellow => "yellow".to_string(),
    _ => "white".to_string(),
  }
}

/// Parses a color string, dispatching to hex or named-color parsing.
fn parse_color(s: &str) -> Result<Color, String> {
  if let Some(hex) = s.strip_prefix('#') {
    parse_hex_color(hex)
  } else {
    parse_named_color(s)
  }
}

/// Parses a 6-digit hex color string (without the `#` prefix) into an RGB [`Color`].
fn parse_hex_color(hex: &str) -> Result<Color, String> {
  if hex.len() != 6 {
    return Err(format!("invalid hex color: #{hex} (expected 6 hex digits)"));
  }
  let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| format!("invalid hex color: #{hex}"))?;
  let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| format!("invalid hex color: #{hex}"))?;
  let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| format!("invalid hex color: #{hex}"))?;
  Ok(Color::Rgb(r, g, b))
}

/// Maps a case-insensitive color name (e.g. `"bright cyan"`) to a [`Color`].
fn parse_named_color(name: &str) -> Result<Color, String> {
  match name.to_lowercase().as_str() {
    "black" => Ok(Color::Black),
    "blue" => Ok(Color::Blue),
    "bright black" => Ok(Color::BrightBlack),
    "bright blue" => Ok(Color::BrightBlue),
    "bright cyan" => Ok(Color::BrightCyan),
    "bright green" => Ok(Color::BrightGreen),
    "bright magenta" => Ok(Color::BrightMagenta),
    "bright red" => Ok(Color::BrightRed),
    "bright white" => Ok(Color::BrightWhite),
    "bright yellow" => Ok(Color::BrightYellow),
    "cyan" => Ok(Color::Cyan),
    "green" => Ok(Color::Green),
    "magenta" => Ok(Color::Magenta),
    "red" => Ok(Color::Red),
    "white" => Ok(Color::White),
    "yellow" => Ok(Color::Yellow),
    _ => Err(format!("unknown color name: {name}")),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod color_value {
    use super::*;

    mod apply_to {
      use pretty_assertions::assert_eq;

      use super::*;

      #[test]
      fn it_applies_all_modifiers() {
        let value = ColorValue {
          bg: Some(Color::Black),
          bold: true,
          dim: true,
          fg: Some(Color::White),
          italic: true,
          underline: true,
        };
        let style = value.apply_to(Style::new());

        assert_eq!(
          style,
          Style::new()
            .fg(Color::White)
            .bg(Color::Black)
            .bold()
            .dim()
            .italic()
            .underline()
        );
      }

      #[test]
      fn it_applies_bold_and_fg() {
        let value = ColorValue {
          bg: None,
          bold: true,
          dim: false,
          fg: Some(Color::Rgb(148, 72, 199)),
          italic: false,
          underline: false,
        };
        let style = value.apply_to(Style::new());

        assert_eq!(style, Style::new().fg(Color::Rgb(148, 72, 199)).bold());
      }

      #[test]
      fn it_applies_fg_color() {
        let value = ColorValue {
          bg: None,
          bold: false,
          dim: false,
          fg: Some(Color::Red),
          italic: false,
          underline: false,
        };
        let style = value.apply_to(Style::new());

        assert_eq!(style, Style::new().fg(Color::Red));
      }
    }

    mod deserialize {
      use pretty_assertions::assert_eq;

      use super::*;

      #[test]
      fn it_deserializes_full_colors_section() {
        let toml_str = r##"
[colors]
"log.error" = "#D23434"
"log.warn" = "yellow"
emphasis = { fg = "#9448C7", bold = true }
"##;

        #[derive(Deserialize)]
        struct TestConfig {
          #[serde(default)]
          colors: std::collections::HashMap<String, ColorValue>,
        }

        let config: TestConfig = toml::from_str(toml_str).unwrap();

        assert_eq!(config.colors.len(), 3);
        assert_eq!(config.colors["log.error"].fg, Some(Color::Rgb(210, 52, 52)));
        assert_eq!(config.colors["log.warn"].fg, Some(Color::Yellow));
        assert_eq!(config.colors["emphasis"].fg, Some(Color::Rgb(148, 72, 199)));
        assert!(config.colors["emphasis"].bold);
      }

      #[test]
      fn it_deserializes_hex_string() {
        let value: ColorValue = toml::from_str::<toml::Table>(r##"color = "#D23434""##).unwrap()["color"]
          .clone()
          .try_into()
          .unwrap();

        assert_eq!(value.fg, Some(Color::Rgb(210, 52, 52)));
        assert!(!value.bold);
      }

      #[test]
      fn it_deserializes_inline_table() {
        let toml_str = r##"
[color]
fg = "#9448C7"
bold = true
"##;
        let table: toml::Table = toml::from_str(toml_str).unwrap();
        let value: ColorValue = table["color"].clone().try_into().unwrap();

        assert_eq!(value.fg, Some(Color::Rgb(148, 72, 199)));
        assert!(value.bold);
        assert!(!value.dim);
        assert!(!value.italic);
        assert!(!value.underline);
        assert_eq!(value.bg, None);
      }

      #[test]
      fn it_deserializes_inline_table_with_bg() {
        let toml_str = r##"
[color]
fg = "red"
bg = "#000000"
underline = true
"##;
        let table: toml::Table = toml::from_str(toml_str).unwrap();
        let value: ColorValue = table["color"].clone().try_into().unwrap();

        assert_eq!(value.fg, Some(Color::Red));
        assert_eq!(value.bg, Some(Color::Rgb(0, 0, 0)));
        assert!(value.underline);
      }

      #[test]
      fn it_deserializes_named_color() {
        let value: ColorValue = toml::from_str::<toml::Table>("color = \"yellow\"").unwrap()["color"]
          .clone()
          .try_into()
          .unwrap();

        assert_eq!(value.fg, Some(Color::Yellow));
      }
    }
  }

  mod settings {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_accepts_all_11_palette_keys() {
      let toml_str = r##"
[colors.palette]
accent = "#FF0000"
border = "#FF0000"
error = "#FF0000"
primary = "#FF0000"
"primary.dark" = "#FF0000"
"primary.light" = "#FF0000"
success = "#FF0000"
text = "#FF0000"
"text.dim" = "#FF0000"
"text.muted" = "#FF0000"
warning = "#FF0000"
"##;

      #[derive(Deserialize)]
      struct TestConfig {
        #[serde(default)]
        colors: Settings,
      }

      let config: TestConfig = toml::from_str(toml_str).unwrap();

      assert_eq!(config.colors.palette.len(), 11);
    }

    #[test]
    fn it_defaults_to_empty() {
      let settings = Settings::default();

      assert!(settings.is_empty());
    }

    #[test]
    fn it_deserializes_both_sections() {
      let toml_str = r##"
[colors.palette]
primary = "#9448C7"

[colors.overrides]
"status.done" = { fg = "#AABBCC", bold = true }
"##;

      #[derive(Deserialize)]
      struct TestConfig {
        #[serde(default)]
        colors: Settings,
      }

      let config: TestConfig = toml::from_str(toml_str).unwrap();

      assert_eq!(config.colors.palette.len(), 1);
      assert_eq!(config.colors.overrides.len(), 1);
      assert_eq!(config.colors.palette["primary"], Color::Rgb(148, 72, 199));
      assert!(config.colors.overrides["status.done"].bold);
    }

    #[test]
    fn it_deserializes_overrides_section() {
      let toml_str = r##"
[colors.overrides]
emphasis = "#9448C7"
"log.error" = "red"
"##;

      #[derive(Deserialize)]
      struct TestConfig {
        #[serde(default)]
        colors: Settings,
      }

      let config: TestConfig = toml::from_str(toml_str).unwrap();

      assert!(!config.colors.is_empty());

      let entries: Vec<_> = config.colors.iter().collect();
      assert_eq!(entries.len(), 2);
    }

    #[test]
    fn it_deserializes_palette_section() {
      let toml_str = r##"
[colors.palette]
primary = "#9448C7"
success = "#00FF88"
"##;

      #[derive(Deserialize)]
      struct TestConfig {
        #[serde(default)]
        colors: Settings,
      }

      let config: TestConfig = toml::from_str(toml_str).unwrap();

      assert!(!config.colors.is_empty());
      assert_eq!(config.colors.palette.len(), 2);
      assert_eq!(config.colors.palette["primary"], Color::Rgb(148, 72, 199));
      assert_eq!(config.colors.palette["success"], Color::Rgb(0, 255, 136));
    }

    #[test]
    fn it_is_empty_when_no_palette_or_overrides() {
      let settings = Settings::default();

      assert!(settings.is_empty());
      assert_eq!(settings.palette.len(), 0);
      assert_eq!(settings.overrides.len(), 0);
    }

    #[test]
    fn it_roundtrips_through_serialization() {
      let mut settings = Settings::default();
      settings.palette.insert("primary".to_string(), Color::Rgb(148, 72, 199));
      settings.overrides.insert(
        "emphasis".to_string(),
        ColorValue {
          bg: None,
          bold: true,
          dim: false,
          fg: Some(Color::Rgb(148, 72, 199)),
          italic: false,
          underline: false,
        },
      );

      let toml_str = toml::to_string(&settings).unwrap();
      let deserialized: Settings = toml::from_str(&toml_str).unwrap();

      assert_eq!(deserialized.palette["primary"], Color::Rgb(148, 72, 199));
      assert_eq!(deserialized.overrides["emphasis"].fg, Some(Color::Rgb(148, 72, 199)));
      assert!(deserialized.overrides["emphasis"].bold);
    }

    #[test]
    fn it_skips_unknown_palette_keys() {
      let toml_str = r##"
[colors.palette]
primary = "#9448C7"
nonexistent = "#FF0000"
"##;

      #[derive(Deserialize)]
      struct TestConfig {
        #[serde(default)]
        colors: Settings,
      }

      let config: TestConfig = toml::from_str(toml_str).unwrap();

      assert_eq!(config.colors.palette.len(), 1);
      assert!(config.colors.palette.contains_key("primary"));
    }
  }

  mod parse_color {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_parses_bright_named_color() {
      let color = parse_color("bright cyan").unwrap();

      assert_eq!(color, Color::BrightCyan);
    }

    #[test]
    fn it_parses_hex_color() {
      let color = parse_color("#D23434").unwrap();

      assert_eq!(color, Color::Rgb(210, 52, 52));
    }

    #[test]
    fn it_parses_hex_color_lowercase() {
      let color = parse_color("#9448c7").unwrap();

      assert_eq!(color, Color::Rgb(148, 72, 199));
    }

    #[test]
    fn it_parses_named_color() {
      let color = parse_color("red").unwrap();

      assert_eq!(color, Color::Red);
    }

    #[test]
    fn it_parses_named_color_case_insensitive() {
      let color = parse_color("Yellow").unwrap();

      assert_eq!(color, Color::Yellow);
    }

    #[test]
    fn it_returns_error_for_invalid_hex() {
      let result = parse_color("#GG0000");

      assert!(result.is_err());
    }

    #[test]
    fn it_returns_error_for_short_hex() {
      let result = parse_color("#FFF");

      assert!(result.is_err());
    }

    #[test]
    fn it_returns_error_for_unknown_name() {
      let result = parse_color("chartreuse");

      assert!(result.is_err());
    }
  }
}
