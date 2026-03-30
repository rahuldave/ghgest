//! Factory helpers for building model instances in tests.

use std::path::PathBuf;

use chrono::Utc;

use crate::{
  config::Settings,
  model::{Artifact, Iteration, Task, iteration, task::Status},
};

/// Create a minimal [`Artifact`] with sensible defaults for the given encoded ID.
pub fn make_test_artifact(id: &str) -> Artifact {
  let now = Utc::now();
  Artifact {
    archived_at: None,
    body: String::new(),
    created_at: now,
    id: id.parse().unwrap(),
    kind: None,
    metadata: yaml_serde::Mapping::new(),
    tags: vec![],
    title: format!("Artifact {id}"),
    updated_at: now,
  }
}

/// Build a [`Settings`] whose `data_dir` points at the given path.
pub fn make_test_config(data_dir: PathBuf) -> Settings {
  toml::from_str(&format!("[storage]\ndata_dir = \"{}\"", data_dir.display())).unwrap()
}

/// Create a minimal [`Iteration`] with sensible defaults for the given encoded ID.
pub fn make_test_iteration(id: &str) -> Iteration {
  let now = Utc::now();
  Iteration {
    completed_at: None,
    created_at: now,
    description: String::new(),
    id: id.parse().unwrap(),
    links: vec![],
    metadata: toml::Table::new(),
    status: iteration::Status::Active,
    tags: vec![],
    tasks: vec![],
    title: format!("Iteration {id}"),
    updated_at: now,
  }
}

/// Create a minimal [`Task`] with sensible defaults for the given encoded ID.
pub fn make_test_task(id: &str) -> Task {
  let now = Utc::now();
  Task {
    assigned_to: None,
    created_at: now,
    description: String::new(),
    id: id.parse().unwrap(),
    links: vec![],
    metadata: toml::Table::new(),
    phase: None,
    priority: None,
    resolved_at: None,
    status: Status::Open,
    tags: vec![],
    title: format!("Task {id}"),
    updated_at: now,
  }
}
