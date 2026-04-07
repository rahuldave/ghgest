use crate::support::helpers::GestCmd;

#[test]
fn it_creates_an_iteration() {
  let g = GestCmd::new();
  let output = g
    .cmd()
    .args(["iteration", "create", "Sprint 1"])
    .output()
    .expect("iteration create failed to run");

  assert!(output.status.success(), "iteration create exited non-zero");
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("created iteration"), "got: {stdout}");
  assert!(stdout.contains("Sprint 1"), "got: {stdout}");
}
