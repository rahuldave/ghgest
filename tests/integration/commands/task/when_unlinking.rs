use serde_json::Value;

use crate::support::helpers::GestCmd;

#[test]
fn it_removes_a_task_to_task_relationship() {
  let g = GestCmd::new();
  let src = g.create_task("source task");
  let dst = g.create_task("target task");
  g.link_task(&src, &dst, "task", "relates-to");

  let unlink = g
    .cmd()
    .args(["task", "unlink", &src, &dst])
    .output()
    .expect("task unlink failed to run");
  assert!(
    unlink.status.success(),
    "task unlink exited non-zero: {}",
    String::from_utf8_lossy(&unlink.stderr)
  );

  let show = g
    .cmd()
    .args(["task", "show", &src, "--json"])
    .output()
    .expect("task show --json failed");
  assert!(show.status.success());
  let stdout = String::from_utf8_lossy(&show.stdout);
  let json: Value = serde_json::from_str(&stdout).expect("invalid JSON from task show --json");
  let relationships = json["relationships"]
    .as_array()
    .expect("relationships should be an array");
  assert!(
    relationships.is_empty(),
    "relationships should be empty after unlink, got: {relationships:?}"
  );
}

#[test]
fn it_removes_a_task_to_artifact_relationship() {
  let g = GestCmd::new();
  let task_id = g.create_task("linker");
  let artifact_id = g.create_artifact("spec", "body");
  g.link_task(&task_id, &artifact_id, "artifact", "relates-to");

  let unlink = g
    .cmd()
    .args(["task", "unlink", &task_id, &artifact_id, "--artifact"])
    .output()
    .expect("task unlink failed to run");
  assert!(
    unlink.status.success(),
    "task unlink exited non-zero: {}",
    String::from_utf8_lossy(&unlink.stderr)
  );

  let show = g
    .cmd()
    .args(["task", "show", &task_id, "--json"])
    .output()
    .expect("task show --json failed");
  assert!(show.status.success());
  let json: Value = serde_json::from_str(&String::from_utf8_lossy(&show.stdout)).expect("invalid JSON");
  let relationships = json["relationships"]
    .as_array()
    .expect("relationships should be an array");
  assert!(
    relationships.is_empty(),
    "relationships should be empty after unlink, got: {relationships:?}"
  );
}

#[test]
fn it_fails_when_no_matching_edge_exists() {
  let g = GestCmd::new();
  let src = g.create_task("source task");
  let dst = g.create_task("target task");

  let output = g
    .cmd()
    .args(["task", "unlink", &src, &dst])
    .output()
    .expect("task unlink failed to run");
  assert!(!output.status.success(), "task unlink should fail when no edge exists");
  let stderr = String::from_utf8_lossy(&output.stderr);
  assert!(
    stderr.contains("no relationship found"),
    "stderr should mention no relationship: {stderr}"
  );
}

#[test]
fn it_fails_when_multiple_edges_match_without_rel() {
  let g = GestCmd::new();
  let src = g.create_task("source task");
  let dst = g.create_task("target task");
  g.link_task(&src, &dst, "task", "relates-to");
  g.link_task(&src, &dst, "task", "blocks");

  let output = g
    .cmd()
    .args(["task", "unlink", &src, &dst])
    .output()
    .expect("task unlink failed to run");
  assert!(
    !output.status.success(),
    "task unlink should fail when multiple edges match and --rel is missing"
  );
  let stderr = String::from_utf8_lossy(&output.stderr);
  assert!(
    stderr.contains("multiple relationships found"),
    "stderr should mention multiple relationships: {stderr}"
  );
  assert!(
    stderr.contains("--rel"),
    "stderr should suggest --rel to disambiguate: {stderr}"
  );
}

#[test]
fn it_succeeds_when_multiple_edges_exist_and_rel_is_specified() {
  let g = GestCmd::new();
  let src = g.create_task("source task");
  let dst = g.create_task("target task");
  g.link_task(&src, &dst, "task", "relates-to");
  g.link_task(&src, &dst, "task", "blocks");

  let unlink = g
    .cmd()
    .args(["task", "unlink", &src, &dst, "--rel", "blocks"])
    .output()
    .expect("task unlink failed to run");
  assert!(
    unlink.status.success(),
    "task unlink should succeed with --rel: {}",
    String::from_utf8_lossy(&unlink.stderr)
  );

  let show = g
    .cmd()
    .args(["task", "show", &src, "--json"])
    .output()
    .expect("task show --json failed");
  assert!(show.status.success());
  let json: Value = serde_json::from_str(&String::from_utf8_lossy(&show.stdout)).expect("invalid JSON");
  let relationships = json["relationships"]
    .as_array()
    .expect("relationships should be an array");
  assert_eq!(
    relationships.len(),
    1,
    "exactly one relationship should remain, got: {relationships:?}"
  );
  let remaining = relationships[0]["rel_type"]
    .as_str()
    .expect("rel_type should be a string");
  assert_eq!(remaining, "relates-to", "the surviving rel should be relates-to");
}

#[test]
fn it_is_reversible_via_undo() {
  let g = GestCmd::new();
  let src = g.create_task("source task");
  let dst = g.create_task("target task");
  g.link_task(&src, &dst, "task", "relates-to");

  let unlink = g
    .cmd()
    .args(["task", "unlink", &src, &dst])
    .output()
    .expect("task unlink failed to run");
  assert!(
    unlink.status.success(),
    "task unlink exited non-zero: {}",
    String::from_utf8_lossy(&unlink.stderr)
  );

  // Confirm the relationship is gone.
  let show = g
    .cmd()
    .args(["task", "show", &src, "--json"])
    .output()
    .expect("task show --json failed");
  let json: Value = serde_json::from_str(&String::from_utf8_lossy(&show.stdout)).expect("invalid JSON");
  assert!(
    json["relationships"].as_array().unwrap().is_empty(),
    "relationship should be absent after unlink"
  );

  // Undo.
  let undo = g.cmd().args(["undo"]).output().expect("undo failed to run");
  assert!(
    undo.status.success(),
    "undo exited non-zero: {}",
    String::from_utf8_lossy(&undo.stderr)
  );

  // The relationship should be restored.
  let show = g
    .cmd()
    .args(["task", "show", &src, "--json"])
    .output()
    .expect("task show --json failed");
  let json: Value = serde_json::from_str(&String::from_utf8_lossy(&show.stdout)).expect("invalid JSON");
  let relationships = json["relationships"]
    .as_array()
    .expect("relationships should be an array");
  assert_eq!(
    relationships.len(),
    1,
    "relationship should be restored after undo, got: {relationships:?}"
  );
  assert_eq!(
    relationships[0]["rel_type"].as_str(),
    Some("relates-to"),
    "restored rel_type should match the original"
  );
}
