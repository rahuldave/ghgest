use std::path::Path;

use serde_json::Value;

use super::{Config, env::GEST_CONFIG};

const GLOBAL_CONFIG_NAMES: &[&str] = &["config.json", "config.toml", "config.yaml", "config.yml"];
const PROJECT_EXTERNAL_NAMES: &[&str] = &[".gest.json", ".gest.toml", ".gest.yaml", ".gest.yml"];
const PROJECT_INREPO_NAMES: &[&str] = &[
  ".gest/config.json",
  ".gest/config.toml",
  ".gest/config.yaml",
  ".gest/config.yml",
];

pub fn load() -> crate::Result<Config> {
  let mut base = serde_json::json!({});

  let global = load_global()?;
  merge_value(&mut base, global);

  let cwd = std::env::current_dir()?;
  let project_config = if walk_up_for_dir(&cwd, ".gest").is_some() {
    log::debug!("found .gest directory, loading in-repo config");
    find_config_from_root(&cwd, ".gest", PROJECT_INREPO_NAMES)?
  } else if let Some(git_root) = walk_up_for_dir(&cwd, ".git") {
    log::debug!("found .git root at {}, loading project config", git_root.display());
    find_config(&git_root, PROJECT_EXTERNAL_NAMES)?
  } else {
    log::debug!("no .gest or .git found, loading config from cwd");
    find_config(&cwd, PROJECT_EXTERNAL_NAMES)?
  };
  merge_value(&mut base, project_config);

  let config: Config = serde_json::from_value(base)?;
  log::info!("config loaded");
  Ok(config)
}

fn find_config(dir: &Path, names: &[&str]) -> crate::Result<Value> {
  for name in names {
    let path = dir.join(name);
    let value = load_file(&path)?;
    if value != serde_json::json!({}) {
      return Ok(value);
    }
  }
  Ok(serde_json::json!({}))
}

fn find_config_from_root(start: &Path, dir_name: &str, names: &[&str]) -> crate::Result<Value> {
  if let Some(root) = walk_up_for_dir(start, dir_name) {
    find_config(&root, names)
  } else {
    Ok(serde_json::json!({}))
  }
}

fn load_file(path: &Path) -> crate::Result<Value> {
  let content = match std::fs::read_to_string(path) {
    Ok(c) => c,
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
      log::trace!("config file not found: {}", path.display());
      return Ok(serde_json::json!({}));
    }
    Err(e) => return Err(e.into()),
  };

  log::debug!("loaded config file: {}", path.display());
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
      "unsupported config file extension: {ext}"
    ))),
  }
}

fn load_global() -> crate::Result<Value> {
  if let Ok(path) = GEST_CONFIG.value() {
    log::debug!("loading global config from $GEST_CONFIG: {}", path.display());
    return load_file(&path);
  }

  if let Some(config_home) = dir_spec::config_home() {
    let config_dir = config_home.join("gest");
    log::trace!("searching for global config in {}", config_dir.display());
    return find_config(&config_dir, GLOBAL_CONFIG_NAMES);
  }

  log::trace!("no global config home found");
  Ok(serde_json::json!({}))
}

fn merge_value(base: &mut Value, overlay: Value) {
  match (base, overlay) {
    (Value::Object(base_map), Value::Object(overlay_map)) => {
      for (key, value) in overlay_map {
        let entry = base_map.entry(key).or_insert(Value::Null);
        merge_value(entry, value);
      }
    }
    (base, overlay) => {
      *base = overlay;
    }
  }
}

fn walk_up_for_dir(start: &Path, name: &str) -> Option<std::path::PathBuf> {
  let mut current = start.to_path_buf();
  loop {
    if current.join(name).is_dir() {
      return Some(current);
    }
    if !current.pop() {
      return None;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod load_file {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_parses_json() {
      let tmp = tempfile::tempdir().unwrap();
      let path = tmp.path().join("config.json");
      std::fs::write(&path, r#"{"harness": {"command": "test"}}"#).unwrap();

      let value = load_file(&path).unwrap();
      assert_eq!(value["harness"]["command"], "test");
    }

    #[test]
    fn it_parses_toml() {
      let tmp = tempfile::tempdir().unwrap();
      let path = tmp.path().join("config.toml");
      std::fs::write(&path, "[harness]\ncommand = \"test\"").unwrap();

      let value = load_file(&path).unwrap();
      assert_eq!(value["harness"]["command"], "test");
    }

    #[test]
    fn it_parses_yaml() {
      let tmp = tempfile::tempdir().unwrap();
      let path = tmp.path().join("config.yaml");
      std::fs::write(&path, "harness:\n  command: test").unwrap();

      let value = load_file(&path).unwrap();
      assert_eq!(value["harness"]["command"], "test");
    }

    #[test]
    fn it_returns_empty_object_when_not_found() {
      let value = load_file(Path::new("/nonexistent/config.toml")).unwrap();
      assert_eq!(value, serde_json::json!({}));
    }

    #[test]
    fn it_returns_error_for_unsupported_extension() {
      let tmp = tempfile::tempdir().unwrap();
      let path = tmp.path().join("config.xml");
      std::fs::write(&path, "<config/>").unwrap();

      let result = load_file(&path);
      assert!(result.is_err());
    }
  }

  mod merge_value {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn it_adds_new_keys() {
      let mut base = json!({"a": 1});
      merge_value(&mut base, json!({"b": 2}));
      assert_eq!(base, json!({"a": 1, "b": 2}));
    }

    #[test]
    fn it_deep_merges_nested_objects() {
      let mut base = json!({"harness": {"command": "claude"}});
      merge_value(&mut base, json!({"harness": {"args": ["--flag"]}}));
      assert_eq!(base, json!({"harness": {"command": "claude", "args": ["--flag"]}}));
    }

    #[test]
    fn it_overwrites_scalars() {
      let mut base = json!({"harness": {"command": "claude"}});
      merge_value(&mut base, json!({"harness": {"command": "other"}}));
      assert_eq!(base, json!({"harness": {"command": "other"}}));
    }
  }
}
