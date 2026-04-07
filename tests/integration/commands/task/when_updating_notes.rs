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
fn it_updates_a_note_body() {
  let g = GestCmd::new();
  let task_id = g.create_task("notable");
  let note_id = add_note(&g, &task_id, "original body");

  let output = g
    .cmd()
    .args(["task", "note", "update", &note_id, "-b", "updated body"])
    .output()
    .expect("task note update failed");

  assert!(
    output.status.success(),
    "task note update should succeed: {}",
    String::from_utf8_lossy(&output.stderr)
  );

  let show = g
    .cmd()
    .args(["task", "note", "show", &note_id])
    .output()
    .expect("task note show failed");
  let stdout = String::from_utf8_lossy(&show.stdout);
  assert!(stdout.contains("updated body"), "got: {stdout}");
}

#[test]
fn it_updates_a_note_body_as_json() {
  let g = GestCmd::new();
  let task_id = g.create_task("notable");
  let note_id = add_note(&g, &task_id, "original body");

  let output = g
    .cmd()
    .args(["task", "note", "update", &note_id, "-b", "updated via json", "--json"])
    .output()
    .expect("task note update --json failed");

  assert!(output.status.success());
  let stdout = String::from_utf8_lossy(&output.stdout);
  let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid json");
  assert_eq!(parsed["body"].as_str(), Some("updated via json"));
}
