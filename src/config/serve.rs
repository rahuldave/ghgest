//! Web server configuration settings.

use std::net::IpAddr;

use serde::{Deserialize, Serialize};

/// Default bind address (localhost only).
const DEFAULT_BIND_ADDRESS: IpAddr = IpAddr::V4(std::net::Ipv4Addr::LOCALHOST);

/// Default port for the built-in web server.
const DEFAULT_PORT: u16 = 2300;

/// Configuration for the `[serve]` section.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Settings {
  bind_address: IpAddr,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  debounce_ms: Option<u64>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  log_level: Option<String>,
  open: bool,
  port: u16,
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      bind_address: DEFAULT_BIND_ADDRESS,
      debounce_ms: None,
      log_level: None,
      open: true,
      port: DEFAULT_PORT,
    }
  }
}

impl Settings {
  /// The IP address the server should bind to.
  pub fn bind_address(&self) -> IpAddr {
    self.bind_address
  }

  /// File watcher debounce window in milliseconds.
  ///
  /// Defaults to `2000` when not explicitly configured.
  pub fn debounce_ms(&self) -> u64 {
    self.debounce_ms.unwrap_or(2000)
  }

  /// Returns the configured serve log level string, if any.
  pub fn log_level(&self) -> Option<&str> {
    self.log_level.as_deref()
  }

  /// Whether to automatically open the browser when the server starts.
  pub fn open(&self) -> bool {
    self.open
  }

  /// The port the server should listen on.
  pub fn port(&self) -> u16 {
    self.port
  }
}

#[cfg(test)]
mod tests {
  use std::net::Ipv4Addr;

  use pretty_assertions::assert_eq;

  use super::*;

  #[test]
  fn it_defaults_to_bind_address_localhost() {
    let settings = Settings::default();

    assert_eq!(settings.bind_address(), IpAddr::V4(Ipv4Addr::LOCALHOST));
  }

  #[test]
  fn it_defaults_to_open_true() {
    let settings = Settings::default();

    assert!(settings.open());
  }

  #[test]
  fn it_defaults_to_port_2300() {
    let settings = Settings::default();

    assert_eq!(settings.port(), 2300);
  }

  #[test]
  fn it_deserializes_bind_address() {
    let toml_str = r#"bind_address = "0.0.0.0""#;
    let settings: Settings = toml::from_str(toml_str).unwrap();

    assert_eq!(settings.bind_address(), IpAddr::V4(Ipv4Addr::UNSPECIFIED));
  }

  #[test]
  fn it_deserializes_open() {
    let toml_str = "open = false";
    let settings: Settings = toml::from_str(toml_str).unwrap();

    assert!(!settings.open());
  }

  #[test]
  fn it_deserializes_port() {
    let toml_str = "port = 8080";
    let settings: Settings = toml::from_str(toml_str).unwrap();

    assert_eq!(settings.port(), 8080);
  }

  #[test]
  fn it_defaults_to_no_log_level() {
    let settings = Settings::default();

    assert_eq!(settings.log_level(), None);
  }

  #[test]
  fn it_deserializes_log_level() {
    let toml_str = r#"log_level = "debug""#;
    let settings: Settings = toml::from_str(toml_str).unwrap();

    assert_eq!(settings.log_level(), Some("debug"));
  }

  #[test]
  fn it_omits_none_log_level_on_serialize() {
    let settings = Settings::default();
    let serialized = toml::to_string(&settings).unwrap();

    assert!(!serialized.contains("log_level"));
  }

  #[test]
  fn it_defaults_to_debounce_ms_2000() {
    let settings = Settings::default();

    assert_eq!(settings.debounce_ms(), 2000);
  }

  #[test]
  fn it_deserializes_debounce_ms() {
    let toml_str = "debounce_ms = 500";
    let settings: Settings = toml::from_str(toml_str).unwrap();

    assert_eq!(settings.debounce_ms(), 500);
  }

  #[test]
  fn it_omits_none_debounce_ms_on_serialize() {
    let settings = Settings::default();
    let serialized = toml::to_string(&settings).unwrap();

    assert!(!serialized.contains("debounce_ms"));
  }

  #[test]
  fn it_round_trips_through_toml() {
    let settings = Settings {
      bind_address: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
      debounce_ms: Some(500),
      log_level: Some("debug".to_string()),
      open: false,
      port: 9090,
    };
    let serialized = toml::to_string(&settings).unwrap();
    let deserialized: Settings = toml::from_str(&serialized).unwrap();

    assert_eq!(settings, deserialized);
  }
}
