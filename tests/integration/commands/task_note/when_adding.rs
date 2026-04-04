use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_adds_a_note() {
  let env = GestCmd::new();
  let task_id = env.create_task("Test Task");

  env
    .cmd()
    .args([
      "task",
      "note",
      "add",
      &task_id,
      "--agent",
      "test-agent",
      "--body",
      "My note",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("added note"));
}

#[test]
fn it_errors_on_nonexistent_task() {
  let env = GestCmd::new();

  env
    .cmd()
    .args([
      "task",
      "note",
      "add",
      "nonexistent-task-id",
      "--agent",
      "test-agent",
      "--body",
      "My note",
    ])
    .assert()
    .failure();
}
