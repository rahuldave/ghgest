use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_cancels_an_iteration() {
  let env = GestCmd::new();
  let id = env.create_iteration("Sprint 1");

  env
    .cmd()
    .args(["iteration", "cancel", &id])
    .assert()
    .success()
    .stdout(predicate::str::contains("Cancelled iteration"));
}

#[test]
fn it_cascades_to_tasks() {
  let env = GestCmd::new();
  let iter_id = env.create_iteration("Sprint 1");
  let task_id = env.create_task("Task A");

  env.run(&["iteration", "add", &iter_id, &task_id]);

  env.run(&["iteration", "cancel", &iter_id]);

  env
    .cmd()
    .args(["task", "show", &task_id, "--json"])
    .assert()
    .success()
    .stdout(predicate::str::contains("\"status\": \"cancelled\""));
}

#[test]
fn it_outputs_json() {
  let env = GestCmd::new();
  let id = env.create_iteration("Sprint 1");

  env
    .cmd()
    .args(["iteration", "cancel", &id, "--json"])
    .assert()
    .success()
    .stdout(predicate::str::contains("\"status\": \"cancelled\""));
}

#[test]
fn it_outputs_quiet() {
  let env = GestCmd::new();
  let id = env.create_iteration("Sprint 1");

  let output = env
    .cmd()
    .args(["iteration", "cancel", &id, "--quiet"])
    .output()
    .expect("failed to run cancel");

  let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
  assert!(!stdout.is_empty());
}
