use serde_json::Value;

use crate::support::helpers::GestCmd;

#[test]
fn it_emits_relationships_tags_and_notes_on_artifact_show_json() {
  let g = GestCmd::new();
  let artifact_id = g.create_artifact("Envelope test", "body");
  g.attach_tag("artifact", &artifact_id, "important");
  add_note(&g, &artifact_id, "This is a note");
  let task_id = g.create_task("linker");
  g.link_task(&task_id, &artifact_id, "artifact", "relates-to");

  let output = g
    .cmd()
    .args(["artifact", "show", &artifact_id, "--json"])
    .output()
    .expect("artifact show --json failed");

  assert!(
    output.status.success(),
    "exit non-zero: {}",
    String::from_utf8_lossy(&output.stderr)
  );
  let json: Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");
  assert!(json.get("relationships").is_some(), "missing relationships key: {json}");
  assert!(json.get("tags").is_some(), "missing tags key: {json}");
  assert!(json.get("notes").is_some(), "missing notes key: {json}");
  let tags = json["tags"].as_array().unwrap();
  assert!(
    tags.iter().any(|t| t == "important"),
    "expected 'important' tag in: {tags:?}"
  );
  let notes = json["notes"].as_array().unwrap();
  assert!(!notes.is_empty(), "expected at least one note");
  assert_eq!(notes[0]["body"], "This is a note");
  let rels = json["relationships"].as_array().unwrap();
  assert!(!rels.is_empty(), "expected at least one relationship");
  // Relationship entries must not contain source_type
  for rel in rels {
    assert!(
      rel.get("source_type").is_none(),
      "source_type should not appear in relationship: {rel}"
    );
  }
}

#[test]
fn it_omits_notes_from_artifact_list_json() {
  let g = GestCmd::new();
  let id = g.create_artifact("Listed envelope", "body");
  g.attach_tag("artifact", &id, "listed-tag");
  add_note(&g, &id, "should not appear");

  let output = g
    .cmd()
    .args(["artifact", "list", "--json"])
    .output()
    .expect("artifact list --json failed");

  assert!(output.status.success());
  let json: Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");
  let arr = json.as_array().expect("expected array");
  assert!(!arr.is_empty(), "list should not be empty");
  for elem in arr {
    assert!(elem.get("relationships").is_some(), "missing relationships: {elem}");
    assert!(elem.get("tags").is_some(), "missing tags: {elem}");
    assert!(elem.get("notes").is_none(), "notes should be omitted from list: {elem}");
  }
}

#[test]
fn it_serializes_empty_sidecars_as_arrays() {
  let g = GestCmd::new();
  let id = g.create_artifact("Bare artifact", "no sidecars");

  let output = g
    .cmd()
    .args(["artifact", "show", &id, "--json"])
    .output()
    .expect("artifact show --json failed");

  assert!(output.status.success());
  let json: Value = serde_json::from_slice(&output.stdout).expect("invalid JSON");
  assert_eq!(
    json["relationships"],
    Value::Array(vec![]),
    "expected empty relationships array"
  );
  assert_eq!(json["tags"], Value::Array(vec![]), "expected empty tags array");
  let notes = json["notes"].as_array().expect("notes should be present on show");
  assert!(notes.is_empty(), "expected empty notes array");
}

#[test]
fn it_preserves_human_output_for_artifact_show() {
  let g = GestCmd::new();
  let id = g.create_artifact("Human readable", "some body");

  let output = g
    .cmd()
    .args(["artifact", "show", &id])
    .output()
    .expect("artifact show failed");

  assert!(output.status.success());
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("Human readable"), "expected title in output: {stdout}");
  // Human output should not contain JSON envelope keys
  assert!(
    !stdout.contains("\"relationships\""),
    "human output should not contain JSON keys"
  );
}

fn add_note(g: &GestCmd, artifact_id: &str, body: &str) {
  let output = g
    .cmd()
    .args(["artifact", "note", "add", artifact_id, "--body", body])
    .output()
    .expect("artifact note add failed");

  assert!(
    output.status.success(),
    "artifact note add exited non-zero: {}",
    String::from_utf8_lossy(&output.stderr)
  );
}
