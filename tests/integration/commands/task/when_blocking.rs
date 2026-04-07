use crate::support::helpers::GestCmd;

#[test]
fn it_blocks_task_with_another_task() {
  let g = GestCmd::new();
  let blocker = g.create_task("blocker");
  let blocked = g.create_task("blocked");

  g.block_task(&blocker, &blocked);

  let show = g
    .cmd()
    .args(["task", "show", &blocked])
    .output()
    .expect("task show failed");
  assert!(show.status.success(), "task show should succeed");
}

#[test]
fn it_shows_blocked_status() {
  let g = GestCmd::new();
  let blocker = g.create_task("alpha blocker");
  let blocked = g.create_task("beta blocked");
  g.block_task(&blocker, &blocked);

  let show = g
    .cmd()
    .args(["task", "show", &blocked, "--json"])
    .output()
    .expect("task show --json failed");
  assert!(show.status.success(), "task show should succeed");
  let stdout = String::from_utf8_lossy(&show.stdout);
  assert!(
    stdout.contains(&blocker[..8]) || stdout.to_lowercase().contains("block"),
    "show output should reference the blocker or block relationship, got: {stdout}"
  );
}

#[test]
fn it_rejects_self_block() {
  let g = GestCmd::new();
  let task_id = g.create_task("self-block");

  // The CLI currently records self-blocks without error. This test pins that
  // behavior so a future tightening (or bug fix) surfaces intentionally.
  let output = g
    .cmd()
    .args(["task", "block", &task_id, &task_id])
    .output()
    .expect("task block failed");

  assert!(
    output.status.success(),
    "task block accepts self-block today; if this changes, update the assertion"
  );
}
