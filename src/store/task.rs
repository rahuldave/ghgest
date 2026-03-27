use std::{
  fs,
  path::{Path, PathBuf},
};

use chrono::Utc;

use super::fs::{ensure_dirs, read_dir_files, resolve_id};
use crate::model::{Id, NewTask, Task, TaskFilter, TaskPatch};

pub fn create_task(data_dir: &Path, new: NewTask) -> crate::Result<Task> {
  let now = Utc::now();
  let task = Task {
    created_at: now,
    description: new.description,
    id: Id::new(),
    links: new.links,
    metadata: new.metadata,
    resolved_at: None,
    status: new.status,
    tags: new.tags,
    title: new.title,
    updated_at: now,
  };

  write_task(data_dir, &task)?;

  if task.status.is_terminal() {
    resolve_task(data_dir, &task.id)?;
    return read_task(data_dir, &task.id);
  }

  Ok(task)
}

pub fn is_task_resolved(data_dir: &Path, id: &Id) -> bool {
  let resolved_path = data_dir.join(format!("tasks/resolved/{id}.toml"));
  let active_path = data_dir.join(format!("tasks/{id}.toml"));
  resolved_path.exists() && !active_path.exists()
}

pub fn list_tasks(data_dir: &Path, filter: &TaskFilter) -> crate::Result<Vec<Task>> {
  let mut tasks = Vec::new();

  for path in read_dir_files(&data_dir.join("tasks"), "toml")? {
    let content = fs::read_to_string(&path)?;
    let task: Task = toml::from_str(&content)?;
    tasks.push(task);
  }

  if filter.all {
    for path in read_dir_files(&data_dir.join("tasks/resolved"), "toml")? {
      let content = fs::read_to_string(&path)?;
      let task: Task = toml::from_str(&content)?;
      tasks.push(task);
    }
  }

  tasks.retain(|task| {
    if let Some(ref status) = filter.status
      && &task.status != status
    {
      return false;
    }
    if let Some(ref tag) = filter.tag
      && !task.tags.contains(tag)
    {
      return false;
    }
    true
  });

  Ok(tasks)
}

pub fn read_task(data_dir: &Path, id: &Id) -> crate::Result<Task> {
  let active = data_dir.join(format!("tasks/{id}.toml"));
  let resolved = data_dir.join(format!("tasks/resolved/{id}.toml"));

  let path = if active.exists() {
    active
  } else if resolved.exists() {
    log::debug!("reading resolved task {id}");
    resolved
  } else {
    return Err(crate::Error::generic(format!("Task not found: '{id}'")));
  };

  log::trace!("reading task from {}", path.display());
  let content = fs::read_to_string(path)?;
  let task: Task = toml::from_str(&content)?;
  Ok(task)
}

pub fn resolve_task(data_dir: &Path, id: &Id) -> crate::Result<()> {
  let mut task = read_task(data_dir, id)?;
  let now = Utc::now();
  task.resolved_at = Some(now);
  task.updated_at = now;

  ensure_dirs(data_dir)?;
  let content = toml::to_string(&task)?;
  let resolved_path = data_dir.join(format!("tasks/resolved/{id}.toml"));
  fs::write(resolved_path, content)?;

  let active_path = data_dir.join(format!("tasks/{id}.toml"));
  if active_path.exists() {
    fs::remove_file(active_path)?;
  }

  Ok(())
}

pub fn resolve_task_id(data_dir: &Path, prefix: &str, include_resolved: bool) -> crate::Result<Id> {
  log::debug!("resolving task ID prefix '{prefix}'");
  resolve_id(
    &data_dir.join("tasks"),
    Some(&data_dir.join("tasks/resolved")),
    "toml",
    prefix,
    include_resolved,
    "Task",
  )
}

#[allow(dead_code)]
pub fn task_path(data_dir: &Path, id: &Id) -> PathBuf {
  let active = data_dir.join(format!("tasks/{id}.toml"));
  let resolved = data_dir.join(format!("tasks/resolved/{id}.toml"));
  if resolved.exists() && !active.exists() {
    resolved
  } else {
    active
  }
}

#[allow(dead_code)]
pub fn unresolve_task(data_dir: &Path, id: &Id) -> crate::Result<()> {
  let mut task = read_task(data_dir, id)?;
  task.resolved_at = None;
  task.updated_at = Utc::now();

  ensure_dirs(data_dir)?;
  let content = toml::to_string(&task)?;
  let active_path = data_dir.join(format!("tasks/{id}.toml"));
  fs::write(active_path, content)?;

  let resolved_path = data_dir.join(format!("tasks/resolved/{id}.toml"));
  if resolved_path.exists() {
    fs::remove_file(resolved_path)?;
  }

  Ok(())
}

pub fn update_task(data_dir: &Path, id: &Id, patch: TaskPatch) -> crate::Result<Task> {
  let mut task = read_task(data_dir, id)?;
  let was_resolved = is_task_resolved(data_dir, id);

  if let Some(description) = patch.description {
    task.description = description;
  }
  if let Some(metadata) = patch.metadata {
    task.metadata = metadata;
  }
  if let Some(status) = patch.status {
    task.status = status;
  }
  if let Some(tags) = patch.tags {
    task.tags = tags;
  }
  if let Some(title) = patch.title {
    task.title = title;
  }

  task.updated_at = Utc::now();

  if task.status.is_terminal() && !was_resolved {
    // Write then resolve — but resolve_task would re-read from disk.
    // Instead, do the resolve inline using the already-read task data.
    task.resolved_at = Some(task.updated_at);

    ensure_dirs(data_dir)?;
    let content = toml::to_string(&task)?;
    let resolved_path = data_dir.join(format!("tasks/resolved/{id}.toml"));
    fs::write(resolved_path, content)?;

    let active_path = data_dir.join(format!("tasks/{id}.toml"));
    if active_path.exists() {
      fs::remove_file(active_path)?;
    }
  } else if !task.status.is_terminal() && was_resolved {
    // Unresolve inline — avoid re-reading from disk.
    task.resolved_at = None;

    ensure_dirs(data_dir)?;
    let content = toml::to_string(&task)?;
    let active_path = data_dir.join(format!("tasks/{id}.toml"));
    fs::write(active_path, content)?;

    let resolved_path = data_dir.join(format!("tasks/resolved/{id}.toml"));
    if resolved_path.exists() {
      fs::remove_file(resolved_path)?;
    }
  } else {
    write_task(data_dir, &task)?;
  }

  Ok(task)
}

pub fn write_task(data_dir: &Path, task: &Task) -> crate::Result<()> {
  ensure_dirs(data_dir)?;
  let content = toml::to_string(task)?;
  let path = data_dir.join(format!("tasks/{}.toml", task.id));
  log::trace!("writing task {} to {}", task.id, path.display());
  fs::write(path, content)?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use chrono::Utc;

  use crate::model::{Task, TaskFilter, link::Link, task::Status};

  fn make_test_task(id: &str, title: &str) -> Task {
    Task {
      created_at: Utc::now(),
      description: String::new(),
      id: id.parse().unwrap(),
      links: vec![],
      metadata: toml::Table::new(),
      resolved_at: None,
      status: Status::Open,
      tags: vec![],
      title: title.to_string(),
      updated_at: Utc::now(),
    }
  }

  mod is_task_resolved {
    use super::*;

    #[test]
    fn it_returns_false_for_active_task() {
      let dir = tempfile::tempdir().unwrap();
      let task = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk", "Active");
      crate::store::write_task(dir.path(), &task).unwrap();

      assert!(!crate::store::is_task_resolved(dir.path(), &task.id));
    }

    #[test]
    fn it_returns_true_for_resolved_task() {
      let dir = tempfile::tempdir().unwrap();
      let task = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk", "Resolved");
      crate::store::write_task(dir.path(), &task).unwrap();
      crate::store::resolve_task(dir.path(), &task.id).unwrap();

      assert!(crate::store::is_task_resolved(dir.path(), &task.id));
    }
  }

  mod resolve_task {
    use super::*;

    #[test]
    fn it_moves_file_to_resolved() {
      let dir = tempfile::tempdir().unwrap();
      let task = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk", "To Resolve");
      crate::store::write_task(dir.path(), &task).unwrap();

      assert!(dir.path().join("tasks/zyxwvutsrqponmlkzyxwvutsrqponmlk.toml").exists());
      crate::store::resolve_task(dir.path(), &task.id).unwrap();

      assert!(!dir.path().join("tasks/zyxwvutsrqponmlkzyxwvutsrqponmlk.toml").exists());
      assert!(
        dir
          .path()
          .join("tasks/resolved/zyxwvutsrqponmlkzyxwvutsrqponmlk.toml")
          .exists()
      );
    }

    #[test]
    fn it_sets_resolved_at() {
      let dir = tempfile::tempdir().unwrap();
      let task = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk", "To Resolve");
      crate::store::write_task(dir.path(), &task).unwrap();

      crate::store::resolve_task(dir.path(), &task.id).unwrap();

      let loaded = crate::store::read_task(dir.path(), &task.id).unwrap();
      assert!(loaded.resolved_at.is_some());
    }
  }

  mod list_tasks {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_excludes_resolved_by_default() {
      let dir = tempfile::tempdir().unwrap();
      let task = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk", "Active Task");
      crate::store::write_task(dir.path(), &task).unwrap();
      crate::store::resolve_task(dir.path(), &task.id).unwrap();

      let filter = TaskFilter::default();
      let tasks = crate::store::list_tasks(dir.path(), &filter).unwrap();
      assert_eq!(tasks.len(), 0);
    }

    #[test]
    fn it_filters_by_status() {
      let dir = tempfile::tempdir().unwrap();
      let mut task1 = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk", "Open Task");
      task1.status = Status::Open;
      let mut task2 = make_test_task("kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk", "Done Task");
      task2.status = Status::Done;
      crate::store::write_task(dir.path(), &task1).unwrap();
      crate::store::write_task(dir.path(), &task2).unwrap();

      let filter = TaskFilter {
        status: Some(Status::Done),
        ..Default::default()
      };
      let tasks = crate::store::list_tasks(dir.path(), &filter).unwrap();
      assert_eq!(tasks.len(), 1);
      assert_eq!(tasks[0].title, "Done Task");
    }

    #[test]
    fn it_filters_by_tag() {
      let dir = tempfile::tempdir().unwrap();
      let mut task1 = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk", "Tagged");
      task1.tags = vec!["rust".to_string()];
      let task2 = make_test_task("kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk", "Untagged");
      crate::store::write_task(dir.path(), &task1).unwrap();
      crate::store::write_task(dir.path(), &task2).unwrap();

      let filter = TaskFilter {
        tag: Some("rust".to_string()),
        ..Default::default()
      };
      let tasks = crate::store::list_tasks(dir.path(), &filter).unwrap();
      assert_eq!(tasks.len(), 1);
      assert_eq!(tasks[0].title, "Tagged");
    }

    #[test]
    fn it_includes_resolved_when_all() {
      let dir = tempfile::tempdir().unwrap();
      let task = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk", "Active Task");
      crate::store::write_task(dir.path(), &task).unwrap();
      crate::store::resolve_task(dir.path(), &task.id).unwrap();

      let filter = TaskFilter {
        all: true,
        ..Default::default()
      };
      let tasks = crate::store::list_tasks(dir.path(), &filter).unwrap();
      assert_eq!(tasks.len(), 1);
    }

    #[test]
    fn it_returns_active_tasks() {
      let dir = tempfile::tempdir().unwrap();
      let task1 = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk", "Task One");
      let task2 = make_test_task("kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk", "Task Two");
      crate::store::write_task(dir.path(), &task1).unwrap();
      crate::store::write_task(dir.path(), &task2).unwrap();

      let filter = TaskFilter::default();
      let tasks = crate::store::list_tasks(dir.path(), &filter).unwrap();
      assert_eq!(tasks.len(), 2);
    }
  }

  mod resolve_task_id {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_errors_on_ambiguous_prefix() {
      let dir = tempfile::tempdir().unwrap();
      let task1 = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk", "Task 1");
      let task2 = make_test_task("zyxwkkkkkkkkkkkkkkkkkkkkkkkkkkkk", "Task 2");
      crate::store::write_task(dir.path(), &task1).unwrap();
      crate::store::write_task(dir.path(), &task2).unwrap();

      let result = crate::store::resolve_task_id(dir.path(), "zyxw", false);
      assert!(result.is_err());
      let err = result.unwrap_err().to_string();
      assert!(err.contains("Ambiguous"), "Expected ambiguous error, got: {err}");
    }

    #[test]
    fn it_errors_on_not_found() {
      let dir = tempfile::tempdir().unwrap();
      crate::store::ensure_dirs(dir.path()).unwrap();

      let result = crate::store::resolve_task_id(dir.path(), "nnnn", false);
      assert!(result.is_err());
      let err = result.unwrap_err().to_string();
      assert!(err.contains("not found"), "Expected not found error, got: {err}");
      assert!(err.contains("--all"), "Expected --all hint, got: {err}");
    }

    #[test]
    fn it_resolves_unique_prefix() {
      let dir = tempfile::tempdir().unwrap();
      let task = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk", "Test");
      crate::store::write_task(dir.path(), &task).unwrap();

      let resolved = crate::store::resolve_task_id(dir.path(), "zyxw", false).unwrap();
      assert_eq!(resolved.to_string(), "zyxwvutsrqponmlkzyxwvutsrqponmlk");
    }

    #[test]
    fn it_prefers_active_over_resolved_with_shared_prefix() {
      let dir = tempfile::tempdir().unwrap();
      let active = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk", "Active");
      let to_resolve = make_test_task("zyxwkkkkkkkkkkkkkkkkkkkkkkkkkkkk", "Resolved");
      crate::store::write_task(dir.path(), &active).unwrap();
      crate::store::write_task(dir.path(), &to_resolve).unwrap();
      crate::store::resolve_task(dir.path(), &to_resolve.id).unwrap();

      let resolved = crate::store::resolve_task_id(dir.path(), "zyxw", true).unwrap();
      assert_eq!(resolved.to_string(), "zyxwvutsrqponmlkzyxwvutsrqponmlk");
    }

    #[test]
    fn it_falls_back_to_resolved_when_no_active_match() {
      let dir = tempfile::tempdir().unwrap();
      let task = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk", "Resolved");
      crate::store::write_task(dir.path(), &task).unwrap();
      crate::store::resolve_task(dir.path(), &task.id).unwrap();

      let resolved = crate::store::resolve_task_id(dir.path(), "zyxw", true).unwrap();
      assert_eq!(resolved.to_string(), "zyxwvutsrqponmlkzyxwvutsrqponmlk");
    }

    #[test]
    fn it_errors_not_found_for_resolved_when_not_included() {
      let dir = tempfile::tempdir().unwrap();
      let task = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk", "Resolved");
      crate::store::write_task(dir.path(), &task).unwrap();
      crate::store::resolve_task(dir.path(), &task.id).unwrap();

      let result = crate::store::resolve_task_id(dir.path(), "zyxw", false);
      assert!(result.is_err());
      let err = result.unwrap_err().to_string();
      assert!(err.contains("not found"), "Expected not found error, got: {err}");
      assert!(err.contains("--all"), "Expected --all hint, got: {err}");
    }

    #[test]
    fn it_errors_ambiguous_for_multiple_active_without_checking_resolved() {
      let dir = tempfile::tempdir().unwrap();
      let task1 = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk", "Active 1");
      let task2 = make_test_task("zyxwkkkkkkkkkkkkkkkkkkkkkkkkkkkk", "Active 2");
      let task3 = make_test_task("zyxwmmmmmmmmmmmmmmmmmmmmmmmmmmmm", "Resolved");
      crate::store::write_task(dir.path(), &task1).unwrap();
      crate::store::write_task(dir.path(), &task2).unwrap();
      crate::store::write_task(dir.path(), &task3).unwrap();
      crate::store::resolve_task(dir.path(), &task3.id).unwrap();

      let result = crate::store::resolve_task_id(dir.path(), "zyxw", true);
      assert!(result.is_err());
      let err = result.unwrap_err().to_string();
      assert!(err.contains("Ambiguous"), "Expected ambiguous error, got: {err}");
      assert!(
        !err.contains("zyxwmmmmmmmmmmmmmmmmmmmmmmmmmmmm"),
        "Should not include archived ID in error: {err}"
      );
    }

    #[test]
    fn it_errors_ambiguous_for_multiple_resolved_matches() {
      let dir = tempfile::tempdir().unwrap();
      let task1 = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk", "Resolved 1");
      let task2 = make_test_task("zyxwkkkkkkkkkkkkkkkkkkkkkkkkkkkk", "Resolved 2");
      crate::store::write_task(dir.path(), &task1).unwrap();
      crate::store::write_task(dir.path(), &task2).unwrap();
      crate::store::resolve_task(dir.path(), &task1.id).unwrap();
      crate::store::resolve_task(dir.path(), &task2.id).unwrap();

      let result = crate::store::resolve_task_id(dir.path(), "zyxw", true);
      assert!(result.is_err());
      let err = result.unwrap_err().to_string();
      assert!(err.contains("Ambiguous"), "Expected ambiguous error, got: {err}");
    }
  }

  mod task_io {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_roundtrips_task_with_links_and_metadata() {
      let dir = tempfile::tempdir().unwrap();
      let mut task = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk", "Test Task");
      task.description = "A description".to_string();
      task.links = vec![Link {
        ref_: "https://example.com".to_string(),
        rel: crate::model::RelationshipType::RelatesTo,
      }];
      task.metadata = {
        let mut table = toml::Table::new();
        table.insert("priority".to_string(), toml::Value::String("high".to_string()));
        table
      };
      task.tags = vec!["rust".to_string(), "test".to_string()];

      crate::store::write_task(dir.path(), &task).unwrap();
      let loaded = crate::store::read_task(dir.path(), &task.id).unwrap();

      assert_eq!(task, loaded);
    }
  }

  mod unresolve_task {
    use super::*;

    #[test]
    fn it_clears_resolved_at() {
      let dir = tempfile::tempdir().unwrap();
      let task = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk", "To Unresolve");
      crate::store::write_task(dir.path(), &task).unwrap();
      crate::store::resolve_task(dir.path(), &task.id).unwrap();

      let resolved = crate::store::read_task(dir.path(), &task.id).unwrap();
      assert!(resolved.resolved_at.is_some());

      crate::store::unresolve_task(dir.path(), &task.id).unwrap();

      let unresolved = crate::store::read_task(dir.path(), &task.id).unwrap();
      assert!(unresolved.resolved_at.is_none());
    }

    #[test]
    fn it_moves_file_from_resolved_to_active() {
      let dir = tempfile::tempdir().unwrap();
      let task = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk", "To Unresolve");
      crate::store::write_task(dir.path(), &task).unwrap();
      crate::store::resolve_task(dir.path(), &task.id).unwrap();

      assert!(!dir.path().join("tasks/zyxwvutsrqponmlkzyxwvutsrqponmlk.toml").exists());
      assert!(
        dir
          .path()
          .join("tasks/resolved/zyxwvutsrqponmlkzyxwvutsrqponmlk.toml")
          .exists()
      );

      crate::store::unresolve_task(dir.path(), &task.id).unwrap();

      assert!(dir.path().join("tasks/zyxwvutsrqponmlkzyxwvutsrqponmlk.toml").exists());
      assert!(
        !dir
          .path()
          .join("tasks/resolved/zyxwvutsrqponmlkzyxwvutsrqponmlk.toml")
          .exists()
      );
    }
  }
}
