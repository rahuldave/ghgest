use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_reopens_a_cancelled_iteration() {
  let env = GestCmd::new();
  let id = env.create_iteration("Sprint 1");

  env.run(&["iteration", "cancel", &id]);

  env
    .cmd()
    .args(["iteration", "reopen", &id])
    .assert()
    .success()
    .stdout(predicate::str::contains("Reopened iteration"));
}

#[test]
fn it_restores_cancelled_tasks_to_open() {
  let env = GestCmd::new();
  let iter_id = env.create_iteration("Sprint 1");
  let task_id = env.create_task("Task A");

  env.run(&["iteration", "add", &iter_id, &task_id]);
  env.run(&["iteration", "cancel", &iter_id]);
  env.run(&["iteration", "reopen", &iter_id]);

  env
    .cmd()
    .args(["task", "show", &task_id, "--json"])
    .assert()
    .success()
    .stdout(predicate::str::contains("\"status\": \"open\""));
}

#[test]
fn it_leaves_done_tasks_unchanged() {
  let env = GestCmd::new();
  let iter_id = env.create_iteration("Sprint 1");
  let task_open = env.create_task("Open Task");
  let task_done = env.create_task("Done Task");

  env.run(&["iteration", "add", &iter_id, &task_open]);
  env.run(&["iteration", "add", &iter_id, &task_done]);
  env.run(&["task", "complete", &task_done]);

  env.run(&["iteration", "cancel", &iter_id]);
  env.run(&["iteration", "reopen", &iter_id]);

  env
    .cmd()
    .args(["task", "show", &task_open, "--json"])
    .assert()
    .success()
    .stdout(predicate::str::contains("\"status\": \"open\""));

  env
    .cmd()
    .args(["task", "show", &task_done, "--json"])
    .assert()
    .success()
    .stdout(predicate::str::contains("\"status\": \"done\""));
}

#[test]
fn it_outputs_json() {
  let env = GestCmd::new();
  let id = env.create_iteration("Sprint 1");

  env.run(&["iteration", "cancel", &id]);

  env
    .cmd()
    .args(["iteration", "reopen", &id, "--json"])
    .assert()
    .success()
    .stdout(predicate::str::contains("\"status\": \"active\""));
}
