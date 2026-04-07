use crate::support::helpers::GestCmd;

#[test]
fn it_reopens_cancelled_iteration() {
  let g = GestCmd::new();
  let iter_id = g.create_iteration("reopen-me");

  g.cancel_iteration(&iter_id);
  g.reopen_iteration(&iter_id);

  let show = g
    .cmd()
    .args(["iteration", "show", &iter_id, "--json"])
    .output()
    .expect("iteration show failed");
  assert!(show.status.success(), "iteration show should succeed");
  let stdout = String::from_utf8_lossy(&show.stdout);
  let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid json");
  assert_eq!(
    parsed["status"].as_str(),
    Some("active"),
    "reopened iteration should be active, got: {stdout}"
  );
}

#[test]
fn it_reopens_completed_iteration() {
  let g = GestCmd::new();
  let iter_id = g.create_iteration("done-sprint");

  g.complete_iteration(&iter_id);
  g.reopen_iteration(&iter_id);

  let show = g
    .cmd()
    .args(["iteration", "show", &iter_id, "--json"])
    .output()
    .expect("iteration show failed");
  assert!(show.status.success());
  let stdout = String::from_utf8_lossy(&show.stdout);
  let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid json");
  assert_eq!(
    parsed["status"].as_str(),
    Some("active"),
    "reopened iteration should be active, got: {stdout}"
  );
}
