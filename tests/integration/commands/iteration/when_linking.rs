use crate::support::helpers::GestCmd;

#[test]
fn it_links_iteration_to_artifact() {
  let g = GestCmd::new();
  let iter_id = g.create_iteration("link-sprint");
  let artifact_id = g.create_artifact("Linked spec", "body");

  let output = g
    .cmd()
    .args(["iteration", "link", &iter_id, "relates-to", &artifact_id, "--artifact"])
    .output()
    .expect("iteration link failed");

  assert!(
    output.status.success(),
    "iteration link should succeed: {}",
    String::from_utf8_lossy(&output.stderr)
  );
}
