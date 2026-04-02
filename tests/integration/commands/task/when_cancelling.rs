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
fn it_cancels_the_task() {
  let env = GestCmd::new();
  let id = create_task_and_get_id(&env, "Task to cancel");

  env
    .run(&["task", "cancel", &id])
    .success()
    .stdout(predicate::str::contains("cancelled"));
}

#[test]
fn it_produces_same_status_as_update_with_cancelled() {
  let env = GestCmd::new();
  let id = create_task_and_get_id(&env, "Compare cancel");

  env.run(&["task", "cancel", &id]).success();

  env
    .run(&["task", "show", &id])
    .success()
    .stdout(predicate::str::contains("cancelled"));
}
