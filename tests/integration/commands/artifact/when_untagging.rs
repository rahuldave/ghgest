use crate::support::helpers::GestCmd;

#[test]
fn it_handles_missing_tag() {
  let g = GestCmd::new();
  let artifact_id = g.create_artifact("No tags", "body");

  let output = g
    .cmd()
    .args(["artifact", "untag", &artifact_id, "never-set"])
    .output()
    .expect("artifact untag failed");

  // The artifact should remain accessible regardless of whether untag errors.
  let show = g
    .cmd()
    .args(["artifact", "show", &artifact_id])
    .output()
    .expect("artifact show failed");
  assert!(
    show.status.success(),
    "artifact should remain accessible; untag exit code was {:?}",
    output.status.code()
  );
}

#[test]
fn it_removes_tag_from_artifact() {
  let g = GestCmd::new();
  let artifact_id = g.create_artifact("Tagged artifact", "body");
  g.attach_tag("artifact", &artifact_id, "keep");
  g.attach_tag("artifact", &artifact_id, "drop");

  let output = g
    .cmd()
    .args(["artifact", "untag", &artifact_id, "drop"])
    .output()
    .expect("artifact untag failed");
  assert!(
    output.status.success(),
    "artifact untag should succeed: {}",
    String::from_utf8_lossy(&output.stderr)
  );

  let list = g
    .cmd()
    .args(["tag", "list", "--artifact"])
    .output()
    .expect("tag list failed");
  let stdout = String::from_utf8_lossy(&list.stdout);
  assert!(stdout.contains("keep"), "keep tag should remain, got: {stdout}");
  assert!(!stdout.contains("drop"), "drop tag should be gone, got: {stdout}");
}
