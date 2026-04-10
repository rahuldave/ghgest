use serde_json::Value;

use crate::support::helpers::GestCmd;

#[test]
fn it_emits_relationships_tags_and_notes_on_iteration_show_json() {
  let g = GestCmd::new();
  let iter_id = g.create_iteration("Envelope sprint");
  let iter_id2 = g.create_iteration("Related sprint");

  g.attach_tag("iteration", &iter_id, "urgent");

  let link_output = g
    .cmd()
    .args(["iteration", "link", &iter_id, "relates-to", &iter_id2])
    .output()
    .expect("iteration link failed");
  assert!(
    link_output.status.success(),
    "iteration link failed: {}",
    String::from_utf8_lossy(&link_output.stderr)
  );

  let show_output = g
    .cmd()
    .args(["iteration", "show", &iter_id, "--json"])
    .output()
    .expect("iteration show --json failed");
  assert!(
    show_output.status.success(),
    "iteration show --json failed: {}",
    String::from_utf8_lossy(&show_output.stderr)
  );

  let stdout = String::from_utf8_lossy(&show_output.stdout);
  let json: Value = serde_json::from_str(&stdout).expect("invalid JSON from iteration show --json");

  assert!(json.get("title").is_some(), "envelope should flatten iteration title");
  assert!(
    json.get("relationships").is_some(),
    "envelope should include relationships"
  );
  assert!(json.get("tags").is_some(), "envelope should include tags");
  assert!(json.get("notes").is_some(), "show includes notes key");

  let tags = json["tags"].as_array().expect("tags should be an array");
  assert!(
    tags.iter().any(|t| t.as_str() == Some("urgent")),
    "tags should contain 'urgent'"
  );

  let relationships = json["relationships"]
    .as_array()
    .expect("relationships should be an array");
  assert!(!relationships.is_empty(), "relationships should not be empty");
}

#[test]
fn it_omits_notes_from_iteration_list_json() {
  let g = GestCmd::new();
  g.create_iteration("List envelope sprint");

  let list_output = g
    .cmd()
    .args(["iteration", "list", "--json"])
    .output()
    .expect("iteration list --json failed");
  assert!(list_output.status.success());

  let stdout = String::from_utf8_lossy(&list_output.stdout);
  let json: Value = serde_json::from_str(&stdout).expect("invalid JSON from iteration list --json");

  let arr = json.as_array().expect("list JSON should be an array");
  assert!(!arr.is_empty(), "list should have at least one iteration");

  for entry in arr {
    assert!(entry.get("notes").is_none(), "list envelopes should omit notes");
    assert!(entry.get("tags").is_some(), "list envelopes should include tags");
    assert!(
      entry.get("relationships").is_some(),
      "list envelopes should include relationships"
    );
  }
}

#[test]
fn it_relationship_entries_omit_source_type() {
  let g = GestCmd::new();
  let iter_id = g.create_iteration("Rel source type sprint");
  let iter_id2 = g.create_iteration("Other rel sprint");

  let link_output = g
    .cmd()
    .args(["iteration", "link", &iter_id, "relates-to", &iter_id2])
    .output()
    .expect("iteration link failed");
  assert!(link_output.status.success());

  let show_output = g
    .cmd()
    .args(["iteration", "show", &iter_id, "--json"])
    .output()
    .expect("iteration show --json failed");
  assert!(show_output.status.success());

  let stdout = String::from_utf8_lossy(&show_output.stdout);
  let json: Value = serde_json::from_str(&stdout).expect("invalid JSON");

  let relationships = json["relationships"]
    .as_array()
    .expect("relationships should be an array");
  for rel in relationships {
    assert!(
      rel.get("source_type").is_none(),
      "relationship entries should omit source_type"
    );
    assert!(
      rel.get("rel_type").is_some(),
      "relationship entries should include rel_type"
    );
    assert!(
      rel.get("source_id").is_some(),
      "relationship entries should include source_id"
    );
    assert!(
      rel.get("target_id").is_some(),
      "relationship entries should include target_id"
    );
    assert!(
      rel.get("target_type").is_some(),
      "relationship entries should include target_type"
    );
  }
}

#[test]
fn it_serializes_empty_sidecars_as_arrays() {
  let g = GestCmd::new();
  let iter_id = g.create_iteration("Empty sidecars sprint");

  let show_output = g
    .cmd()
    .args(["iteration", "show", &iter_id, "--json"])
    .output()
    .expect("iteration show --json failed");
  assert!(show_output.status.success());

  let stdout = String::from_utf8_lossy(&show_output.stdout);
  let json: Value = serde_json::from_str(&stdout).expect("invalid JSON");

  assert_eq!(json["tags"], Value::Array(vec![]), "empty tags should be []");
  assert_eq!(
    json["relationships"],
    Value::Array(vec![]),
    "empty relationships should be []"
  );
  assert_eq!(json["notes"], Value::Array(vec![]), "empty notes should be []");
}

#[test]
fn it_preserves_human_output_for_iteration_show() {
  let g = GestCmd::new();
  let iter_id = g.create_iteration("Human output sprint");

  let show_output = g
    .cmd()
    .args(["iteration", "show", &iter_id])
    .output()
    .expect("iteration show failed");
  assert!(show_output.status.success());

  let stdout = String::from_utf8_lossy(&show_output.stdout);

  assert!(
    stdout.contains("Human output sprint"),
    "human output should contain the title"
  );
  assert!(
    !stdout.contains("\"relationships\""),
    "human output should not contain JSON keys"
  );
  assert!(
    !stdout.contains("\"tags\""),
    "human output should not contain JSON keys"
  );
}
