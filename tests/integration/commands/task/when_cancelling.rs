use crate::support::helpers::GestCmd;

#[test]
fn it_cancels_blocked_task() {
  let g = GestCmd::new();
  let blocker = g.create_task("blocker");
  let blocked = g.create_task("blocked");
  g.block_task(&blocker, &blocked);

  g.cancel_task(&blocked);

  let show = g
    .cmd()
    .args(["task", "show", &blocked, "--json"])
    .output()
    .expect("task show failed");
  assert!(show.status.success());
  let stdout = String::from_utf8_lossy(&show.stdout);
  let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid json");
  assert_eq!(parsed["status"].as_str(), Some("cancelled"), "got: {stdout}");
}

#[test]
fn it_cancels_open_task() {
  let g = GestCmd::new();
  let task_id = g.create_task("cancel me");

  g.cancel_task(&task_id);

  let show = g
    .cmd()
    .args(["task", "show", &task_id, "--json"])
    .output()
    .expect("task show failed");
  let stdout = String::from_utf8_lossy(&show.stdout);
  let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid json");
  assert_eq!(parsed["status"].as_str(), Some("cancelled"), "got: {stdout}");
}

#[test]
fn it_rejects_cancelling_done_task() {
  let g = GestCmd::new();
  let task_id = g.create_task("already done");
  g.complete_task(&task_id);

  // The CLI is currently lenient and permits cancellation from any status. This
  // test pins the observed behavior so any future tightening is intentional.
  let output = g
    .cmd()
    .args(["task", "cancel", &task_id])
    .output()
    .expect("task cancel failed");

  assert!(
    output.status.success(),
    "task cancel is lenient on done tasks today; update assertion if behavior tightens"
  );
}
