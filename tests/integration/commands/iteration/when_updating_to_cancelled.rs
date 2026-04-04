use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_sets_cancelled_status() {
  let env = GestCmd::new();
  let id = env.create_iteration("Sprint 1");

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
  let id = env.create_iteration("Sprint 2");

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
