use unicode_width::UnicodeWidthStr;

/// Return the visible column width of `s` after stripping ANSI escape sequences.
pub fn display_width(s: &str) -> usize {
  let stripped = strip_ansi(s);
  UnicodeWidthStr::width(stripped.as_str())
}

/// Query the terminal width, falling back to 80 columns if unavailable.
pub fn terminal_width() -> u16 {
  terminal_size::terminal_size().map(|(w, _)| w.0).unwrap_or(80)
}

/// Remove ANSI CSI and OSC escape sequences from a string.
fn strip_ansi(s: &str) -> String {
  let mut result = String::with_capacity(s.len());
  let mut chars = s.chars().peekable();

  while let Some(c) = chars.next() {
    if c == '\x1b' {
      match chars.peek() {
        Some('[') => {
          chars.next();
          while let Some(&next) = chars.peek() {
            chars.next();
            if next.is_ascii_alphabetic() {
              break;
            }
          }
        }
        Some(']') => {
          chars.next();
          while let Some(&next) = chars.peek() {
            if next == '\x07' {
              chars.next();
              break;
            }
            if next == '\x1b' {
              chars.next();
              if chars.peek() == Some(&'\\') {
                chars.next();
              }
              break;
            }
            chars.next();
          }
        }
        _ => {}
      }
    } else {
      result.push(c);
    }
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_counts_ascii_width() {
    assert_eq!(display_width("hello"), 5);
  }

  #[test]
  fn it_counts_bullet_as_single_width() {
    assert_eq!(display_width("●"), 1);
  }

  #[test]
  fn it_counts_circle_half_as_single_width() {
    assert_eq!(display_width("◐"), 1);
  }

  #[test]
  fn it_counts_circled_division_as_single_width() {
    assert_eq!(display_width("⊘"), 1);
  }

  #[test]
  fn it_counts_diamond_as_single_width() {
    assert_eq!(display_width("◆"), 1);
  }

  #[test]
  fn it_preserves_plain_text_when_stripping_ansi() {
    assert_eq!(strip_ansi("no escapes here"), "no escapes here");
  }

  #[test]
  fn it_returns_positive_terminal_width() {
    assert!(terminal_width() > 0);
  }

  #[test]
  fn it_returns_zero_for_empty_string() {
    assert_eq!(display_width(""), 0);
  }

  #[test]
  fn it_strips_ansi_csi_from_width() {
    assert_eq!(display_width("\x1b[31mred\x1b[0m"), 3);
  }

  #[test]
  fn it_strips_ansi_osc_bel_from_width() {
    assert_eq!(display_width("\x1b]0;title\x07text"), 4);
  }

  #[test]
  fn it_strips_ansi_osc_st_from_width() {
    assert_eq!(display_width("\x1b]0;title\x1b\\text"), 4);
  }
}
