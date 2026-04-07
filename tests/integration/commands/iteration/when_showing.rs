use crate::support::helpers::GestCmd;

#[test]
fn it_shows_iteration_status_for_new_iteration() {
  let g = GestCmd::new();
  let id = g.create_iteration("Status sprint");

  let output = g
    .cmd()
    .args(["iteration", "status", &id])
    .output()
    .expect("iteration status failed to run");

  assert!(output.status.success(), "iteration status exited non-zero");
}
