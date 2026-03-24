use clap::Args;

use crate::config::Config;

/// Get a configuration value by key
#[derive(Debug, Args)]
pub struct Command {
  /// Dot-delimited config key (e.g. harness.command)
  pub key: String,
}

impl Command {
  pub fn call(&self, config: &Config) -> crate::Result<()> {
    let value = resolve_key(config, &self.key)?;
    println!("{value}");
    Ok(())
  }
}

fn resolve_key(config: &crate::config::Config, key: &str) -> crate::Result<String> {
  let json = serde_json::to_value(config)?;
  let mut current = &json;

  for segment in key.split('.') {
    match current.get(segment) {
      Some(v) => current = v,
      None => {
        return Err(crate::Error::generic(format!("Unknown config key: '{key}'")));
      }
    }
  }

  match current {
    serde_json::Value::String(s) => Ok(s.clone()),
    serde_json::Value::Null => Ok("null".to_string()),
    other => Ok(other.to_string()),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod resolve_key {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_errors_on_unknown_key() {
      let config = crate::config::Config::default();
      let result = resolve_key(&config, "nonexistent.key");
      assert!(result.is_err());
      let err = result.unwrap_err().to_string();
      assert!(
        err.contains("Unknown config key"),
        "Expected unknown key error, got: {err}"
      );
    }

    #[test]
    fn it_errors_on_unset_optional_key() {
      let config = crate::config::Config::default();
      let result = resolve_key(&config, "storage.data_dir");
      assert!(result.is_err());
    }

    #[test]
    fn it_resolves_harness_command() {
      let config = crate::config::Config::default();
      let value = resolve_key(&config, "harness.command").unwrap();
      assert_eq!(value, "claude");
    }

    #[test]
    fn it_resolves_storage_data_dir_when_set() {
      let config = crate::config::Config {
        storage: crate::config::StorageConfig {
          data_dir: Some(std::path::PathBuf::from("/custom/path")),
        },
        ..Default::default()
      };
      let value = resolve_key(&config, "storage.data_dir").unwrap();
      assert_eq!(value, "/custom/path");
    }

    #[test]
    fn it_resolves_top_level_section() {
      let config = crate::config::Config::default();
      let value = resolve_key(&config, "harness").unwrap();
      assert!(value.contains("command"));
    }
  }
}
