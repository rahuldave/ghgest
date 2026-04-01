use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_reads_back_an_integer_via_config_get() {
  let env = GestCmd::new();

  env
    .cmd()
    .args(["config", "set", "serve.port", "8080"])
    .assert()
    .success();

  env
    .cmd()
    .args(["config", "get", "serve.port"])
    .assert()
    .success()
    .stdout(predicate::str::contains("8080"));
}

#[test]
fn it_writes_a_boolean_false_not_a_string() {
  let env = GestCmd::new();

  env
    .cmd()
    .args(["config", "set", "serve.open", "false"])
    .assert()
    .success();

  let content = env.read_data_file("config.toml");
  assert!(
    content.contains("open = false"),
    "expected 'open = false' (boolean) in config file, got:\n{content}"
  );
}

#[test]
fn it_writes_a_boolean_not_a_string_for_true_and_false() {
  let env = GestCmd::new();

  env
    .cmd()
    .args(["config", "set", "serve.open", "true"])
    .assert()
    .success();

  let content = env.read_data_file("config.toml");
  assert!(
    content.contains("open = true"),
    "expected 'open = true' (boolean) in config file, got:\n{content}"
  );
}

#[test]
fn it_writes_a_string_for_non_numeric_non_boolean_values() {
  let env = GestCmd::new();

  env
    .cmd()
    .args(["config", "set", "log.level", "debug"])
    .assert()
    .success();

  let content = env.read_data_file("config.toml");
  assert!(
    content.contains("level = \"debug\""),
    "expected 'level = \"debug\"' (string) in config file, got:\n{content}"
  );
}

#[test]
fn it_writes_an_integer_not_a_string_for_numeric_values() {
  let env = GestCmd::new();

  env
    .cmd()
    .args(["config", "set", "serve.port", "8080"])
    .assert()
    .success();

  let content = env.read_data_file("config.toml");
  assert!(
    content.contains("port = 8080"),
    "expected 'port = 8080' (integer) in config file, got:\n{content}"
  );
}
