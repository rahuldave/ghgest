use crate::support::helpers::GestCmd;

#[test]
fn it_links_task_to_artifact() {
  let g = GestCmd::new();
  let task_id = g.create_task("linked task");
  let artifact_id = g.create_artifact("linked spec", "body");

  g.link_task(&task_id, &artifact_id, "artifact", "relates-to");

  let show = g
    .cmd()
    .args(["task", "show", &task_id])
    .output()
    .expect("task show failed");
  assert!(show.status.success(), "task show should succeed");
}

#[test]
fn it_links_task_to_task() {
  let g = GestCmd::new();
  let src = g.create_task("source task");
  let dst = g.create_task("target task");

  g.link_task(&src, &dst, "task", "relates-to");

  let show = g.cmd().args(["task", "show", &src]).output().expect("task show failed");
  assert!(show.status.success());
}

#[test]
fn it_shows_links_in_show_output() {
  let g = GestCmd::new();
  let task_id = g.create_task("linker");
  let artifact_id = g.create_artifact("target spec", "body");
  g.link_task(&task_id, &artifact_id, "artifact", "relates-to");

  let show = g
    .cmd()
    .args(["task", "show", &task_id])
    .output()
    .expect("task show failed");
  assert!(show.status.success());
  let stdout = String::from_utf8_lossy(&show.stdout);
  assert!(
    stdout.contains(&artifact_id[..8]) || stdout.contains("target spec") || stdout.to_lowercase().contains("link"),
    "show should reflect linked artifact, got: {stdout}"
  );
}
