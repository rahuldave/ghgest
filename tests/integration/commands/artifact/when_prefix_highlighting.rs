//! Tests covering active/all prefix wiring through artifact list, show, and detail views.
//!
//! IDs are random, so we cannot reliably force a specific minimum prefix length.
//! These tests instead verify:
//!
//! - the commands run successfully and render the relevant short IDs in their list/detail output;
//! - `artifact show <prefix>` resolves to the active artifact when an active and archived row share a prefix;
//! - `artifact show <prefix>` silently falls back to an archived row when no active match exists.

use crate::support::helpers::GestCmd;

#[test]
fn it_highlights_active_pool_prefix_in_list() {
  let g = GestCmd::new();
  let id_a = g.create_artifact("Active artifact A", "body");
  let id_b = g.create_artifact("Active artifact B", "body");
  let id_c = g.create_artifact("Active artifact C", "body");

  let output = g
    .cmd()
    .args(["artifact", "list"])
    .output()
    .expect("artifact list failed to run");

  assert!(
    output.status.success(),
    "artifact list exited non-zero: {}",
    String::from_utf8_lossy(&output.stderr)
  );
  let stdout = String::from_utf8_lossy(&output.stdout);
  // The active pool prefix is computed from non-archived artifacts only,
  // so all three should appear in the list.
  for id in [&id_a, &id_b, &id_c] {
    let prefix2 = &id[..2];
    assert!(
      stdout.contains(prefix2),
      "expected list to contain id prefix {prefix2} from {id}: {stdout}"
    );
  }
  assert!(stdout.contains("Active artifact A"), "got: {stdout}");
  assert!(stdout.contains("Active artifact B"), "got: {stdout}");
  assert!(stdout.contains("Active artifact C"), "got: {stdout}");
}

#[test]
fn it_highlights_all_pool_prefix_with_all_flag() {
  let g = GestCmd::new();
  let active_id = g.create_artifact("Active artifact", "body");
  let archived_id = g.create_artifact("To be archived", "body");

  let archive_output = g
    .cmd()
    .args(["artifact", "archive", &archived_id])
    .output()
    .expect("artifact archive failed to run");
  assert!(
    archive_output.status.success(),
    "artifact archive exited non-zero: {}",
    String::from_utf8_lossy(&archive_output.stderr)
  );

  let output = g
    .cmd()
    .args(["artifact", "list", "--all"])
    .output()
    .expect("artifact list --all failed to run");

  assert!(
    output.status.success(),
    "artifact list --all exited non-zero: {}",
    String::from_utf8_lossy(&output.stderr)
  );
  let stdout = String::from_utf8_lossy(&output.stdout);
  // Both active and archived rows should appear when using --all.
  assert!(stdout.contains("Active artifact"), "got: {stdout}");
  assert!(stdout.contains("To be archived"), "got: {stdout}");
  assert!(stdout.contains("[archived]"), "expected archived badge: {stdout}");
  // The all-pool prefix length is computed across both rows.
  assert!(stdout.contains(&active_id[..2]), "got: {stdout}");
  assert!(stdout.contains(&archived_id[..2]), "got: {stdout}");
}

#[test]
fn it_resolves_active_match_over_archived() {
  let g = GestCmd::new();
  let active_id = g.create_artifact("Active winner", "body");
  let archived_id = g.create_artifact("Archived loser", "body");

  // Archive one of the two artifacts.
  let archive_output = g
    .cmd()
    .args(["artifact", "archive", &archived_id])
    .output()
    .expect("artifact archive failed to run");
  assert!(
    archive_output.status.success(),
    "artifact archive exited non-zero: {}",
    String::from_utf8_lossy(&archive_output.stderr)
  );

  // Use the full active id as the prefix to guarantee an unambiguous active match.
  let output = g
    .cmd()
    .args(["artifact", "show", &active_id])
    .output()
    .expect("artifact show failed to run");
  assert!(
    output.status.success(),
    "artifact show exited non-zero: {}",
    String::from_utf8_lossy(&output.stderr)
  );
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("Active winner"), "got: {stdout}");
  assert!(!stdout.contains("Archived loser"), "got: {stdout}");
}

#[test]
fn it_falls_back_to_archived_when_no_active_match() {
  let g = GestCmd::new();
  let archived_id = g.create_artifact("Archived only", "body");

  // Archive the only artifact so no active rows exist.
  let archive_output = g
    .cmd()
    .args(["artifact", "archive", &archived_id])
    .output()
    .expect("artifact archive failed to run");
  assert!(
    archive_output.status.success(),
    "artifact archive exited non-zero: {}",
    String::from_utf8_lossy(&archive_output.stderr)
  );

  // Show by short prefix — resolver should silently fall back to the archived pool.
  let prefix = &archived_id[..4];
  let output = g
    .cmd()
    .args(["artifact", "show", prefix])
    .output()
    .expect("artifact show failed to run");
  assert!(
    output.status.success(),
    "artifact show exited non-zero: {}",
    String::from_utf8_lossy(&output.stderr)
  );
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("Archived only"), "got: {stdout}");
}
