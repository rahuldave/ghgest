use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_cascades_cancel_to_open_tasks() {
  let env = GestCmd::new();
  let iter_id = env.create_iteration("Sprint 1");
  let task_id = env.create_task("Task A");

  env.run(&["iteration", "add", &iter_id, &task_id]);

  env
    .cmd()
    .args(["iteration", "update", &iter_id, "--status", "cancelled"])
    .assert()
    .success();

  env
    .cmd()
    .args(["task", "show", &task_id, "--json"])
    .assert()
    .success()
    .stdout(predicate::str::contains("\"status\": \"cancelled\""));
}

#[test]
fn it_cascades_cancel_to_in_progress_tasks() {
  let env = GestCmd::new();
  let iter_id = env.create_iteration("Sprint 1");
  let task_id = env.create_task("Task A");

  env.run(&["iteration", "add", &iter_id, &task_id]);
  env.run(&["task", "update", &task_id, "--status", "in-progress"]);

  env
    .cmd()
    .args(["iteration", "update", &iter_id, "--status", "cancelled"])
    .assert()
    .success();

  env
    .cmd()
    .args(["task", "show", &task_id, "--json"])
    .assert()
    .success()
    .stdout(predicate::str::contains("\"status\": \"cancelled\""));
}

#[test]
fn it_leaves_done_tasks_unchanged_on_cancel() {
  let env = GestCmd::new();
  let iter_id = env.create_iteration("Sprint 1");
  let task_id = env.create_task("Task A");

  env.run(&["iteration", "add", &iter_id, &task_id]);
  env.run(&["task", "complete", &task_id]);

  env
    .cmd()
    .args(["iteration", "update", &iter_id, "--status", "cancelled"])
    .assert()
    .success();

  env
    .cmd()
    .args(["task", "show", &task_id, "--json"])
    .assert()
    .success()
    .stdout(predicate::str::contains("\"status\": \"done\""));
}

#[test]
fn it_reopens_cancelled_tasks_on_iteration_reopen() {
  let env = GestCmd::new();
  let iter_id = env.create_iteration("Sprint 1");
  let task_id = env.create_task("Task A");

  env.run(&["iteration", "add", &iter_id, &task_id]);

  // Cancel then reopen
  env.run(&["iteration", "update", &iter_id, "--status", "cancelled"]);
  env.run(&["iteration", "update", &iter_id, "--status", "active"]);

  env
    .cmd()
    .args(["task", "show", &task_id, "--json"])
    .assert()
    .success()
    .stdout(predicate::str::contains("\"status\": \"open\""));
}

#[test]
fn it_leaves_done_tasks_unchanged_on_reopen() {
  let env = GestCmd::new();
  let iter_id = env.create_iteration("Sprint 1");
  let task_open = env.create_task("Open Task");
  let task_done = env.create_task("Done Task");

  env.run(&["iteration", "add", &iter_id, &task_open]);
  env.run(&["iteration", "add", &iter_id, &task_done]);
  env.run(&["task", "complete", &task_done]);

  // Cancel then reopen
  env.run(&["iteration", "update", &iter_id, "--status", "cancelled"]);
  env.run(&["iteration", "update", &iter_id, "--status", "active"]);

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
