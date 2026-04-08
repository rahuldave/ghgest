use crate::support::helpers::GestCmd;

#[test]
fn it_creates_a_task() {
  let g = GestCmd::new();
  let output = g
    .cmd()
    .args(["task", "create", "Hello task"])
    .output()
    .expect("task create failed to run");

  assert!(output.status.success(), "task create exited non-zero");
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("created task"), "got: {stdout}");
  assert!(stdout.contains("Hello task"), "got: {stdout}");
}

#[test]
fn it_rejects_phase_without_iteration() {
  let g = GestCmd::new();
  let output = g
    .cmd()
    .args(["task", "create", "Hello task", "--phase", "2"])
    .output()
    .expect("task create failed to run");

  assert!(
    !output.status.success(),
    "task create --phase without --iteration should error"
  );
  let stderr = String::from_utf8_lossy(&output.stderr);
  assert!(
    stderr.contains("--iteration"),
    "expected error to mention --iteration, got: {stderr}"
  );
}
