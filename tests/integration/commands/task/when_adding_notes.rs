use crate::support::helpers::GestCmd;

#[test]
fn it_adds_a_note_via_body_flag() {
  let g = GestCmd::new();
  let id = g.create_task("Notable task");

  let output = g
    .cmd()
    .args(["task", "note", "add", &id, "-b", "first note body"])
    .output()
    .expect("task note add failed to run");

  assert!(
    output.status.success(),
    "task note add exited non-zero: {}",
    String::from_utf8_lossy(&output.stderr)
  );
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("added note"), "got: {stdout}");
}

#[test]
fn it_rejects_a_positional_note_body() {
  let g = GestCmd::new();
  let id = g.create_task("Notable task");

  let output = g
    .cmd()
    .args(["task", "note", "add", &id, "positional body"])
    .output()
    .expect("task note add failed to run");

  assert!(
    !output.status.success(),
    "task note add should reject positional body, got success"
  );
}
