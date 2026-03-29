use chrono::Utc;
use clap::Args;

use crate::{
  config,
  config::Config,
  store,
  ui::{components::MetadataSet, theme::Theme},
};

/// Set a metadata value on an iteration
#[derive(Debug, Args)]
pub struct Command {
  /// Iteration ID or unique prefix
  pub id: String,
  /// Dot-delimited key path (e.g. outer.inner)
  pub path: String,
  /// Value to set (strings, numbers, and booleans are auto-detected)
  pub value: String,
}

impl Command {
  pub fn call(&self, config: &Config, _theme: &Theme) -> crate::Result<()> {
    let data_dir = config::data_dir(config)?;
    let id = store::resolve_iteration_id(&data_dir, &self.id, false)?;
    let mut iteration = store::read_iteration(&data_dir, &id)?;

    set_dot_path(&mut iteration.metadata, &self.path, &self.value)?;

    iteration.updated_at = Utc::now();
    store::write_iteration(&data_dir, &iteration)?;

    MetadataSet::new(&id, &self.path, &self.value).write_to(&mut std::io::stdout())?;
    Ok(())
  }
}

fn parse_toml_value(s: &str) -> toml::Value {
  if let Ok(n) = s.parse::<i64>() {
    return toml::Value::Integer(n);
  }
  if let Ok(n) = s.parse::<f64>() {
    return toml::Value::Float(n);
  }
  match s {
    "true" => toml::Value::Boolean(true),
    "false" => toml::Value::Boolean(false),
    _ => toml::Value::String(s.to_string()),
  }
}

fn set_dot_path(table: &mut toml::Table, path: &str, value: &str) -> crate::Result<()> {
  let segments: Vec<&str> = path.split('.').collect();
  let toml_value = parse_toml_value(value);

  if segments.len() == 1 {
    table.insert(segments[0].to_string(), toml_value);
    return Ok(());
  }

  set_nested(table, &segments, toml_value);
  Ok(())
}

fn set_nested(table: &mut toml::Table, segments: &[&str], value: toml::Value) {
  let key = segments[0].to_string();

  if segments.len() == 1 {
    table.insert(key, value);
    return;
  }

  let nested = table
    .entry(&key)
    .or_insert_with(|| toml::Value::Table(toml::Table::new()));

  if let toml::Value::Table(t) = nested {
    set_nested(t, &segments[1..], value);
  } else {
    let mut new_table = toml::Table::new();
    set_nested(&mut new_table, &segments[1..], value);
    table.insert(key, toml::Value::Table(new_table));
  }
}
