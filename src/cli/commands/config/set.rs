use std::{env, fs, path::PathBuf};

use clap::Args;
use toml::{Table, Value};

use crate::{
  AppContext,
  cli::Error,
  config::{self, env::GEST_CONFIG},
  ui::components::SuccessMessage,
};

/// Set a configuration value.
#[derive(Args, Debug)]
pub struct Command {
  /// Dot-delimited config key (e.g. `log.level`).
  key: String,
  /// Value to assign; auto-parsed as bool, int, float, or string.
  value: String,
  /// Write to the global (user-level) config instead of the project config.
  #[arg(short, long)]
  global: bool,
}

impl Command {
  pub async fn call(&self, _context: &AppContext) -> Result<(), Error> {
    let (scope_label, config_path) = if self.global {
      ("global", resolve_global_config_path()?)
    } else {
      ("project", resolve_project_config_path()?)
    };

    let mut root: Table = if config_path.is_file() {
      let content = fs::read_to_string(&config_path)?;
      toml::from_str(&content).map_err(config::Error::from)?
    } else {
      Table::new()
    };

    set_dot_path(&mut root, &self.key, parse_scalar(&self.value));

    if let Some(parent) = config_path.parent() {
      fs::create_dir_all(parent)?;
    }

    let content = toml::to_string_pretty(&Value::Table(root))?;
    fs::write(&config_path, content)?;

    let message = SuccessMessage::new("set config")
      .field("scope", scope_label)
      .field("key", &self.key)
      .field("value", &self.value)
      .field("path", config_path.display().to_string());
    println!("{message}");

    Ok(())
  }
}

/// Parse a raw string into the most specific TOML scalar type.
fn parse_scalar(s: &str) -> Value {
  match s {
    "true" => return Value::Boolean(true),
    "false" => return Value::Boolean(false),
    _ => {}
  }
  if let Ok(n) = s.parse::<i64>() {
    return Value::Integer(n);
  }
  if let Ok(n) = s.parse::<f64>()
    && n.is_finite()
  {
    return Value::Float(n);
  }
  Value::String(s.to_string())
}

/// Resolve the global config file path, honoring `$GEST_CONFIG`.
fn resolve_global_config_path() -> Result<PathBuf, Error> {
  GEST_CONFIG
    .value()
    .ok()
    .or_else(|| dir_spec::config_home().map(|path| path.join("gest/config.toml")))
    .ok_or_else(|| config::Error::XDGDirNotFound("config").into())
}

/// Resolve the project config file path by walking ancestors of `$CWD`.
///
/// Returns the most-local existing `.gest.toml` or `.config/gest.toml`; falls
/// back to `$CWD/.gest.toml` if no existing file is found.
fn resolve_project_config_path() -> Result<PathBuf, Error> {
  let cwd = env::current_dir()?;
  for ancestor in cwd.ancestors() {
    let gest_toml = ancestor.join(".gest.toml");
    if gest_toml.is_file() {
      return Ok(gest_toml);
    }
    let nested = ancestor.join(".config/gest.toml");
    if nested.is_file() {
      return Ok(nested);
    }
  }
  Ok(cwd.join(".gest.toml"))
}

/// Insert `value` at a dot-delimited path, creating intermediate tables as needed.
fn set_dot_path(root: &mut Table, key: &str, value: Value) {
  let segments: Vec<&str> = key.split('.').collect();
  let Some((last, rest)) = segments.split_last() else {
    return;
  };

  let mut current = root;
  for segment in rest {
    let entry = current
      .entry((*segment).to_string())
      .or_insert_with(|| Value::Table(Table::new()));
    if !entry.is_table() {
      *entry = Value::Table(Table::new());
    }
    current = entry.as_table_mut().expect("entry was just set to a table");
  }
  current.insert((*last).to_string(), value);
}
