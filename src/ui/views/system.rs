use std::fmt::{self, Display, Formatter};

use yansi::Paint;

use crate::ui::{
  atoms::{label::Label, value::Value},
  composites::success_message::SuccessMessage,
  theming::theme::Theme,
};

/// Renders the resolved configuration summary (paths, settings, color overrides).
pub struct ConfigView<'a> {
  global_config: Option<&'a str>,
  log_level: &'a str,
  overrides_count: usize,
  palette_count: usize,
  project_config: Option<&'a str>,
  project_dir: &'a str,
  theme: &'a Theme,
}

impl<'a> ConfigView<'a> {
  pub fn new(project_dir: &'a str, log_level: &'a str, theme: &'a Theme) -> Self {
    Self {
      global_config: None,
      log_level,
      overrides_count: 0,
      palette_count: 0,
      project_config: None,
      project_dir,
      theme,
    }
  }

  /// Sets the global config file path to display.
  pub fn global_config(mut self, path: &'a str) -> Self {
    self.global_config = Some(path);
    self
  }

  /// Sets the number of active token overrides.
  pub fn overrides_count(mut self, count: usize) -> Self {
    self.overrides_count = count;
    self
  }

  /// Sets the number of active palette overrides.
  pub fn palette_count(mut self, count: usize) -> Self {
    self.palette_count = count;
    self
  }

  /// Sets the project-local config file path to display.
  pub fn project_config(mut self, path: &'a str) -> Self {
    self.project_config = Some(path);
    self
  }
}

impl Display for ConfigView<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    writeln!(f, "{}", "configuration".paint(self.theme.config_heading),)?;

    let path_label_width = 7;
    writeln!(f)?;
    writeln!(
      f,
      "  {}  {}",
      Label::new("global", self.theme.config_label).pad_to(path_label_width),
      Value::new(self.global_config.unwrap_or("(none)"), self.theme.config_value,),
    )?;
    writeln!(
      f,
      "  {}  {}",
      Label::new("project", self.theme.config_label).pad_to(path_label_width),
      Value::new(self.project_config.unwrap_or("(none)"), self.theme.config_value,),
    )?;

    let setting_label_width = 11;
    writeln!(f)?;
    writeln!(
      f,
      "  {}  {}",
      Label::new("project_dir", self.theme.config_label).pad_to(setting_label_width),
      Value::new(self.project_dir, self.theme.config_value),
    )?;
    writeln!(
      f,
      "  {}  {}",
      Label::new("log_level", self.theme.config_label).pad_to(setting_label_width),
      Value::new(self.log_level, self.theme.config_value),
    )?;

    writeln!(f)?;
    if self.palette_count > 0 {
      let msg = format!("{} palette color(s) set", self.palette_count);
      writeln!(
        f,
        "  {}  {}",
        Label::new("palette", self.theme.config_label).pad_to(setting_label_width),
        Value::new(&msg, self.theme.config_value),
      )?;
    } else {
      writeln!(
        f,
        "  {}  {}",
        Label::new("palette", self.theme.config_label).pad_to(setting_label_width),
        "(using defaults)".paint(self.theme.config_no_overrides),
      )?;
    }
    if self.overrides_count > 0 {
      let msg = format!("{} token override(s) set", self.overrides_count);
      write!(
        f,
        "  {}  {}",
        Label::new("overrides", self.theme.config_label).pad_to(setting_label_width),
        Value::new(&msg, self.theme.config_value),
      )?;
    } else {
      write!(
        f,
        "  {}  {}",
        Label::new("overrides", self.theme.config_label).pad_to(setting_label_width),
        "(none)".paint(self.theme.config_no_overrides),
      )?;
    }

    Ok(())
  }
}

/// Renders the post-initialization success message with getting-started hints.
pub struct InitView<'a> {
  config_path: Option<&'a str>,
  project_dir: &'a str,
  theme: &'a Theme,
}

impl<'a> InitView<'a> {
  pub fn new(project_dir: &'a str, config_path: Option<&'a str>, theme: &'a Theme) -> Self {
    Self {
      project_dir,
      config_path,
      theme,
    }
  }
}

impl Display for InitView<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let mut msg = SuccessMessage::new("initialized gest", self.theme).field("project dir", self.project_dir);
    if let Some(config) = self.config_path {
      msg = msg.field("config", config);
    }
    write!(f, "{msg}")?;

    writeln!(f)?;
    writeln!(f)?;
    writeln!(f, "  {}", "get started".paint(self.theme.init_section),)?;
    for cmd in [
      "gest task create \"my first task\"",
      "gest artifact create --file spec.md",
      "gest iteration create \"sprint 1\"",
    ] {
      writeln!(
        f,
        "    {} {}",
        "$".paint(self.theme.init_command_prefix),
        cmd.paint(self.theme.init_value),
      )?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn theme() -> Theme {
    yansi::disable();
    Theme::default()
  }

  mod config_view {
    use super::*;

    mod display {
      use super::*;

      #[test]
      fn it_renders_file_paths() {
        let theme = theme();
        let view = ConfigView::new(".gest/", "warn", &theme)
          .global_config("~/.config/gest/config.toml")
          .project_config(".gest/config.toml");
        let rendered = format!("{view}");

        assert!(rendered.contains("global"));
        assert!(rendered.contains("~/.config/gest/config.toml"));
        assert!(rendered.contains("project"));
        assert!(rendered.contains(".gest/config.toml"));
      }

      #[test]
      fn it_renders_heading() {
        let theme = theme();
        let view = ConfigView::new(".gest/", "warn", &theme);
        let rendered = format!("{view}");

        assert!(rendered.contains("configuration"));
      }

      #[test]
      fn it_renders_settings() {
        let theme = theme();
        let view = ConfigView::new(".gest/", "warn", &theme);
        let rendered = format!("{view}");

        assert!(rendered.contains("project_dir"));
        assert!(rendered.contains(".gest/"));
        assert!(rendered.contains("log_level"));
        assert!(rendered.contains("warn"));
      }

      #[test]
      fn it_shows_defaults_when_no_colors_configured() {
        let theme = theme();
        let view = ConfigView::new(".gest/", "warn", &theme);
        let rendered = format!("{view}");

        assert!(rendered.contains("palette"));
        assert!(rendered.contains("using defaults"));
        assert!(rendered.contains("overrides"));
        assert!(rendered.contains("(none)"));
      }

      #[test]
      fn it_shows_none_for_missing_paths() {
        let theme = theme();
        let view = ConfigView::new(".gest/", "warn", &theme);
        let rendered = format!("{view}");

        assert!(rendered.contains("(none)"));
      }

      #[test]
      fn it_shows_overrides_count_when_present() {
        let theme = theme();
        let view = ConfigView::new(".gest/", "warn", &theme).overrides_count(3);
        let rendered = format!("{view}");

        assert!(rendered.contains("3 token override(s) set"));
      }

      #[test]
      fn it_shows_palette_and_overrides_separately() {
        let theme = theme();
        let view = ConfigView::new(".gest/", "warn", &theme)
          .palette_count(2)
          .overrides_count(5);
        let rendered = format!("{view}");

        assert!(rendered.contains("2 palette color(s) set"));
        assert!(rendered.contains("5 token override(s) set"));
      }

      #[test]
      fn it_shows_palette_count_when_present() {
        let theme = theme();
        let view = ConfigView::new(".gest/", "warn", &theme).palette_count(4);
        let rendered = format!("{view}");

        assert!(rendered.contains("4 palette color(s) set"));
      }
    }
  }

  mod init_view {
    use super::*;

    mod display {
      use super::*;

      #[test]
      fn it_renders_get_started_section() {
        let theme = theme();
        let view = InitView::new(".gest/", Some(".gest/config.toml"), &theme);
        let rendered = format!("{view}");

        assert!(rendered.contains("get started"));
        assert!(rendered.contains("$ gest task create"));
        assert!(rendered.contains("$ gest artifact create --file spec.md"));
        assert!(rendered.contains("$ gest iteration create"));
      }

      #[test]
      fn it_renders_success_and_paths_for_local_mode() {
        let theme = theme();
        let view = InitView::new(".gest/", Some(".gest/config.toml"), &theme);
        let rendered = format!("{view}");

        assert!(rendered.contains('\u{2713}'), "expected check icon");
        assert!(rendered.contains("initialized gest"));
        assert!(rendered.contains(".gest/"));
        assert!(rendered.contains(".gest/config.toml"));
      }

      #[test]
      fn it_renders_success_without_config_for_global_mode() {
        let theme = theme();
        let view = InitView::new("/home/user/.local/share/gest/abc123", None, &theme);
        let rendered = format!("{view}");

        assert!(rendered.contains('\u{2713}'), "expected check icon");
        assert!(rendered.contains("initialized gest"));
        assert!(rendered.contains("/home/user/.local/share/gest/abc123"));
        assert!(!rendered.contains("config"));
      }
    }
  }
}
