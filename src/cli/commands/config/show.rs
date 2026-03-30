use clap::Args;

use crate::{
  cli,
  config::Settings,
  ui::{theme::Theme, views::system::ConfigView},
};

/// Display the merged configuration and discovered config file sources.
#[derive(Debug, Args)]
pub struct Command;

impl Command {
  /// Render active settings, data directory, log level, and config file locations.
  pub fn call(&self, settings: &Settings, theme: &Theme) -> cli::Result<()> {
    let cwd = std::env::current_dir()?;
    let data_dir_path = settings.storage().data_dir(cwd)?;
    let data_dir = data_dir_path.display().to_string();
    let log_level = settings.log().level().unwrap_or("warn");

    let mut view = ConfigView::new(&data_dir, log_level, theme).has_color_overrides(!settings.colors().is_empty());

    if let Some(config_home) = dir_spec::config_home() {
      let global = config_home.join("gest/config.toml");
      if global.exists() {
        let global_str = global.display().to_string();
        view = view.global_config(Box::leak(global_str.into_boxed_str()));
      }
    }

    if let Ok(cwd) = std::env::current_dir() {
      for name in &[".gest/config.toml", ".gest.toml"] {
        let path = cwd.join(name);
        if path.exists() {
          let path_str = path.display().to_string();
          view = view.project_config(Box::leak(path_str.into_boxed_str()));
          break;
        }
      }
    }

    println!("{view}");
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod call {
    use super::*;

    #[test]
    fn it_succeeds_with_default_config() {
      let settings = Settings::default();
      let cmd = Command;
      cmd.call(&settings, &Theme::default()).unwrap();
    }
  }
}
