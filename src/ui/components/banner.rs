use std::{
  env::consts::{ARCH, OS},
  fmt::{self, Write as _},
  io,
};

use yansi::Paint;

use crate::ui::colors;

const ART: &str = concat!(
  " ██████╗ ███████╗███████╗████████╗\n",
  "██╔════╝ ██╔════╝██╔════╝╚══██╔══╝\n",
  "██║  ███╗█████╗  ███████╗   ██║\n",
  "██║   ██║██╔══╝  ╚════██║   ██║\n",
  "╚██████╔╝███████╗███████║   ██║\n",
  " ╚═════╝ ╚══════╝╚══════╝   ╚═╝",
);

const AUTHOR: &str = "by @aaronmallen";

pub struct Banner {
  author: bool,
  color: bool,
  version: bool,
}

impl Banner {
  pub fn new() -> Self {
    Self {
      author: false,
      color: false,
      version: false,
    }
  }

  pub fn with_author(mut self) -> Self {
    self.author = true;
    self
  }

  pub fn with_color(mut self) -> Self {
    self.color = true;
    self
  }

  pub fn with_version(mut self) -> Self {
    self.version = true;
    self
  }

  pub fn write_to(&self, w: &mut impl io::Write) -> io::Result<()> {
    if self.color {
      write!(w, "{}", colored_banner_string())?;
    } else {
      write!(w, "{}", ART.trim_end())?;
    }

    if self.author {
      let art_width = art_width();
      let padding = art_width.saturating_sub(AUTHOR.len());
      if self.color {
        write!(w, "\n{:>pad$}{}", "", colored_author_string(), pad = padding)?;
      } else {
        write!(w, "\n{:>pad$}{}", "", AUTHOR, pad = padding)?;
      }
    }

    if self.version {
      if self.color {
        write!(w, "\n\n{}", colored_version_string())?;
      } else {
        write!(w, "\n\n{}", version_string())?;
      }
    }

    Ok(())
  }
}

impl Default for Banner {
  fn default() -> Self {
    Self::new()
  }
}

impl fmt::Display for Banner {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut buf = Vec::new();
    self.write_to(&mut buf).map_err(|_| fmt::Error)?;
    let s = String::from_utf8(buf).map_err(|_| fmt::Error)?;
    write!(f, "{}", s.trim_end())
  }
}

fn art_width() -> usize {
  ART
    .lines()
    .filter(|l| !l.is_empty())
    .map(|l| l.chars().count())
    .max()
    .unwrap_or(0)
}

fn colored_author_string() -> String {
  format!(
    "{}{}",
    "by @".fg(colors::SILVER).italic(),
    "aaronmallen".fg(colors::EMBER).bold(),
  )
}

fn colored_banner_string() -> String {
  let lines: Vec<&str> = ART.trim_end().lines().filter(|l| !l.is_empty()).collect();
  let last = (lines.len() - 1) as f32;
  let mut buf = String::with_capacity(ART.len() * 2);

  for (i, line) in lines.iter().enumerate() {
    if i > 0 {
      buf.push('\n');
    }
    let t = i as f32 / last;
    let fill = lerp_rgb(colors::VIOLET, colors::AZURE, t);

    for ch in line.chars() {
      match ch {
        '█' => write!(buf, "{}", ch.fg(fill)).unwrap(),
        '╔' | '╗' | '╚' | '╝' | '║' | '═' | '╠' | '╣' | '╦' | '╩' | '╬' => {
          write!(buf, "{}", ch.fg(colors::VIOLET_DARK)).unwrap()
        }
        _ => buf.push(ch),
      }
    }
  }

  buf
}

fn colored_version_string() -> String {
  format!(
    "v{} {}-{} ({} revision {})",
    env!("CARGO_PKG_VERSION"),
    OS,
    ARCH,
    env!("BUILD_DATE").fg(colors::AZURE),
    env!("GIT_SHA").fg(colors::JADE),
  )
}

fn lerp_rgb(from: yansi::Color, to: yansi::Color, t: f32) -> yansi::Color {
  match (from, to) {
    (yansi::Color::Rgb(r1, g1, b1), yansi::Color::Rgb(r2, g2, b2)) => yansi::Color::Rgb(
      (r1 as f32 + (r2 as f32 - r1 as f32) * t) as u8,
      (g1 as f32 + (g2 as f32 - g1 as f32) * t) as u8,
      (b1 as f32 + (b2 as f32 - b1 as f32) * t) as u8,
    ),
    _ => from,
  }
}

fn version_string() -> String {
  format!(
    "v{} {}-{} ({} revision {})",
    env!("CARGO_PKG_VERSION"),
    OS,
    ARCH,
    env!("BUILD_DATE"),
    env!("GIT_SHA"),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  mod display {
    use super::*;

    #[test]
    fn it_displays_the_ascii_art() {
      let output = format!("{}", Banner::new());

      assert!(output.contains("███████"), "Should contain block characters");
      assert!(output.contains("██████╔╝"), "Should contain box-drawing characters");
    }

    #[test]
    fn it_excludes_author_and_version_by_default() {
      let output = format!("{}", Banner::new());

      assert!(!output.contains("@aaronmallen"), "Should not contain author");
      assert!(
        !output.contains(env!("CARGO_PKG_VERSION")),
        "Should not contain version"
      );
    }

    #[test]
    fn it_includes_author_when_enabled() {
      let output = format!("{}", Banner::new().with_author());

      assert!(output.contains("by @aaronmallen"), "Should contain author line");
    }

    #[test]
    fn it_includes_version_when_enabled() {
      let output = format!("{}", Banner::new().with_version());

      assert!(output.contains(env!("CARGO_PKG_VERSION")), "Should contain version");
      assert!(output.contains(OS), "Should contain OS");
      assert!(output.contains(ARCH), "Should contain arch");
      assert!(output.contains(env!("BUILD_DATE")), "Should contain build date");
      assert!(output.contains(env!("GIT_SHA")), "Should contain git SHA");
    }

    #[test]
    fn it_includes_all_sections_when_all_enabled() {
      let output = format!("{}", Banner::new().with_author().with_version());

      assert!(output.contains("███████"), "Should contain art");
      assert!(output.contains("by @aaronmallen"), "Should contain author");
      assert!(output.contains(env!("CARGO_PKG_VERSION")), "Should contain version");
    }
  }

  mod write_to {
    use super::*;

    #[test]
    fn it_writes_ascii_art() {
      let banner = Banner::new();
      let mut buf = Vec::new();
      banner.write_to(&mut buf).unwrap();
      let output = String::from_utf8(buf).unwrap();

      assert!(output.contains("███████"), "Should contain block characters");
    }

    #[test]
    fn it_writes_author_when_enabled() {
      let banner = Banner::new().with_author();
      let mut buf = Vec::new();
      banner.write_to(&mut buf).unwrap();
      let output = String::from_utf8(buf).unwrap();

      assert!(output.contains("by @aaronmallen"), "Should contain author");
    }

    #[test]
    fn it_writes_version_when_enabled() {
      let banner = Banner::new().with_version();
      let mut buf = Vec::new();
      banner.write_to(&mut buf).unwrap();
      let output = String::from_utf8(buf).unwrap();

      assert!(output.contains(env!("CARGO_PKG_VERSION")), "Should contain version");
    }
  }

  mod author_alignment {
    use super::*;

    #[test]
    fn it_right_aligns_author_to_art_width() {
      let banner = Banner::new().with_author();
      let mut buf = Vec::new();
      banner.write_to(&mut buf).unwrap();
      let output = String::from_utf8(buf).unwrap();

      let author_line = output.lines().find(|l| l.contains("by @aaronmallen")).unwrap();
      let width = art_width();

      assert_eq!(
        author_line.chars().count(),
        width,
        "Author line width should match art width"
      );
    }
  }

  mod lerp_rgb {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_start_color_at_zero() {
      let from = yansi::Color::Rgb(100, 150, 200);
      let to = yansi::Color::Rgb(200, 50, 100);

      assert_eq!(lerp_rgb(from, to, 0.0), yansi::Color::Rgb(100, 150, 200));
    }

    #[test]
    fn it_returns_end_color_at_one() {
      let from = yansi::Color::Rgb(100, 150, 200);
      let to = yansi::Color::Rgb(200, 50, 100);

      assert_eq!(lerp_rgb(from, to, 1.0), yansi::Color::Rgb(200, 50, 100));
    }

    #[test]
    fn it_returns_midpoint_at_half() {
      let from = yansi::Color::Rgb(100, 150, 200);
      let to = yansi::Color::Rgb(200, 50, 100);

      assert_eq!(lerp_rgb(from, to, 0.5), yansi::Color::Rgb(150, 100, 150));
    }

    #[test]
    fn it_returns_from_color_for_non_rgb_variants() {
      assert_eq!(lerp_rgb(yansi::Color::Red, yansi::Color::Blue, 0.5), yansi::Color::Red);
    }
  }
}
