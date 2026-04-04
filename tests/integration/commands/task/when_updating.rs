use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_updates_task_status() {
  let env = GestCmd::new();
  let id = env.create_task("Status task");

  env.run(&["task", "update", &id, "--status", "done"]).success();
}

#[test]
fn it_updates_task_title() {
  let env = GestCmd::new();
  let id = env.create_task("Original title");

  env.run(&["task", "update", &id, "--title", "New title"]).success();

  env
    .run(&["task", "show", &id])
    .success()
    .stdout(predicate::str::contains("New title"));
}

#[test]
fn it_outputs_json_with_updated_title() {
  let env = GestCmd::new();
  let id = env.create_task("JSON title");

  let output = env
    .cmd()
    .args(["task", "update", &id, "--title", "Updated JSON title", "--json"])
    .output()
    .expect("failed to run gest task update --json");

  assert!(output.status.success(), "task update --json failed");

  let stdout = String::from_utf8(output.stdout).expect("stdout is not valid utf8");
  let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("output is not valid JSON");

  assert_eq!(parsed["title"].as_str().unwrap(), "Updated JSON title");
}

#[test]
fn it_outputs_bare_id_with_quiet_flag() {
  let env = GestCmd::new();
  let id = env.create_task("Quiet task");

  let output = env
    .cmd()
    .args(["task", "update", &id, "--title", "Quiet updated", "-q"])
    .output()
    .expect("failed to run gest task update -q");

  assert!(output.status.success(), "task update -q failed");

  let stdout = String::from_utf8(output.stdout).expect("stdout is not valid utf8");
  let trimmed = stdout.trim();

  assert!(
    trimmed.chars().all(|c| c.is_ascii_lowercase()),
    "quiet output should be a bare ID, got: {trimmed}"
  );
}
