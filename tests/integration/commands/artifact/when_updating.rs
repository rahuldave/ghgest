use crate::support::helpers::GestCmd;

#[test]
fn it_updates_body() {
  let g = GestCmd::new();
  let artifact_id = g.create_artifact("Titled", "original body");

  g.cmd()
    .args(["artifact", "update", &artifact_id, "--body", "rewritten body"])
    .assert()
    .success();

  let show = g
    .cmd()
    .args(["artifact", "show", &artifact_id])
    .output()
    .expect("artifact show failed");
  let stdout = String::from_utf8_lossy(&show.stdout);
  assert!(stdout.contains("rewritten body"), "got: {stdout}");
}

#[test]
fn it_updates_title() {
  let g = GestCmd::new();
  let artifact_id = g.create_artifact("original title", "body");

  g.cmd()
    .args(["artifact", "update", &artifact_id, "--title", "renamed artifact"])
    .assert()
    .success();

  let show = g
    .cmd()
    .args(["artifact", "show", &artifact_id])
    .output()
    .expect("artifact show failed");
  let stdout = String::from_utf8_lossy(&show.stdout);
  assert!(stdout.contains("renamed artifact"), "got: {stdout}");
}
