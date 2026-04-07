use crate::support::helpers::GestCmd;

fn add_note(g: &GestCmd, task_id: &str, body: &str) -> String {
  let output = g
    .cmd()
    .args(["task", "note", "add", task_id, "-b", body, "--quiet"])
    .output()
    .expect("task note add failed");
  assert!(output.status.success());
  String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[test]
fn it_shows_a_note_by_id() {
  let g = GestCmd::new();
  let task_id = g.create_task("notable");
  let note_id = add_note(&g, &task_id, "showable body");

  let output = g
    .cmd()
    .args(["task", "note", "show", &note_id])
    .output()
    .expect("task note show failed");

  assert!(output.status.success(), "task note show should succeed");
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("showable body"), "got: {stdout}");
}

#[test]
fn it_shows_a_note_as_json() {
  let g = GestCmd::new();
  let task_id = g.create_task("notable");
  let note_id = add_note(&g, &task_id, "json body");

  let output = g
    .cmd()
    .args(["task", "note", "show", &note_id, "--json"])
    .output()
    .expect("task note show --json failed");

  assert!(output.status.success());
  let stdout = String::from_utf8_lossy(&output.stdout);
  let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid json");
  assert_eq!(parsed["body"].as_str(), Some("json body"));
}

#[test]
fn it_errors_on_missing_note() {
  let g = GestCmd::new();

  let output = g
    .cmd()
    .args(["task", "note", "show", "zzzzzzzz"])
    .output()
    .expect("task note show failed");

  assert!(!output.status.success());
}
