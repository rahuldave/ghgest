use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_sets_metadata() {
  let env = GestCmd::new();
  let iter_id = env.create_iteration("Sprint 1");

  env
    .cmd()
    .args(["iteration", "meta", "set", &iter_id, "team", "backend"])
    .assert()
    .success()
    .stdout(predicate::str::contains("backend"));
}
