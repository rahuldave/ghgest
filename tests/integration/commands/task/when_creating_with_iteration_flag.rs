use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_adds_the_new_task_to_the_iteration() {
  let env = GestCmd::new();
  let iter_id = env.create_iteration("Sprint 1");

  env
    .cmd()
    .args(["task", "create", "Flag Task", "-i", &iter_id])
    .assert()
    .success();

  let output = env
    .cmd()
    .args(["iteration", "show", "--json", &iter_id])
    .output()
    .expect("failed to run gest iteration show --json");

  assert!(output.status.success(), "iteration show --json failed");

  let json: serde_json::Value =
    serde_json::from_slice(&output.stdout).expect("iteration show --json output is not valid JSON");

  let tasks = json["tasks"]
    .as_array()
    .expect("expected 'tasks' array in iteration JSON");
  let has_task_ref = tasks
    .iter()
    .any(|entry| entry.as_str().map(|s| s.starts_with("tasks/")).unwrap_or(false));

  assert!(
    has_task_ref,
    "expected a tasks/ ref in iteration tasks array, got: {tasks:?}"
  );
}

#[test]
fn it_still_outputs_created_task_confirmation() {
  let env = GestCmd::new();
  let iter_id = env.create_iteration("Sprint Output");

  env
    .cmd()
    .args(["task", "create", "Output Task", "-i", &iter_id])
    .assert()
    .success()
    .stdout(predicate::str::contains("created task"));
}
