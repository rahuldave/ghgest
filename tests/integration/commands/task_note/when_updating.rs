use predicates::prelude::*;

use crate::support::helpers::GestCmd;

fn add_note_and_get_id(env: &GestCmd, task_id: &str, body: &str) -> String {
  env
    .cmd()
    .args(["task", "note", "add", task_id, "--agent", "test-agent", "--body", body])
    .assert()
    .success();

  let output = env
    .cmd()
    .args(["task", "note", "list", task_id, "--json"])
    .output()
    .expect("failed to run note list");

  let notes: serde_json::Value = serde_json::from_slice(&output.stdout).expect("failed to parse note list JSON");

  notes[0]["id"].as_str().expect("note id not found in JSON").to_string()
}

#[test]
fn it_updates_a_note() {
  let env = GestCmd::new();
  let task_id = env.create_task("Test Task");
  let note_id = add_note_and_get_id(&env, &task_id, "Original content");

  env
    .cmd()
    .args([
      "task",
      "note",
      "update",
      &task_id,
      &note_id,
      "--body",
      "Updated content",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("updated note"));

  env
    .cmd()
    .args(["task", "note", "show", &task_id, &note_id])
    .assert()
    .success()
    .stdout(predicate::str::contains("Updated content"));
}
