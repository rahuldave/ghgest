use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_updates_iteration_status() {
  let env = GestCmd::new();
  let id = env.create_iteration("Sprint 1");

  env
    .cmd()
    .args(["iteration", "update", &id, "--status", "completed"])
    .assert()
    .success()
    .stdout(predicate::str::contains("Updated iteration"));
}

#[test]
fn it_updates_iteration_title() {
  let env = GestCmd::new();
  let id = env.create_iteration("Old Title");

  env
    .cmd()
    .args(["iteration", "update", &id, "--title", "New Title"])
    .assert()
    .success()
    .stdout(predicate::str::contains("Updated iteration"));

  env
    .cmd()
    .args(["iteration", "show", &id])
    .assert()
    .success()
    .stdout(predicate::str::contains("New Title"));
}
