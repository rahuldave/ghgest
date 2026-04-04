use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_adds_a_task_to_iteration() {
  let env = GestCmd::new();
  let iter_id = env.create_iteration("Sprint 1");
  let task_id = env.create_task("Implement feature");

  env
    .cmd()
    .args(["iteration", "add", &iter_id, &task_id])
    .assert()
    .success()
    .stdout(predicate::str::contains("Added task"));
}
