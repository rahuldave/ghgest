use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_errors_on_nonexistent() {
  let env = GestCmd::new();

  env
    .cmd()
    .args(["iteration", "show", "doesnotexist00000000000000000000"])
    .assert()
    .failure();
}

#[test]
fn it_shows_an_iteration() {
  let env = GestCmd::new();
  let id = env.create_iteration("Sprint 1");

  env
    .cmd()
    .args(["iteration", "show", &id])
    .assert()
    .success()
    .stdout(predicate::str::contains("Sprint 1"));
}
