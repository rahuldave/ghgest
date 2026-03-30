use chrono::Utc;
use clap::Args;

use crate::{
  cli::{self, AppContext},
  store,
  ui::composites::success_message::SuccessMessage,
};

/// Set a metadata value on a task using a dot-delimited key path.
#[derive(Debug, Args)]
pub struct Command {
  /// Task ID or unique prefix.
  pub id: String,
  /// Dot-delimited key path (e.g. `outer.inner`).
  pub path: String,
  /// Value to set (strings, numbers, and booleans are auto-detected).
  pub value: String,
}

impl Command {
  /// Write the value into the task's metadata table and persist.
  pub fn call(&self, ctx: &AppContext) -> cli::Result<()> {
    let data_dir = &ctx.data_dir;
    let theme = &ctx.theme;
    let id = store::resolve_task_id(data_dir, &self.id, false)?;
    let mut task = store::read_task(data_dir, &id)?;

    set_dot_path(&mut task.metadata, &self.path, &self.value)?;

    task.updated_at = Utc::now();
    store::write_task(data_dir, &task)?;

    let msg = format!("Set {}.{} = {}", id, self.path, self.value);
    println!("{}", SuccessMessage::new(&msg, theme));
    Ok(())
  }
}

/// Parse a string into the most specific TOML scalar type (int, float, bool, or string).
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

/// Insert a value at a dot-delimited path, creating intermediate tables as needed.
fn set_dot_path(table: &mut toml::Table, path: &str, value: &str) -> cli::Result<()> {
  let segments: Vec<&str> = path.split('.').collect();
  let toml_value = parse_toml_value(value);

  if segments.len() == 1 {
    table.insert(segments[0].to_string(), toml_value);
    return Ok(());
  }

  set_nested(table, &segments, toml_value);
  Ok(())
}

/// Recursively descend into (or create) nested tables and insert the value at the final segment.
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

#[cfg(test)]
mod tests {
  use super::*;

  mod call {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::test_helpers::{make_test_context, make_test_task};

    #[test]
    fn it_sets_metadata_value() {
      let dir = tempfile::tempdir().unwrap();
      let ctx = make_test_context(dir.path());
      let task = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      store::write_task(&ctx.data_dir, &task).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        path: "priority".to_string(),
        value: "high".to_string(),
      };
      cmd.call(&ctx).unwrap();

      let loaded = store::read_task(&ctx.data_dir, &task.id).unwrap();
      assert_eq!(
        loaded.metadata.get("priority"),
        Some(&toml::Value::String("high".to_string()))
      );
    }

    #[test]
    fn it_sets_nested_metadata_value() {
      let dir = tempfile::tempdir().unwrap();
      let ctx = make_test_context(dir.path());
      let task = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      store::write_task(&ctx.data_dir, &task).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        path: "config.timeout".to_string(),
        value: "30".to_string(),
      };
      cmd.call(&ctx).unwrap();

      let loaded = store::read_task(&ctx.data_dir, &task.id).unwrap();
      let config = loaded.metadata.get("config").unwrap().as_table().unwrap();
      assert_eq!(config.get("timeout"), Some(&toml::Value::Integer(30)));
    }
  }

  mod parse_toml_value {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_falls_back_to_string() {
      assert_eq!(parse_toml_value("hello"), toml::Value::String("hello".to_string()));
    }

    #[test]
    fn it_parses_booleans() {
      assert_eq!(parse_toml_value("true"), toml::Value::Boolean(true));
      assert_eq!(parse_toml_value("false"), toml::Value::Boolean(false));
    }

    #[test]
    fn it_parses_floats() {
      assert_eq!(parse_toml_value("3.14"), toml::Value::Float(3.14));
    }

    #[test]
    fn it_parses_integers() {
      assert_eq!(parse_toml_value("42"), toml::Value::Integer(42));
    }
  }
}
