use crate::support::helpers::GestCmd;

#[test]
fn it_lists_artifacts() {
  let g = GestCmd::new();
  g.create_artifact("Listable artifact", "body");

  let output = g
    .cmd()
    .args(["artifact", "list"])
    .output()
    .expect("artifact list failed to run");

  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("Listable artifact"), "got: {stdout}");
}
