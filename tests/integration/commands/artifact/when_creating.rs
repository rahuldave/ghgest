use crate::support::helpers::GestCmd;

#[test]
fn it_creates_an_artifact() {
  let g = GestCmd::new();
  let output = g
    .cmd()
    .args(["artifact", "create", "My spec", "--body", "The body."])
    .output()
    .expect("artifact create failed to run");

  assert!(output.status.success(), "artifact create exited non-zero");
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("created artifact"), "got: {stdout}");
  assert!(stdout.contains("My spec"), "got: {stdout}");
}
