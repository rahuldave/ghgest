use serde_json::Value;

use crate::support::helpers::GestCmd;

#[test]
fn it_emits_relationships_tags_and_notes_on_task_show_json() {
  let g = GestCmd::new();
  let task_a = g.create_task("Envelope task");
  let task_b = g.create_task("Blocked task");
  g.attach_tag("task", &task_a, "urgent");
  g.block_task(&task_a, &task_b);

  let note_output = g
    .cmd()
    .args(["task", "note", "add", &task_a, "--body", "a note"])
    .output()
    .expect("note add failed");
  assert!(note_output.status.success());

  let output = g
    .cmd()
    .args(["task", "show", &task_a, "--json"])
    .output()
    .expect("task show --json failed");

  assert!(output.status.success());
  let json: Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");

  assert_eq!(json["title"], "Envelope task");
  assert!(
    json["tags"]
      .as_array()
      .unwrap()
      .contains(&Value::String("urgent".into()))
  );
  assert!(!json["relationships"].as_array().unwrap().is_empty());
  assert!(!json["notes"].as_array().unwrap().is_empty());
  assert_eq!(json["notes"][0]["body"], "a note");
}

#[test]
fn it_omits_notes_from_task_list_json() {
  let g = GestCmd::new();
  let task_id = g.create_task("List envelope task");
  g.attach_tag("task", &task_id, "list-tag");

  let output = g
    .cmd()
    .args(["task", "list", "--json"])
    .output()
    .expect("task list --json failed");

  assert!(output.status.success());
  let json: Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");

  let arr = json.as_array().expect("expected array");
  assert!(!arr.is_empty());

  let first = &arr[0];
  assert!(first.get("tags").is_some(), "expected tags key");
  assert!(first.get("relationships").is_some(), "expected relationships key");
  assert!(first.get("notes").is_none(), "notes should be omitted from list");
}

#[test]
fn it_relationship_entries_omit_source_type() {
  let g = GestCmd::new();
  let task_a = g.create_task("Source task");
  let task_b = g.create_task("Target task");
  g.block_task(&task_a, &task_b);

  let output = g
    .cmd()
    .args(["task", "show", &task_a, "--json"])
    .output()
    .expect("task show --json failed");

  assert!(output.status.success());
  let json: Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");

  let rels = json["relationships"].as_array().expect("expected relationships array");
  for rel in rels {
    assert!(
      rel.get("source_type").is_none(),
      "source_type should not appear in relationship entries"
    );
  }
}

#[test]
fn it_serializes_empty_sidecars_as_arrays() {
  let g = GestCmd::new();
  let task_id = g.create_task("Bare task");

  let output = g
    .cmd()
    .args(["task", "show", &task_id, "--json"])
    .output()
    .expect("task show --json failed");

  assert!(output.status.success());
  let json: Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");

  assert_eq!(json["tags"], Value::Array(vec![]));
  assert_eq!(json["relationships"], Value::Array(vec![]));
  assert_eq!(json["notes"], Value::Array(vec![]));
}

#[test]
fn it_preserves_human_output_for_task_show() {
  let g = GestCmd::new();
  let task_id = g.create_task("Human output task");

  let output = g
    .cmd()
    .args(["task", "show", &task_id])
    .output()
    .expect("task show failed");

  assert!(output.status.success());
  let stdout = String::from_utf8_lossy(&output.stdout);

  assert!(
    stdout.contains("Human output task"),
    "human output should contain the title"
  );
  // Human output should not be JSON
  assert!(
    serde_json::from_str::<Value>(&stdout).is_err(),
    "human output should not be valid JSON"
  );
}
