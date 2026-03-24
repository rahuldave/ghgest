mod color_value;
mod discovery;
pub mod env;
mod loader;

use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

pub use self::{color_value::ColorValue, discovery::data_dir, loader::load};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
  #[serde(default)]
  pub colors: HashMap<String, ColorValue>,
  #[serde(default)]
  pub harness: HarnessConfig,
  #[serde(default)]
  pub log: LogConfig,
  #[serde(default)]
  pub storage: StorageConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HarnessConfig {
  #[serde(default = "default_harness_command")]
  pub command: String,
}

impl Default for HarnessConfig {
  fn default() -> Self {
    Self {
      command: default_harness_command(),
    }
  }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LogConfig {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub level: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct StorageConfig {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub data_dir: Option<PathBuf>,
}

fn default_harness_command() -> String {
  "claude".to_string()
}
