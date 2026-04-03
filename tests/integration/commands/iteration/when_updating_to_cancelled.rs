use predicates::prelude::*;

use crate::support::helpers::GestCmd;

fn create_iteration(env: &GestCmd, title: &str) -> String {
  let output = env
    .cmd()
    .args(["iteration", "create", title])
    .output()
    .expect("failed to run gest iteration create");

  let stdout = String::from_utf8_lossy(&output.stdout);
  stdout
    .split_whitespace()
    .last()
    .expect("no output from iteration create")
    .to_string()
}

#[test]
fn it_sets_cancelled_status() {
  let env = GestCmd::new();
  let id = create_iteration(&env, "Sprint 1");

  env
    .cmd()
    .args(["iteration", "update", &id, "--status", "cancelled"])
    .assert()
    .success()
    .stdout(predicate::str::contains("Updated iteration"));

  env
    .cmd()
    .args(["iteration", "show", &id, "--json"])
    .assert()
    .success()
    .stdout(predicate::str::contains("\"status\": \"cancelled\""));
}

#[test]
fn it_accepts_deprecated_failed_status() {
  let env = GestCmd::new();
  let id = create_iteration(&env, "Sprint 2");

  env
    .cmd()
    .args(["iteration", "update", &id, "--status", "failed"])
    .assert()
    .success()
    .stdout(predicate::str::contains("Updated iteration"));

  env
    .cmd()
    .args(["iteration", "show", &id, "--json"])
    .assert()
    .success()
    .stdout(predicate::str::contains("\"status\": \"failed\""));
}
