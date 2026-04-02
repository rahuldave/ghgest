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
fn it_marks_task_as_done() {
  let env = GestCmd::new();
  let id = create_task_and_get_id(&env, "Complete me");

  env
    .run(&["task", "complete", &id])
    .success()
    .stdout(predicate::str::contains("done"));
}

#[test]
fn it_produces_same_output_as_update_status_done() {
  let env = GestCmd::new();
  let id_complete = create_task_and_get_id(&env, "Via complete");
  let id_update = create_task_and_get_id(&env, "Via update");

  let complete_output = env
    .cmd()
    .args(["task", "complete", &id_complete])
    .output()
    .expect("complete failed");
  let update_output = env
    .cmd()
    .args(["task", "update", &id_update, "--status", "done"])
    .output()
    .expect("update failed");

  let complete_stdout = String::from_utf8(complete_output.stdout).expect("not utf8");
  let update_stdout = String::from_utf8(update_output.stdout).expect("not utf8");

  // Normalize output by removing the task-specific ID prefix so the structure matches.
  let complete_normalized = complete_stdout.replace(&id_complete, "ID");
  let update_normalized = update_stdout.replace(&id_update, "ID");

  assert_eq!(complete_normalized, update_normalized);
}

#[test]
fn it_shows_task_as_done_in_show() {
  let env = GestCmd::new();
  let id = create_task_and_get_id(&env, "Verify done");

  env.run(&["task", "complete", &id]).success();

  env
    .run(&["task", "show", &id])
    .success()
    .stdout(predicate::str::contains("done"));
}
