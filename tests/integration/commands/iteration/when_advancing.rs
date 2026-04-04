use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_advances_phase() {
  let env = GestCmd::new();
  let iter_id = env.create_iteration("Sprint 1");

  // Create two tasks in different phases
  let task1_id = env.create_task("Phase 1 task");
  let task2_id = env.create_task("Phase 2 task");

  // Assign tasks to phases
  env
    .cmd()
    .args(["task", "update", &task1_id, "--phase", "1"])
    .assert()
    .success();

  env
    .cmd()
    .args(["task", "update", &task2_id, "--phase", "2"])
    .assert()
    .success();

  // Add both tasks to the iteration
  env
    .cmd()
    .args(["iteration", "add", &iter_id, &task1_id])
    .assert()
    .success();

  env
    .cmd()
    .args(["iteration", "add", &iter_id, &task2_id])
    .assert()
    .success();

  // Force advance the iteration (phase 1 still has an open task, but we force it)
  env
    .cmd()
    .args(["iteration", "advance", &iter_id, "--force"])
    .assert()
    .success()
    .stdout(predicate::str::contains("Advanced iteration").or(predicate::str::contains("All phases complete")));
}
