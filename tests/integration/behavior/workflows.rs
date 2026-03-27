use crate::support::helpers::GestCmd;

/// Extract the 8-character ID from a confirmation line like "Created task <id>" or "Created artifact <id>".
fn extract_id(output: &str, entity: &str) -> String {
  let prefix = format!("Created {entity} ");
  output
    .trim()
    .strip_prefix(&prefix)
    .unwrap_or_else(|| panic!("expected 'Created {entity} <id>' but got: {output}"))
    .to_string()
}

#[test]
fn it_creates_and_updates_a_task() {
  let env = GestCmd::new();

  // Step 1: Create a task with a description
  let create_output = env
    .run(["task", "create", "Workflow Task", "-d", "Initial description"])
    .output()
    .expect("failed to create task");
  assert!(create_output.status.success());
  let task_id = extract_id(&String::from_utf8_lossy(&create_output.stdout), "task");

  // Step 2: Show the task to verify it exists
  env.run(["task", "show", &task_id]).assert().success();

  // Step 3: Update the task's description
  env
    .run(["task", "update", &task_id, "-d", "Updated description"])
    .assert()
    .success();

  // Step 4: Tag the task
  env
    .run(["task", "tag", &task_id, "integration", "workflow"])
    .assert()
    .success();

  // Step 5: Verify all changes via task show --json
  let show_output = env
    .run(["task", "show", &task_id, "--json"])
    .output()
    .expect("failed to show task as json");
  assert!(show_output.status.success());
  let stdout = String::from_utf8_lossy(&show_output.stdout);
  let json: serde_json::Value = serde_json::from_str(&stdout).expect("invalid JSON from task show --json");

  assert_eq!(json["description"], "Updated description");
  let tags = json["tags"].as_array().expect("tags should be an array");
  let tag_strings: Vec<&str> = tags.iter().filter_map(|v| v.as_str()).collect();
  assert!(
    tag_strings.contains(&"integration"),
    "expected 'integration' tag, got: {tag_strings:?}"
  );
  assert!(
    tag_strings.contains(&"workflow"),
    "expected 'workflow' tag, got: {tag_strings:?}"
  );
}

#[test]
fn it_creates_and_updates_an_artifact() {
  let env = GestCmd::new();

  // Step 1: Create an artifact with title, type, and body
  let create_output = env
    .run([
      "artifact",
      "create",
      "--title",
      "Workflow Spec",
      "--type",
      "spec",
      "--body",
      "Initial body content",
    ])
    .output()
    .expect("failed to create artifact");
  assert!(create_output.status.success());
  let artifact_id = extract_id(&String::from_utf8_lossy(&create_output.stdout), "artifact");

  // Step 2: Show the artifact to verify it exists
  env.run(["artifact", "show", &artifact_id]).assert().success();

  // Step 3: Update the artifact's body
  env
    .run(["artifact", "update", &artifact_id, "--body", "Updated body content"])
    .assert()
    .success();

  // Step 4: Tag the artifact
  env
    .run(["artifact", "tag", &artifact_id, "integration", "spec"])
    .assert()
    .success();

  // Step 5: Archive the artifact
  env.run(["artifact", "archive", &artifact_id]).assert().success();

  // Step 6: Verify via artifact show --json
  let show_output = env
    .run(["artifact", "show", &artifact_id, "--json"])
    .output()
    .expect("failed to show artifact as json");
  assert!(show_output.status.success());
  let stdout = String::from_utf8_lossy(&show_output.stdout);
  let json: serde_json::Value = serde_json::from_str(&stdout).expect("invalid JSON from artifact show --json");

  assert_eq!(json["body"], "Updated body content");
  let tags = json["tags"].as_array().expect("tags should be an array");
  let tag_strings: Vec<&str> = tags.iter().filter_map(|v| v.as_str()).collect();
  assert!(
    tag_strings.contains(&"integration"),
    "expected 'integration' tag, got: {tag_strings:?}"
  );
  assert!(
    tag_strings.contains(&"spec"),
    "expected 'spec' tag, got: {tag_strings:?}"
  );
  assert!(
    json["archived_at"].is_string(),
    "expected archived_at to be set after archiving"
  );
  assert_ne!(
    json["archived_at"],
    serde_json::Value::Null,
    "archived_at should not be null"
  );
}

#[test]
fn it_links_task_to_artifact() {
  let env = GestCmd::new();

  // Step 1: Create a task
  let task_output = env
    .run(["task", "create", "Cross-Entity Task", "-d", "will link to artifact"])
    .output()
    .expect("failed to create task");
  assert!(task_output.status.success());
  let task_id = extract_id(&String::from_utf8_lossy(&task_output.stdout), "task");

  // Step 2: Create an artifact
  let artifact_output = env
    .run([
      "artifact",
      "create",
      "--title",
      "Cross-Entity Artifact",
      "--body",
      "artifact body",
    ])
    .output()
    .expect("failed to create artifact");
  assert!(artifact_output.status.success());
  let artifact_id = extract_id(&String::from_utf8_lossy(&artifact_output.stdout), "artifact");

  // Step 3: Link the task to the artifact
  env
    .run(["task", "link", &task_id, "relates-to", &artifact_id, "--artifact"])
    .assert()
    .success();

  // Step 4: Verify the link appears in task show --json
  let show_output = env
    .run(["task", "show", &task_id, "--json"])
    .output()
    .expect("failed to show task as json");
  assert!(show_output.status.success());
  let stdout = String::from_utf8_lossy(&show_output.stdout);
  let json: serde_json::Value = serde_json::from_str(&stdout).expect("invalid JSON from task show --json");

  let links = json["links"].as_array().expect("links should be an array");
  assert_eq!(links.len(), 1, "expected exactly one link");
  assert_eq!(links[0]["rel"], "relates-to");
  assert!(
    links[0]["ref"].as_str().unwrap().starts_with("artifacts/"),
    "expected link ref to start with 'artifacts/', got: {}",
    links[0]["ref"]
  );
}
