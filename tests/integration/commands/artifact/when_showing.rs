use crate::support::helpers::GestCmd;

#[test]
fn it_shows_an_artifact_by_id() {
  let g = GestCmd::new();
  let id = g.create_artifact("Showable artifact", "detailed body text");

  let output = g
    .cmd()
    .args(["artifact", "show", &id])
    .output()
    .expect("artifact show failed to run");

  assert!(output.status.success(), "artifact show exited non-zero");
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("Showable artifact"), "got: {stdout}");
}
