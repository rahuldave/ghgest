#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("{0}")]
  Generic(String),
  #[error("I/O error: {0}")]
  Io(#[from] std::io::Error),
  #[error("JSON error: {0}")]
  Json(#[from] serde_json::Error),
  #[error("Self-update error: {0}")]
  SelfUpdate(#[from] self_update::errors::Error),
  #[error("TOML deserialization error: {0}")]
  TomlDeserialize(#[from] toml::de::Error),
  #[error("TOML serialization error: {0}")]
  TomlSerialize(#[from] toml::ser::Error),
  #[error("YAML error: {0}")]
  Yaml(#[from] yaml_serde::Error),
}

impl Error {
  pub fn generic(msg: impl Into<String>) -> Self {
    Self::Generic(msg.into())
  }
}

pub type Result<T> = std::result::Result<T, Error>;
