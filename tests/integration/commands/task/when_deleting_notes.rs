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
fn it_deletes_note_by_id() {
  let g = GestCmd::new();
  let task_id = g.create_task("notable");
  let note_id = add_note(&g, &task_id, "doomed body");

  let output = g
    .cmd()
    .args(["task", "note", "delete", &note_id, "--yes"])
    .output()
    .expect("task note delete failed");
  assert!(
    output.status.success(),
    "task note delete should succeed: {}",
    String::from_utf8_lossy(&output.stderr)
  );

  let show = g
    .cmd()
    .args(["task", "note", "show", &note_id])
    .output()
    .expect("task note show failed");
  assert!(!show.status.success(), "deleted note should not be viewable");
}

#[test]
fn it_errors_on_missing_note() {
  let g = GestCmd::new();

  let output = g
    .cmd()
    .args(["task", "note", "delete", "zzzzzzzz", "--yes"])
    .output()
    .expect("task note delete failed");

  assert!(!output.status.success());
}
