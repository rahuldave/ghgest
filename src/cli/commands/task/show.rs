use std::path::Path;

use clap::Args;

use crate::{
  cli, store,
  ui::{theme::Theme, views::task::TaskDetailView},
};

/// Display a task's full details, description, and links.
#[derive(Debug, Args)]
pub struct Command {
  /// Task ID or unique prefix.
  pub id: String,
  /// Output task details as JSON.
  #[arg(short, long)]
  pub json: bool,
}

impl Command {
  /// Resolve the task by ID prefix and render its detail view.
  pub fn call(&self, data_dir: &Path, theme: &Theme) -> cli::Result<()> {
    let id = store::resolve_task_id(data_dir, &self.id, true)?;
    let task = store::read_task(data_dir, &id)?;

    if self.json {
      let json = serde_json::to_string_pretty(&task).map_err(|e| cli::Error::generic(e.to_string()))?;
      println!("{json}");
      return Ok(());
    }

    let id_str = task.id.to_string();
    let status_str = match task.status {
      crate::model::task::Status::Open => "open",
      crate::model::task::Status::InProgress => "in-progress",
      crate::model::task::Status::Done => "done",
      crate::model::task::Status::Cancelled => "cancelled",
    };

    let link_strings: Vec<(String, String)> = task
      .links
      .iter()
      .map(|l| {
        let rel = l.rel.to_string();
        let target = l.ref_.rsplit('/').next().unwrap_or(&l.ref_).to_string();
        (rel, target)
      })
      .collect();

    let links: Vec<(&str, &str)> = link_strings.iter().map(|(r, t)| (r.as_str(), t.as_str())).collect();

    let body = if task.description.is_empty() {
      None
    } else {
      Some(task.description.as_str())
    };

    let view = TaskDetailView {
      id: &id_str,
      title: &task.title,
      status: status_str,
      priority: task.priority,
      phase: task.phase.map(|p| (p as u32, None)),
      assigned: task.assigned_to.as_deref(),
      tags: &task.tags,
      links,
      body,
      theme,
    };
    println!("{view}");

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    model::link::{Link, RelationshipType},
    store,
    test_helpers::{make_test_config, make_test_task},
  };

  mod call {
    use super::*;

    #[test]
    fn it_resolves_resolved_task() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();
      let mut task = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      task.description = "A test description".to_string();
      task.tags = vec!["rust".to_string()];
      task.links = vec![Link {
        ref_: "https://example.com".to_string(),
        rel: RelationshipType::RelatesTo,
      }];
      task.title = "Test Task".to_string();
      store::write_task(&data_dir, &task).unwrap();
      store::resolve_task(&data_dir, &task.id).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        json: false,
      };

      cmd.call(&data_dir, &Theme::default()).unwrap();
    }

    #[test]
    fn it_shows_task_as_json() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();
      let task = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      store::write_task(&data_dir, &task).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        json: true,
      };

      cmd.call(&data_dir, &Theme::default()).unwrap();
    }

    #[test]
    fn it_shows_task_detail() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();
      let task = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      store::write_task(&data_dir, &task).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        json: false,
      };

      cmd.call(&data_dir, &Theme::default()).unwrap();
    }
  }
}
