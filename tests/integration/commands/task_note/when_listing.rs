use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_lists_empty_when_no_notes() {
  let env = GestCmd::new();
  let task_id = env.create_task("Test Task");

  env
    .cmd()
    .args(["task", "note", "list", &task_id])
    .assert()
    .success()
    .stdout(predicate::str::contains("No notes on task"));
}

#[test]
fn it_lists_notes_as_json_with_short_flag() {
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
      "Short flag note",
    ])
    .assert()
    .success();

  env
    .cmd()
    .args(["task", "note", "list", &task_id, "-j"])
    .assert()
    .success()
    .stdout(predicate::str::contains("Short flag note"));
}

#[test]
fn it_lists_notes() {
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
      "First note",
    ])
    .assert()
    .success();

  env
    .cmd()
    .args(["task", "note", "list", &task_id])
    .assert()
    .success()
    .stdout(predicate::str::contains("First note"));
}
