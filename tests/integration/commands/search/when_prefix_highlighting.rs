use std::collections::HashSet;

use crate::support::helpers::{GestCmd, rendered_prefix_len, strip_ansi};

/// Compute the minimum unique prefix length over a set of IDs (each truncated
/// to at most 8 characters), matching `ui::components::atoms::id::min_unique_prefix`.
fn min_unique_prefix(ids: &[String]) -> usize {
  let shorts: Vec<String> = ids.iter().map(|id| id.chars().take(8).collect()).collect();
  for len in 1..=8 {
    let prefixes: HashSet<&str> = shorts.iter().map(|s| &s[..len.min(s.len())]).collect();
    if prefixes.len() == shorts.len() {
      return len;
    }
  }
  8
}

#[test]
fn it_highlights_per_entity_pool_prefixes_in_search() {
  let g = GestCmd::new();

  // Seed several entities of each type, all containing the search term.
  let task_ids: Vec<String> = (0..6).map(|i| g.create_task(&format!("needle task {i}"))).collect();
  let artifact_ids: Vec<String> = (0..5)
    .map(|i| g.create_artifact(&format!("needle artifact {i}"), &format!("body {i}")))
    .collect();
  let iteration_ids: Vec<String> = (0..4)
    .map(|i| g.create_iteration(&format!("needle iteration {i}")))
    .collect();

  let expected_task_prefix = min_unique_prefix(&task_ids);
  let expected_artifact_prefix = min_unique_prefix(&artifact_ids);
  let expected_iteration_prefix = min_unique_prefix(&iteration_ids);

  // Run search with colors forced on so prefix highlighting is observable.
  let output = g
    .raw_cmd()
    .env("CLICOLOR_FORCE", "1")
    .args(["search", "needle"])
    .output()
    .expect("search failed to run");

  assert!(
    output.status.success(),
    "search exited non-zero: {}",
    String::from_utf8_lossy(&output.stderr)
  );

  let stdout = String::from_utf8_lossy(&output.stdout).to_string();
  let plain = strip_ansi(&stdout);

  // Sanity: every seeded entity should appear in stripped output.
  for id in task_ids.iter().chain(artifact_ids.iter()).chain(iteration_ids.iter()) {
    let short: String = id.chars().take(8).collect();
    assert!(plain.contains(&short), "missing id {short} in search output:\n{plain}");
  }

  // Verify per-entity prefix lengths in colored output.
  for id in &task_ids {
    let short: String = id.chars().take(8).collect();
    let got = rendered_prefix_len(&stdout, &short)
      .unwrap_or_else(|| panic!("could not find rendered task id {short} in:\n{stdout}"));
    assert_eq!(
      got, expected_task_prefix,
      "task id {short}: expected prefix_len {expected_task_prefix}, got {got}"
    );
  }

  for id in &artifact_ids {
    let short: String = id.chars().take(8).collect();
    let got = rendered_prefix_len(&stdout, &short)
      .unwrap_or_else(|| panic!("could not find rendered artifact id {short} in:\n{stdout}"));
    assert_eq!(
      got, expected_artifact_prefix,
      "artifact id {short}: expected prefix_len {expected_artifact_prefix}, got {got}"
    );
  }

  for id in &iteration_ids {
    let short: String = id.chars().take(8).collect();
    let got = rendered_prefix_len(&stdout, &short)
      .unwrap_or_else(|| panic!("could not find rendered iteration id {short} in:\n{stdout}"));
    assert_eq!(
      got, expected_iteration_prefix,
      "iteration id {short}: expected prefix_len {expected_iteration_prefix}, got {got}"
    );
  }
}
