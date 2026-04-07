use crate::support::helpers::GestCmd;

#[test]
fn it_claims_unassigned_task() {
  let g = GestCmd::new();
  let task_id = g.create_task("claimable");

  g.claim_task(&task_id);

  let show = g
    .cmd()
    .args(["task", "show", &task_id, "--json"])
    .output()
    .expect("task show failed");
  assert!(show.status.success());
  let stdout = String::from_utf8_lossy(&show.stdout);
  let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid json");
  assert_eq!(
    parsed["status"].as_str(),
    Some("inprogress"),
    "claimed task should be inprogress, got: {stdout}"
  );
  assert!(
    parsed["assigned_to"].is_string(),
    "claimed task should have a non-null assigned_to, got: {stdout}"
  );
}

#[test]
fn it_reassigns_claimed_task() {
  let g = GestCmd::new();
  let task_id = g.create_task("reassignable");

  g.cmd()
    .args(["task", "claim", &task_id, "--as", "alice"])
    .assert()
    .success();

  let first = g
    .cmd()
    .args(["task", "show", &task_id, "--json"])
    .output()
    .expect("task show failed");
  let first_stdout = String::from_utf8_lossy(&first.stdout);
  let first_parsed: serde_json::Value = serde_json::from_str(&first_stdout).expect("valid json");
  let first_author = first_parsed["assigned_to"]
    .as_str()
    .expect("alice assignee")
    .to_string();

  g.cmd()
    .args(["task", "claim", &task_id, "--as", "bob"])
    .assert()
    .success();

  let second = g
    .cmd()
    .args(["task", "show", &task_id, "--json"])
    .output()
    .expect("task show failed");
  let second_stdout = String::from_utf8_lossy(&second.stdout);
  let second_parsed: serde_json::Value = serde_json::from_str(&second_stdout).expect("valid json");
  let second_author = second_parsed["assigned_to"].as_str().expect("bob assignee");

  assert_ne!(
    first_author, second_author,
    "reassigning should change the assigned_to author id"
  );
}
