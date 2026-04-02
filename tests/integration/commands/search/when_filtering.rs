use predicates::prelude::*;

use crate::support::helpers::GestCmd;

// ---------------------------------------------------------------------------
// Entity type scoping: is:<type>
// ---------------------------------------------------------------------------

#[test]
fn it_scopes_results_to_tasks_with_is_task() {
  let env = GestCmd::new();
  env.cmd().args(["task", "create", "filtertask_is"]).assert().success();
  env.create_artifact("filterartifact_is", "body");
  env
    .cmd()
    .args(["iteration", "create", "filteriter_is"])
    .assert()
    .success();

  env
    .cmd()
    .args(["search", "is:task filter"])
    .assert()
    .success()
    .stdout(predicate::str::contains("task"))
    .stdout(predicate::str::contains("filtertask_is"))
    .stdout(predicate::str::contains("artifact").not())
    .stdout(predicate::str::contains("iteration").not());
}

#[test]
fn it_scopes_results_to_artifacts_with_is_artifact() {
  let env = GestCmd::new();
  env.cmd().args(["task", "create", "filtertask_isa"]).assert().success();
  env.create_artifact("filterartifact_isa", "body");
  env
    .cmd()
    .args(["iteration", "create", "filteriter_isa"])
    .assert()
    .success();

  env
    .cmd()
    .args(["search", "is:artifact filter"])
    .assert()
    .success()
    .stdout(predicate::str::contains("artifact"))
    .stdout(predicate::str::contains("filterartifact_isa"))
    .stdout(predicate::str::contains("task").not())
    .stdout(predicate::str::contains("iteration").not());
}

#[test]
fn it_scopes_results_to_iterations_with_is_iteration() {
  let env = GestCmd::new();
  env.cmd().args(["task", "create", "filtertask_isi"]).assert().success();
  env.create_artifact("filterartifact_isi", "body");
  env
    .cmd()
    .args(["iteration", "create", "filteriter_isi"])
    .assert()
    .success();

  env
    .cmd()
    .args(["search", "is:iteration filteriter_isi"])
    .assert()
    .success()
    .stdout(predicate::str::contains("iteration"))
    .stdout(predicate::str::contains("filteriter_isi"))
    .stdout(predicate::str::contains("task").not())
    .stdout(predicate::str::contains("artifact").not());
}

// ---------------------------------------------------------------------------
// Tag filtering: tag:<name>
// ---------------------------------------------------------------------------

#[test]
fn it_filters_by_tag() {
  let env = GestCmd::new();
  env.cmd().args(["task", "create", "aaatagged_ft"]).assert().success();
  env.cmd().args(["task", "create", "bbbplain_ft"]).assert().success();

  // Extract the tagged task's ID via JSON search
  let output = env.cmd().args(["search", "--json", "aaatagged_ft"]).output().unwrap();
  let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
  let id = json["tasks"][0]["id"].as_str().unwrap().to_string();
  env.cmd().args(["task", "tag", &id, "urgent"]).assert().success();

  env
    .cmd()
    .args(["search", "tag:urgent"])
    .assert()
    .success()
    .stdout(predicate::str::contains("aaatagged_ft"))
    .stdout(predicate::str::contains("bbbplain_ft").not());
}

// ---------------------------------------------------------------------------
// Status filtering: status:<status>
// ---------------------------------------------------------------------------

#[test]
fn it_filters_tasks_by_status_open() {
  let env = GestCmd::new();
  env.cmd().args(["task", "create", "opentask_sf"]).assert().success();
  env.cmd().args(["task", "create", "progresstask_sf"]).assert().success();

  // Move second task to in-progress
  let output = env
    .cmd()
    .args(["search", "--json", "progresstask_sf"])
    .output()
    .unwrap();
  let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
  let id = json["tasks"][0]["id"].as_str().unwrap().to_string();
  env
    .cmd()
    .args(["task", "update", &id, "--status", "in-progress"])
    .assert()
    .success();

  env
    .cmd()
    .args(["search", "status:open"])
    .assert()
    .success()
    .stdout(predicate::str::contains("opentask_sf"))
    .stdout(predicate::str::contains("progresstask_sf").not());
}

#[test]
fn it_filters_iterations_by_status_active() {
  let env = GestCmd::new();
  env
    .cmd()
    .args(["iteration", "create", "activeiter_sf"])
    .assert()
    .success();

  env
    .cmd()
    .args(["search", "is:iteration status:active activeiter_sf"])
    .assert()
    .success()
    .stdout(predicate::str::contains("activeiter_sf"));
}

// ---------------------------------------------------------------------------
// Type filtering: type:<kind>
// ---------------------------------------------------------------------------

#[test]
fn it_filters_artifacts_by_type() {
  let env = GestCmd::new();
  env
    .cmd()
    .args([
      "artifact",
      "create",
      "--title",
      "specart_tf",
      "--body",
      "body",
      "--type",
      "spec",
    ])
    .assert()
    .success();
  env
    .cmd()
    .args([
      "artifact",
      "create",
      "--title",
      "rfcart_tf",
      "--body",
      "body",
      "--type",
      "rfc",
    ])
    .assert()
    .success();

  env
    .cmd()
    .args(["search", "type:spec"])
    .assert()
    .success()
    .stdout(predicate::str::contains("specart_tf"))
    .stdout(predicate::str::contains("rfcart_tf").not());
}

// ---------------------------------------------------------------------------
// Negation: -<filter>
// ---------------------------------------------------------------------------

#[test]
fn it_excludes_iterations_with_negated_is() {
  let env = GestCmd::new();
  env.cmd().args(["task", "create", "negtask_ni"]).assert().success();
  env.cmd().args(["iteration", "create", "negiter_ni"]).assert().success();

  // Use -- to prevent clap from interpreting the leading dash as a flag
  env
    .cmd()
    .args(["search", "--", "-is:iteration neg"])
    .assert()
    .success()
    .stdout(predicate::str::contains("negtask_ni"))
    .stdout(predicate::str::contains("negiter_ni").not());
}

#[test]
fn it_excludes_entities_with_negated_tag() {
  let env = GestCmd::new();
  env.cmd().args(["task", "create", "keepme_nt"]).assert().success();
  env.cmd().args(["task", "create", "dropme_nt"]).assert().success();

  // Tag the second task with "wip"
  let output = env.cmd().args(["search", "--json", "dropme_nt"]).output().unwrap();
  let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
  let id = json["tasks"][0]["id"].as_str().unwrap().to_string();
  env.cmd().args(["task", "tag", &id, "wip"]).assert().success();

  // Use -- to prevent clap from interpreting the leading dash as a flag
  env
    .cmd()
    .args(["search", "--", "-tag:wip _nt"])
    .assert()
    .success()
    .stdout(predicate::str::contains("keepme_nt"))
    .stdout(predicate::str::contains("dropme_nt").not());
}

// ---------------------------------------------------------------------------
// Filter combinations
// ---------------------------------------------------------------------------

#[test]
fn it_combines_is_and_tag_filters() {
  let env = GestCmd::new();
  env.cmd().args(["task", "create", "combtask_ct"]).assert().success();
  env.create_artifact("combart_ct", "body");

  // Tag the task
  let output = env.cmd().args(["search", "--json", "combtask_ct"]).output().unwrap();
  let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
  let id = json["tasks"][0]["id"].as_str().unwrap().to_string();
  env.cmd().args(["task", "tag", &id, "combo"]).assert().success();

  // Tag the artifact
  let output = env.cmd().args(["search", "--json", "combart_ct"]).output().unwrap();
  let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
  let id = json["artifacts"][0]["id"].as_str().unwrap().to_string();
  env.cmd().args(["artifact", "tag", &id, "combo"]).assert().success();

  // is:task AND tag:combo should only return the task
  env
    .cmd()
    .args(["search", "is:task tag:combo"])
    .assert()
    .success()
    .stdout(predicate::str::contains("combtask_ct"))
    .stdout(predicate::str::contains("combart_ct").not());
}

#[test]
fn it_combines_free_text_with_filters() {
  let env = GestCmd::new();
  env.cmd().args(["task", "create", "alphatask_ft"]).assert().success();
  env.cmd().args(["task", "create", "betatask_ft"]).assert().success();

  env
    .cmd()
    .args(["search", "is:task alphatask_ft"])
    .assert()
    .success()
    .stdout(predicate::str::contains("alphatask_ft"))
    .stdout(predicate::str::contains("betatask_ft").not());
}

// ---------------------------------------------------------------------------
// No filters = current behavior (returns all matching entity types)
// ---------------------------------------------------------------------------

#[test]
fn it_returns_all_entity_types_without_filters() {
  let env = GestCmd::new();
  env.cmd().args(["task", "create", "xyzuni_nf"]).assert().success();
  env.create_artifact("xyzuni_nf", "body");
  env.cmd().args(["iteration", "create", "xyzuni_nf"]).assert().success();

  env
    .cmd()
    .args(["search", "xyzuni_nf"])
    .assert()
    .success()
    .stdout(predicate::str::contains("3 results for"));
}
