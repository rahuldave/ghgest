use crate::support::helpers::GestCmd;

#[test]
fn it_lists_iterations() {
  let g = GestCmd::new();
  g.create_iteration("Listable sprint");

  let output = g
    .cmd()
    .args(["iteration", "list"])
    .output()
    .expect("iteration list failed to run");

  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("Listable sprint"), "got: {stdout}");
}
