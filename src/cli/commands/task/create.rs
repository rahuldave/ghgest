use std::io::IsTerminal;

use clap::Args;

use crate::{
  config,
  config::Config,
  model::{NewTask, Status},
  store,
  ui::{components::Confirmation, theme::Theme},
};

/// Create a new task
#[derive(Debug, Args)]
pub struct Command {
  /// Task title
  pub title: String,
  /// Description text (opens $EDITOR if omitted and stdin is a terminal)
  #[arg(short, long)]
  pub description: Option<String>,
  /// Key=value metadata pair (repeatable, e.g. -m key=value)
  #[arg(short, long)]
  pub metadata: Vec<String>,
  /// Initial status: open, in-progress, done, or cancelled (default: open)
  #[arg(short, long)]
  pub status: Option<String>,
  /// Comma-separated list of tags
  #[arg(long)]
  pub tags: Option<String>,
}

impl Command {
  pub fn call(&self, config: &Config, theme: &Theme) -> crate::Result<()> {
    let status = match &self.status {
      Some(s) => s.parse::<Status>().map_err(crate::Error::generic)?,
      None => Status::Open,
    };

    let metadata = parse_metadata(&self.metadata)?;

    let tags = match &self.tags {
      Some(t) => t
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect(),
      None => vec![],
    };

    let description = self.read_description()?;

    let new = NewTask {
      description,
      links: vec![],
      metadata,
      status,
      tags,
      title: self.title.clone(),
    };

    let data_dir = config::data_dir(config)?;
    let task = store::create_task(&data_dir, new)?;
    Confirmation::new("Created", "task", &task.id).write_to(&mut std::io::stdout(), theme)?;
    Ok(())
  }

  fn read_description(&self) -> crate::Result<String> {
    if let Some(ref desc) = self.description {
      return Ok(desc.clone());
    }

    if std::io::stdin().is_terminal()
      && let Some(_editor) = crate::cli::editor::resolve_editor()
    {
      let content = crate::cli::editor::edit_temp(None, ".md")?;
      if content.trim().is_empty() {
        return Err(crate::Error::generic("Aborting: empty description"));
      }
      return Ok(content);
    }

    Ok(String::new())
  }
}

fn parse_metadata(entries: &[String]) -> crate::Result<toml::Table> {
  let mut table = toml::Table::new();
  for entry in entries {
    let (key, value) = entry
      .split_once('=')
      .ok_or_else(|| crate::Error::generic(format!("Invalid metadata format '{entry}', expected key=value")))?;
    table.insert(key.to_string(), toml::Value::String(value.to_string()));
  }
  Ok(table)
}

#[cfg(test)]
mod tests {
  use super::*;

  mod call {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{
      config::{Config, StorageConfig},
      store,
    };

    #[test]
    fn it_resolves_task_created_with_cancelled_status() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_config(dir.path());

      let cmd = Command {
        title: "Cancelled Task".to_string(),
        description: Some("Cancelled".to_string()),

        metadata: vec![],
        status: Some("cancelled".to_string()),
        tags: None,
      };

      cmd.call(&config, &Theme::default()).unwrap();

      let filter = crate::model::TaskFilter::default();
      let tasks = store::list_tasks(dir.path(), &filter).unwrap();
      assert_eq!(tasks.len(), 0);

      let filter = crate::model::TaskFilter {
        all: true,
        ..Default::default()
      };
      let tasks = store::list_tasks(dir.path(), &filter).unwrap();
      assert_eq!(tasks.len(), 1);
      assert_eq!(tasks[0].status, Status::Cancelled);
      assert!(tasks[0].resolved_at.is_some());
    }

    #[test]
    fn it_resolves_task_created_with_done_status() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_config(dir.path());

      let cmd = Command {
        title: "Done Task".to_string(),
        description: Some("Already done".to_string()),

        metadata: vec![],
        status: Some("done".to_string()),
        tags: None,
      };

      cmd.call(&config, &Theme::default()).unwrap();

      // Should not appear in active tasks
      let filter = crate::model::TaskFilter::default();
      let tasks = store::list_tasks(dir.path(), &filter).unwrap();
      assert_eq!(tasks.len(), 0);

      // Should appear when including all
      let filter = crate::model::TaskFilter {
        all: true,
        ..Default::default()
      };
      let tasks = store::list_tasks(dir.path(), &filter).unwrap();
      assert_eq!(tasks.len(), 1);
      assert_eq!(tasks[0].title, "Done Task");
      assert_eq!(tasks[0].status, Status::Done);
      assert!(tasks[0].resolved_at.is_some());
    }

    #[test]
    fn it_creates_a_task_with_all_flags() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_config(dir.path());

      let cmd = Command {
        title: "Full Task".to_string(),
        description: Some("A description".to_string()),
        metadata: vec!["priority=high".to_string()],
        status: Some("in-progress".to_string()),
        tags: Some("rust,cli".to_string()),
      };

      cmd.call(&config, &Theme::default()).unwrap();

      let filter = crate::model::TaskFilter::default();
      let tasks = store::list_tasks(dir.path(), &filter).unwrap();
      assert_eq!(tasks.len(), 1);

      let task = &tasks[0];
      assert_eq!(task.title, "Full Task");
      assert_eq!(task.description, "A description");
      assert_eq!(task.status, Status::InProgress);
      assert_eq!(task.tags, vec!["rust", "cli"]);
      assert_eq!(task.links.len(), 0);
      assert_eq!(task.metadata.get("priority").unwrap().as_str().unwrap(), "high");
    }

    #[test]
    fn it_creates_a_task_with_defaults() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_config(dir.path());

      let cmd = Command {
        title: "My Task".to_string(),
        description: None,

        metadata: vec![],
        status: None,
        tags: None,
      };

      cmd.call(&config, &Theme::default()).unwrap();

      let filter = crate::model::TaskFilter::default();
      let tasks = store::list_tasks(dir.path(), &filter).unwrap();
      assert_eq!(tasks.len(), 1);
      assert_eq!(tasks[0].title, "My Task");
      assert_eq!(tasks[0].status, Status::Open);
      assert!(tasks[0].description.is_empty());
    }

    fn make_config(dir: &std::path::Path) -> Config {
      store::ensure_dirs(dir).unwrap();
      Config {
        storage: StorageConfig {
          data_dir: Some(dir.to_path_buf()),
        },
        ..Config::default()
      }
    }
  }

  mod parse_metadata {
    use pretty_assertions::assert_eq;

    #[test]
    fn it_errors_on_missing_equals() {
      let entries = vec!["no-equals".to_string()];
      let result = super::parse_metadata(&entries);
      assert!(result.is_err());
    }

    #[test]
    fn it_parses_key_value_pairs() {
      let entries = vec!["priority=high".to_string(), "team=backend".to_string()];
      let table = super::parse_metadata(&entries).unwrap();
      assert_eq!(table.get("priority").unwrap().as_str().unwrap(), "high");
      assert_eq!(table.get("team").unwrap().as_str().unwrap(), "backend");
    }
  }
}
