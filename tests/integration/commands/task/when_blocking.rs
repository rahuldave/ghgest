use predicates::prelude::*;

use crate::support::helpers::GestCmd;

fn create_task_and_get_id(env: &GestCmd, title: &str) -> String {
  let output = env
    .cmd()
    .args(["task", "create", title])
    .output()
    .expect("failed to run gest task create");

  assert!(output.status.success(), "task create failed");

  let stdout = String::from_utf8(output.stdout).expect("stdout is not valid utf8");
  let first_line = stdout.lines().next().expect("no output from task create");
  first_line
    .split_whitespace()
    .last()
    .expect("no ID in create output")
    .to_string()
}

#[test]
fn it_blocks_two_tasks() {
  let env = GestCmd::new();
  let id1 = create_task_and_get_id(&env, "Blocker task");
  let id2 = create_task_and_get_id(&env, "Blocked task");

  env
    .run(&["task", "block", &id1, &id2])
    .success()
    .stdout(predicate::str::contains("Linked"));
}

#[test]
fn it_blocks_with_artifact_flag() {
  let env = GestCmd::new();
  let task_id = create_task_and_get_id(&env, "Blocker task");
  let artifact_id = env.create_artifact("Some artifact", "body text");

  env
    .run(&["task", "block", &task_id, &artifact_id, "--artifact"])
    .success()
    .stdout(predicate::str::contains("Linked"));
}
