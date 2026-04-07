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
fn it_deletes_note_by_id() {
  let g = GestCmd::new();
  let artifact_id = g.create_artifact("Deletable notes", "body");
  let note_id = add_note(&g, &artifact_id, "doomed note");

  let output = g
    .cmd()
    .args(["artifact", "note", "delete", &note_id])
    .output()
    .expect("artifact note delete failed");
  assert!(
    output.status.success(),
    "artifact note delete should succeed: {}",
    String::from_utf8_lossy(&output.stderr)
  );

  // Subsequent show on the deleted note should fail.
  let show = g
    .cmd()
    .args(["artifact", "note", "show", &note_id])
    .output()
    .expect("artifact note show failed");
  assert!(!show.status.success(), "show on a deleted note should exit non-zero");
}

#[test]
fn it_errors_on_missing_note() {
  let g = GestCmd::new();

  let output = g
    .cmd()
    .args(["artifact", "note", "delete", "zzzzzzzz"])
    .output()
    .expect("artifact note delete failed");

  assert!(
    !output.status.success(),
    "deleting a nonexistent note should exit non-zero"
  );
}
