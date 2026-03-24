use std::{fmt, io, path::PathBuf};

use serde_json::Value;

/// Renders an entire config tree as key-value lines with source annotations.
///
/// Matches the output of `config show`.
pub struct ConfigDisplay {
  lines: Vec<ConfigValue>,
}

impl ConfigDisplay {
  pub fn new(json: &Value, sources: &[PathBuf]) -> Self {
    let mut lines = Vec::new();
    collect_values(json, "", sources, &mut lines);
    Self {
      lines,
    }
  }

  /// Write the full config display to the given writer.
  pub fn write_to(&self, w: &mut impl io::Write) -> io::Result<()> {
    for line in &self.lines {
      line.write_to(w)?;
    }
    Ok(())
  }
}

impl fmt::Display for ConfigDisplay {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut buf = Vec::new();
    self.write_to(&mut buf).map_err(|_| fmt::Error)?;
    let s = String::from_utf8(buf).map_err(|_| fmt::Error)?;
    write!(f, "{}", s.trim_end())
  }
}

/// Config value display component for `config show`.
///
/// Displays config key-value pairs with source annotations.
pub struct ConfigValue {
  key: String,
  source: String,
  value: String,
}

impl ConfigValue {
  pub fn new(key: &str, value: &str, source: &str) -> Self {
    Self {
      key: key.to_string(),
      source: source.to_string(),
      value: value.to_string(),
    }
  }

  /// Write the config value line to the given writer.
  pub fn write_to(&self, w: &mut impl io::Write) -> io::Result<()> {
    writeln!(w, "{} = {}  # source: {}", self.key, self.value, self.source)
  }
}

impl fmt::Display for ConfigValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut buf = Vec::new();
    self.write_to(&mut buf).map_err(|_| fmt::Error)?;
    let s = String::from_utf8(buf).map_err(|_| fmt::Error)?;
    write!(f, "{}", s.trim_end())
  }
}

fn collect_values(value: &Value, prefix: &str, sources: &[PathBuf], out: &mut Vec<ConfigValue>) {
  match value {
    Value::Object(map) => {
      for (key, val) in map {
        let full_key = if prefix.is_empty() {
          key.clone()
        } else {
          format!("{prefix}.{key}")
        };
        collect_values(val, &full_key, sources, out);
      }
    }
    _ => {
      let display_value = match value {
        Value::String(s) => format!("\"{s}\""),
        Value::Null => "null".to_string(),
        other => other.to_string(),
      };
      let source = find_source_for_key(prefix, sources);
      out.push(ConfigValue::new(prefix, &display_value, &source));
    }
  }
}

fn find_source_for_key(key: &str, sources: &[PathBuf]) -> String {
  for source in sources {
    if let Ok(content) = std::fs::read_to_string(source) {
      let ext = source.extension().and_then(|e| e.to_str()).unwrap_or("");
      let json: Option<Value> = match ext {
        "json" => serde_json::from_str(&content).ok(),
        "toml" => {
          let toml_val: Option<toml::Value> = toml::from_str(&content).ok();
          toml_val.and_then(|v| {
            let s = serde_json::to_string(&v).ok()?;
            serde_json::from_str(&s).ok()
          })
        }
        "yaml" | "yml" => yaml_serde::from_str(&content).ok(),
        _ => None,
      };

      if json.is_some_and(|j| has_key(&j, key)) {
        return source.display().to_string();
      }
    }
  }

  "default".to_string()
}

fn has_key(value: &Value, key: &str) -> bool {
  let mut current = value;
  for segment in key.split('.') {
    match current.get(segment) {
      Some(v) => current = v,
      None => return false,
    }
  }
  !current.is_null()
}

#[cfg(test)]
mod tests {
  use super::*;

  mod config_display {
    use super::*;

    mod display {
      use super::*;

      #[test]
      fn it_delegates_to_write_to() {
        let json = serde_json::json!({"key": "value"});
        let display_component = ConfigDisplay::new(&json, &[]);
        let display = display_component.to_string();
        assert!(display.contains("key = \"value\""));
      }
    }

    mod write_to {
      use super::*;

      #[test]
      fn it_defaults_source_when_no_files() {
        let json = serde_json::json!({"key": "value"});
        let display = ConfigDisplay::new(&json, &[]);
        let mut buf = Vec::new();
        display.write_to(&mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("# source: default"), "Should show default source");
      }

      #[test]
      fn it_renders_nested_json_as_flat_keys() {
        let json = serde_json::json!({
          "harness": {
            "command": "claude"
          }
        });
        let display = ConfigDisplay::new(&json, &[]);
        let mut buf = Vec::new();
        display.write_to(&mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(
          output.contains("harness.command = \"claude\""),
          "Should flatten nested keys"
        );
      }

      #[test]
      fn it_renders_null_values() {
        let json = serde_json::json!({
          "storage": {
            "data_dir": null
          }
        });
        let display = ConfigDisplay::new(&json, &[]);
        let mut buf = Vec::new();
        display.write_to(&mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("storage.data_dir = null"), "Should render null");
      }
    }
  }

  mod config_value {
    use super::*;

    mod display {
      use super::*;

      #[test]
      fn it_delegates_to_write_to() {
        let cv = ConfigValue::new("storage.data_dir", "null", "default");
        let display = cv.to_string();
        assert!(display.contains("storage.data_dir = null  # source: default"));
      }
    }

    mod write_to {
      use super::*;

      #[test]
      fn it_writes_key_value_and_source() {
        let cv = ConfigValue::new("harness.command", "\"claude\"", "default");
        let mut buf = Vec::new();
        cv.write_to(&mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("harness.command = \"claude\"  # source: default"));
      }
    }
  }
}
