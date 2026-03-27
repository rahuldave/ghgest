use predicates::prelude::*;

use crate::support::helpers::GestCmd;

/// Extract the 8-character task ID from create output like "Created task abcdefgh".
fn extract_task_id(create_output: &str) -> String {
  create_output
    .trim()
    .strip_prefix("Created task ")
    .expect("expected 'Created task <id>' output")
    .to_string()
}

#[test]
fn it_updates_description_via_flag() {
  let env = GestCmd::new();

  let output = env
    .run(["task", "create", "Update Desc", "-d", "original"])
    .output()
    .expect("failed to create task");
  let id = extract_task_id(&String::from_utf8_lossy(&output.stdout));

  env
    .run(["task", "update", &id, "-d", "updated description"])
    .assert()
    .success()
    .stdout(predicate::str::contains("Updated task"));

  // Verify the change persisted
  let show_output = env
    .run(["task", "show", &id, "--json"])
    .output()
    .expect("failed to show task");
  let stdout = String::from_utf8_lossy(&show_output.stdout);
  let json: serde_json::Value = serde_json::from_str(&stdout).expect("invalid JSON");
  assert_eq!(json["description"], "updated description");
}

#[test]
fn it_updates_status() {
  let env = GestCmd::new();

  let output = env
    .run(["task", "create", "Status Task", "-d", "check status"])
    .output()
    .expect("failed to create task");
  let id = extract_task_id(&String::from_utf8_lossy(&output.stdout));

  env
    .run(["task", "update", &id, "-s", "in-progress"])
    .assert()
    .success()
    .stdout(predicate::str::contains("Updated task"));

  // Verify status changed
  let show_output = env
    .run(["task", "show", &id, "--json"])
    .output()
    .expect("failed to show task");
  let stdout = String::from_utf8_lossy(&show_output.stdout);
  let json: serde_json::Value = serde_json::from_str(&stdout).expect("invalid JSON");
  assert_eq!(json["status"], "in-progress");
}

#[test]
fn it_transitions_open_to_done_and_back() {
  let env = GestCmd::new();

  let output = env
    .run(["task", "create", "Lifecycle Task", "-d", "test lifecycle"])
    .output()
    .expect("failed to create task");
  let id = extract_task_id(&String::from_utf8_lossy(&output.stdout));

  // open -> done
  env.run(["task", "update", &id, "-s", "done"]).assert().success();

  let show = env
    .run(["task", "show", &id, "--json"])
    .output()
    .expect("failed to show task");
  let json: serde_json::Value = serde_json::from_str(&String::from_utf8_lossy(&show.stdout)).unwrap();
  assert_eq!(json["status"], "done");
  assert!(json["resolved_at"].is_string(), "resolved_at should be set");

  // done -> open
  env.run(["task", "update", &id, "-s", "open"]).assert().success();

  let show2 = env
    .run(["task", "show", &id, "--json"])
    .output()
    .expect("failed to show task");
  let json2: serde_json::Value = serde_json::from_str(&String::from_utf8_lossy(&show2.stdout)).unwrap();
  assert_eq!(json2["status"], "open");
  assert_eq!(json2["resolved_at"], "", "resolved_at should be cleared");
}

#[test]
fn it_persists_updates_across_reads() {
  let env = GestCmd::new();

  let output = env
    .run(["task", "create", "Persist Task", "-d", "original"])
    .output()
    .expect("failed to create task");
  let id = extract_task_id(&String::from_utf8_lossy(&output.stdout));

  env
    .run(["task", "update", &id, "-t", "Renamed Task"])
    .assert()
    .success();

  // Verify via show
  let show_output = env
    .run(["task", "show", &id, "--json"])
    .output()
    .expect("failed to show task");
  let stdout = String::from_utf8_lossy(&show_output.stdout);
  let json: serde_json::Value = serde_json::from_str(&stdout).expect("invalid JSON");
  assert_eq!(json["title"], "Renamed Task");

  // Verify via list
  env
    .run(["task", "list"])
    .assert()
    .success()
    .stdout(predicate::str::contains("Renamed Task"));
}
