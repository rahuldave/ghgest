use std::path::{Path, PathBuf};

use clap::Args;
use serde_json::Value;

use crate::{
  config::Config,
  ui::{components::ConfigSet, theme::Theme},
};

const GLOBAL_CONFIG_NAMES: &[&str] = &["config.json", "config.toml", "config.yaml", "config.yml"];
const PROJECT_EXTERNAL_NAMES: &[&str] = &[".gest.json", ".gest.toml", ".gest.yaml", ".gest.yml"];
const PROJECT_INREPO_NAMES: &[&str] = &[
  ".gest/config.json",
  ".gest/config.toml",
  ".gest/config.yaml",
  ".gest/config.yml",
];

/// Set a configuration value
#[derive(Debug, Args)]
pub struct Command {
  /// Dot-delimited config key (e.g. harness.command)
  pub key: String,
  /// Value to set
  pub value: String,
  /// Write to the global (user-level) config instead of the project config
  #[arg(short, long)]
  pub global: bool,
}

impl Command {
  pub fn call(&self, _config: &Config, theme: &Theme) -> crate::Result<()> {
    let scope = if self.global { Scope::Global } else { Scope::Project };

    let config_path = resolve_config_path(&scope)?;

    let mut json = if config_path.exists() {
      load_file(&config_path)?
    } else {
      serde_json::json!({})
    };

    set_dot_path(&mut json, &self.key, &self.value)?;

    if let Some(parent) = config_path.parent() {
      std::fs::create_dir_all(parent)?;
    }

    write_config(&config_path, &json)?;

    let scope_label = match scope {
      Scope::Global => "global",
      Scope::Project => "project",
    };

    ConfigSet::new(&self.key, &self.value, scope_label).write_to(&mut std::io::stdout(), theme)?;

    Ok(())
  }
}

#[derive(Debug)]
enum Scope {
  Global,
  Project,
}

fn find_existing(dir: &Path, names: &[&str]) -> Option<PathBuf> {
  for name in names {
    let path = dir.join(name);
    if path.exists() {
      return Some(path);
    }
  }
  None
}

fn load_file(path: &Path) -> crate::Result<Value> {
  let content = std::fs::read_to_string(path)?;
  let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
  match ext {
    "json" => Ok(serde_json::from_str(&content)?),
    "toml" => {
      let toml_value: toml::Value = toml::from_str(&content)?;
      let json_str = serde_json::to_string(&toml_value)?;
      Ok(serde_json::from_str(&json_str)?)
    }
    "yaml" | "yml" => Ok(yaml_serde::from_str(&content)?),
    _ => Err(crate::Error::generic(format!(
      "Unsupported config file extension: {ext}"
    ))),
  }
}

fn resolve_config_path(scope: &Scope) -> crate::Result<PathBuf> {
  match scope {
    Scope::Global => {
      if let Some(config_home) = dir_spec::config_home() {
        let config_dir = config_home.join("gest");
        if let Some(existing) = find_existing(&config_dir, GLOBAL_CONFIG_NAMES) {
          return Ok(existing);
        }
        return Ok(config_dir.join("config.toml"));
      }
      Err(crate::Error::generic("Unable to determine global config directory"))
    }
    Scope::Project => {
      let cwd = std::env::current_dir()?;

      if cwd.join(".gest").is_dir() {
        if let Some(existing) = find_existing(&cwd, PROJECT_INREPO_NAMES) {
          return Ok(existing);
        }
        return Ok(cwd.join(".gest/config.toml"));
      }

      if let Some(existing) = find_existing(&cwd, PROJECT_EXTERNAL_NAMES) {
        return Ok(existing);
      }

      let mut current = cwd.clone();
      loop {
        if current.join(".git").is_dir() {
          if let Some(existing) = find_existing(&current, PROJECT_EXTERNAL_NAMES) {
            return Ok(existing);
          }
          return Ok(current.join(".gest.toml"));
        }
        if !current.pop() {
          break;
        }
      }

      Ok(cwd.join(".gest.toml"))
    }
  }
}

fn set_dot_path(json: &mut Value, key: &str, value: &str) -> crate::Result<()> {
  let segments: Vec<&str> = key.split('.').collect();
  let mut current = json;

  for segment in &segments[..segments.len() - 1] {
    if !current.is_object() {
      *current = serde_json::json!({});
    }
    current = current
      .as_object_mut()
      .unwrap()
      .entry(*segment)
      .or_insert_with(|| serde_json::json!({}));
  }

  let last = segments.last().unwrap();
  if let Some(obj) = current.as_object_mut() {
    obj.insert((*last).to_string(), Value::String(value.to_string()));
  }

  Ok(())
}

fn write_config(path: &Path, json: &Value) -> crate::Result<()> {
  let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("toml");
  let content = match ext {
    "json" => serde_json::to_string_pretty(json)?,
    "toml" => {
      let toml_value: toml::Value = serde_json::from_value(json.clone())
        .map_err(|e| crate::Error::generic(format!("Failed to convert config to TOML: {e}")))?;
      toml::to_string_pretty(&toml_value)?
    }
    "yaml" | "yml" => yaml_serde::to_string(json)?,
    _ => {
      return Err(crate::Error::generic(format!(
        "Unsupported config file extension: {ext}"
      )));
    }
  };

  std::fs::write(path, content)?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  mod find_existing {
    use super::*;

    #[test]
    fn it_finds_existing_yaml_config() {
      let tmp = tempfile::tempdir().unwrap();
      std::fs::create_dir(tmp.path().join(".gest")).unwrap();
      std::fs::write(tmp.path().join(".gest/config.yaml"), "harness:\n  command: claude\n").unwrap();

      let result = find_existing(tmp.path(), PROJECT_INREPO_NAMES);
      assert!(result.is_some());
      assert!(result.unwrap().ends_with("config.yaml"));
    }

    #[test]
    fn it_returns_none_when_no_config_exists() {
      let tmp = tempfile::tempdir().unwrap();
      let result = find_existing(tmp.path(), PROJECT_INREPO_NAMES);
      assert!(result.is_none());
    }
  }

  mod set_dot_path {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_overwrites_an_existing_key() {
      let mut json = serde_json::json!({"harness": {"command": "claude"}});
      set_dot_path(&mut json, "harness.command", "codex").unwrap();
      assert_eq!(json, serde_json::json!({"harness": {"command": "codex"}}));
    }

    #[test]
    fn it_preserves_sibling_keys() {
      let mut json = serde_json::json!({"harness": {"command": "claude", "other": "val"}});
      set_dot_path(&mut json, "harness.command", "codex").unwrap();
      assert_eq!(
        json,
        serde_json::json!({"harness": {"command": "codex", "other": "val"}})
      );
    }

    #[test]
    fn it_sets_a_nested_key() {
      let mut json = serde_json::json!({});
      set_dot_path(&mut json, "harness.command", "codex").unwrap();
      assert_eq!(json, serde_json::json!({"harness": {"command": "codex"}}));
    }

    #[test]
    fn it_sets_a_top_level_key() {
      let mut json = serde_json::json!({});
      set_dot_path(&mut json, "key", "value").unwrap();
      assert_eq!(json, serde_json::json!({"key": "value"}));
    }
  }

  mod write_config {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_writes_json_format() {
      let tmp = tempfile::tempdir().unwrap();
      let config_path = tmp.path().join("config.json");
      let mut json = serde_json::json!({});
      set_dot_path(&mut json, "harness.command", "codex").unwrap();
      write_config(&config_path, &json).unwrap();

      let content = std::fs::read_to_string(&config_path).unwrap();
      let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
      assert_eq!(parsed["harness"]["command"], "codex");
    }

    #[test]
    fn it_writes_toml_format() {
      let tmp = tempfile::tempdir().unwrap();
      let config_path = tmp.path().join("config.toml");
      let mut json = serde_json::json!({});
      set_dot_path(&mut json, "harness.command", "codex").unwrap();
      write_config(&config_path, &json).unwrap();

      let content = std::fs::read_to_string(&config_path).unwrap();
      assert!(content.contains("codex"), "TOML should contain 'codex', got: {content}");
      let parsed: toml::Value = toml::from_str(&content).unwrap();
      assert_eq!(parsed["harness"]["command"].as_str().unwrap(), "codex");
    }

    #[test]
    fn it_writes_yaml_format() {
      let tmp = tempfile::tempdir().unwrap();
      let config_path = tmp.path().join("config.yaml");
      let mut json = serde_json::json!({});
      set_dot_path(&mut json, "harness.command", "codex").unwrap();
      write_config(&config_path, &json).unwrap();

      let content = std::fs::read_to_string(&config_path).unwrap();
      assert!(content.contains("codex"), "YAML should contain 'codex', got: {content}");
    }
  }
}
