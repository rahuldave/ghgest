use crate::support::helpers::GestCmd;

#[test]
fn it_updates_assigned() {
  let g = GestCmd::new();
  let task_id = g.create_task("assignable task");

  g.cmd()
    .args(["task", "update", &task_id, "--assigned-to", "alice"])
    .assert()
    .success();

  let show = g
    .cmd()
    .args(["task", "show", &task_id, "--json"])
    .output()
    .expect("task show failed");
  let stdout = String::from_utf8_lossy(&show.stdout);
  let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid json");
  assert!(
    parsed["assigned_to"].is_string(),
    "assigned_to should be set after update, got: {stdout}"
  );
}

#[test]
fn it_updates_body() {
  let g = GestCmd::new();
  let task_id = g.create_task("describable task");

  g.cmd()
    .args(["task", "update", &task_id, "--description", "the full story"])
    .assert()
    .success();

  let show = g
    .cmd()
    .args(["task", "show", &task_id])
    .output()
    .expect("task show failed");
  let stdout = String::from_utf8_lossy(&show.stdout);
  assert!(stdout.contains("the full story"), "got: {stdout}");
}

#[test]
fn it_updates_priority() {
  let g = GestCmd::new();
  let task_id = g.create_task("prioritizable task");

  g.cmd()
    .args(["task", "update", &task_id, "--priority", "3"])
    .assert()
    .success();

  let show = g
    .cmd()
    .args(["task", "show", &task_id, "--json"])
    .output()
    .expect("task show failed");
  let stdout = String::from_utf8_lossy(&show.stdout);
  let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid json");
  assert_eq!(parsed["priority"].as_u64(), Some(3), "got: {stdout}");
}

#[test]
fn it_updates_task_status_to_done() {
  let g = GestCmd::new();
  let id = g.create_task("Finishable task");

  g.cmd()
    .args(["task", "update", &id, "--status", "done"])
    .assert()
    .success();

  let output = g.cmd().args(["task", "show", &id]).output().expect("task show failed");
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("done"), "task should be done, got: {stdout}");
}

#[test]
fn it_updates_title() {
  let g = GestCmd::new();
  let task_id = g.create_task("original title");

  g.cmd()
    .args(["task", "update", &task_id, "--title", "renamed title"])
    .assert()
    .success();

  let show = g
    .cmd()
    .args(["task", "show", &task_id])
    .output()
    .expect("task show failed");
  let stdout = String::from_utf8_lossy(&show.stdout);
  assert!(stdout.contains("renamed title"), "got: {stdout}");
}
