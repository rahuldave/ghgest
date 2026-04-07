use crate::support::helpers::GestCmd;

#[test]
fn it_removes_tag_from_iteration() {
  let g = GestCmd::new();
  let iter_id = g.create_iteration("tagged sprint");
  g.attach_tag("iteration", &iter_id, "urgent");

  let output = g
    .cmd()
    .args(["iteration", "untag", &iter_id, "urgent"])
    .output()
    .expect("iteration untag failed");

  assert!(
    output.status.success(),
    "iteration untag should succeed: {}",
    String::from_utf8_lossy(&output.stderr)
  );

  let list = g
    .cmd()
    .args(["tag", "list", "--iteration"])
    .output()
    .expect("tag list failed");
  let stdout = String::from_utf8_lossy(&list.stdout);
  assert!(
    !stdout.contains("urgent"),
    "urgent tag should be gone from iteration tag list, got: {stdout}"
  );
}
