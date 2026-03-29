use clap::Args;

use crate::{config, config::Config, store};

/// Get a metadata value from an iteration
#[derive(Debug, Args)]
pub struct Command {
  /// Iteration ID or unique prefix
  pub id: String,
  /// Dot-delimited key path (e.g. outer.inner)
  pub path: String,
}

impl Command {
  pub fn call(&self, config: &Config) -> crate::Result<()> {
    let data_dir = config::data_dir(config)?;
    let id = store::resolve_iteration_id(&data_dir, &self.id, false)?;
    let iteration = store::read_iteration(&data_dir, &id)?;

    let value = resolve_dot_path(&toml::Value::Table(iteration.metadata), &self.path)
      .ok_or_else(|| crate::Error::generic(format!("Metadata key not found: '{}'", self.path)))?;

    print_toml_value(&value);
    Ok(())
  }
}

fn print_toml_value(value: &toml::Value) {
  match value {
    toml::Value::String(s) => println!("{s}"),
    toml::Value::Boolean(b) => println!("{b}"),
    toml::Value::Integer(n) => println!("{n}"),
    toml::Value::Float(n) => println!("{n}"),
    toml::Value::Datetime(dt) => println!("{dt}"),
    toml::Value::Array(arr) => {
      let json = serde_json::to_string_pretty(arr).unwrap_or_else(|_| format!("{arr:?}"));
      println!("{json}");
    }
    toml::Value::Table(t) => {
      let json = serde_json::to_string_pretty(t).unwrap_or_else(|_| format!("{t:?}"));
      println!("{json}");
    }
  }
}

fn resolve_dot_path(root: &toml::Value, path: &str) -> Option<toml::Value> {
  let segments: Vec<&str> = path.split('.').collect();
  let mut current = root.clone();

  for segment in &segments {
    match current {
      toml::Value::Table(t) => {
        current = t.get(*segment)?.clone();
      }
      _ => return None,
    }
  }

  Some(current)
}
