use crate::support::helpers::GestCmd;

fn add_note(g: &GestCmd, artifact_id: &str, body: &str) -> String {
  let output = g
    .cmd()
    .args(["artifact", "note", "add", artifact_id, "-b", body, "--quiet"])
    .output()
    .expect("artifact note add failed");
  assert!(output.status.success(), "artifact note add should succeed");
  String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[test]
fn it_lists_notes_in_order() {
  let g = GestCmd::new();
  let artifact_id = g.create_artifact("Listable notes", "body");

  add_note(&g, &artifact_id, "first note");
  add_note(&g, &artifact_id, "second note");

  let output = g
    .cmd()
    .args(["artifact", "note", "list", &artifact_id])
    .output()
    .expect("artifact note list failed");
  assert!(output.status.success(), "artifact note list should succeed");
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("first note"), "missing first: {stdout}");
  assert!(stdout.contains("second note"), "missing second: {stdout}");
}

#[test]
fn it_shows_empty_when_no_notes() {
  let g = GestCmd::new();
  let artifact_id = g.create_artifact("No notes yet", "body");

  let output = g
    .cmd()
    .args(["artifact", "note", "list", &artifact_id])
    .output()
    .expect("artifact note list failed");
  assert!(output.status.success(), "artifact note list should succeed");
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(
    stdout.to_lowercase().contains("no notes") || stdout.trim().is_empty() || stdout.contains("0 notes"),
    "expected empty-notes indicator, got: {stdout}"
  );
}
