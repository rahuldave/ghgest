use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_untags_an_iteration() {
  let env = GestCmd::new();
  let iter_id = env.create_iteration("Sprint 1");

  env
    .cmd()
    .args(["iteration", "tag", &iter_id, "sprint", "q1"])
    .assert()
    .success();

  env
    .cmd()
    .args(["iteration", "untag", &iter_id, "sprint"])
    .assert()
    .success()
    .stdout(predicate::str::contains("Untagged iteration"));
}
