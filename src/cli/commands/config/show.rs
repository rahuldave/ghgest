use std::path::PathBuf;

use clap::Args;

use crate::{config::Config, ui::components::ConfigDisplay};

const GLOBAL_CONFIG_NAMES: &[&str] = &["config.json", "config.toml", "config.yaml", "config.yml"];
const PROJECT_EXTERNAL_NAMES: &[&str] = &[".gest.json", ".gest.toml", ".gest.yaml", ".gest.yml"];
const PROJECT_INREPO_NAMES: &[&str] = &[
  ".gest/config.json",
  ".gest/config.toml",
  ".gest/config.yaml",
  ".gest/config.yml",
];

/// Display the merged configuration and discovered config file sources
#[derive(Debug, Args)]
pub struct Command;

impl Command {
  pub fn call(&self, config: &Config) -> crate::Result<()> {
    let json = serde_json::to_value(config)?;
    let sources = discover_sources();

    ConfigDisplay::new(&json, &sources).write_to(&mut std::io::stdout())?;
    Ok(())
  }
}

fn discover_sources() -> Vec<PathBuf> {
  let mut sources = Vec::new();

  if let Ok(cwd) = std::env::current_dir() {
    for name in PROJECT_INREPO_NAMES {
      let path = cwd.join(name);
      if path.exists() {
        sources.push(path);
      }
    }
    for name in PROJECT_EXTERNAL_NAMES {
      let path = cwd.join(name);
      if path.exists() {
        sources.push(path);
      }
    }
    let mut current = cwd;
    loop {
      if current.join(".git").is_dir() {
        for name in PROJECT_EXTERNAL_NAMES {
          let path = current.join(name);
          if path.exists() && !sources.contains(&path) {
            sources.push(path);
          }
        }
        break;
      }
      if !current.pop() {
        break;
      }
    }
  }

  if let Some(config_home) = dir_spec::config_home() {
    let config_dir = config_home.join("gest");
    for name in GLOBAL_CONFIG_NAMES {
      let path = config_dir.join(name);
      if path.exists() {
        sources.push(path);
      }
    }
  }

  sources
}

#[cfg(test)]
mod tests {
  use super::*;

  mod call {
    use super::*;

    #[test]
    fn it_succeeds_with_default_config() {
      let config = crate::config::Config::default();
      let cmd = Command;
      cmd.call(&config).unwrap();
    }
  }
}
