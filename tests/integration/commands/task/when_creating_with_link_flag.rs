use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_creates_the_task_successfully() {
  let env = GestCmd::new();
  let target_id = env.create_task("Blocker");

  env
    .cmd()
    .args(["task", "create", "New Task", "--link", &format!("blocks:{target_id}")])
    .assert()
    .success()
    .stdout(predicate::str::contains("created task"));
}

#[test]
fn it_records_the_link_on_the_created_task() {
  let env = GestCmd::new();
  let target_id = env.create_task("Target Task");

  let output = env
    .cmd()
    .args([
      "task",
      "create",
      "Linked Task",
      "--link",
      &format!("blocks:{target_id}"),
    ])
    .output()
    .expect("failed to run gest task create --link");

  assert!(
    output.status.success(),
    "task create --link failed: {}",
    String::from_utf8_lossy(&output.stderr)
  );

  // Retrieve the newly created task's short ID from stdout.
  let stdout = String::from_utf8_lossy(&output.stdout);
  let created_id = stdout
    .lines()
    .next()
    .and_then(|line| line.split_whitespace().last())
    .expect("no ID in task create output")
    .to_string();

  // task show should display the link relationship.
  env
    .cmd()
    .args(["task", "show", &created_id])
    .assert()
    .success()
    .stdout(predicate::str::contains("blocks"));
}

#[test]
fn it_errors_on_invalid_link_format() {
  let env = GestCmd::new();

  env
    .cmd()
    .args(["task", "create", "Bad Link Task", "--link", "no-colon-here"])
    .assert()
    .failure();
}
