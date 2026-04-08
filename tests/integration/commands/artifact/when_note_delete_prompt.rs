use crate::support::helpers::GestCmd;

fn add_note(g: &GestCmd, artifact_id: &str, body: &str) -> String {
  let output = g
    .cmd()
    .args(["artifact", "note", "add", artifact_id, "-b", body, "--quiet"])
    .output()
    .expect("artifact note add failed");
  assert!(output.status.success());
  String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[test]
fn it_prompts_and_aborts_when_user_declines() {
  let g = GestCmd::new();
  let artifact_id = g.create_artifact("Spec", "body");
  let note_id = add_note(&g, &artifact_id, "keep me");

  let output = g
    .cmd()
    .args(["artifact", "note", "delete", &note_id])
    .write_stdin("n\n")
    .output()
    .expect("artifact note delete failed to run");
  assert!(
    output.status.success(),
    "abort path should exit zero: {}",
    String::from_utf8_lossy(&output.stderr)
  );

  let show = g
    .cmd()
    .args(["artifact", "note", "show", &note_id])
    .output()
    .expect("artifact note show failed");
  assert!(show.status.success(), "note should still exist after declined prompt");
}

#[test]
fn it_skips_prompt_and_deletes_when_yes_is_passed() {
  let g = GestCmd::new();
  let artifact_id = g.create_artifact("yes spec", "body");
  let note_id = add_note(&g, &artifact_id, "zap");

  let output = g
    .cmd()
    .args(["artifact", "note", "delete", &note_id, "--yes"])
    .output()
    .expect("artifact note delete failed");
  assert!(output.status.success());

  let show = g
    .cmd()
    .args(["artifact", "note", "show", &note_id])
    .output()
    .expect("artifact note show failed");
  assert!(!show.status.success(), "deleted note should not be viewable");
}
