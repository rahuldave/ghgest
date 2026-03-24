use chrono::Utc;
use clap::Args;

use crate::{
  config,
  config::Config,
  store,
  ui::{components::TagChange, theme::Theme},
};

/// Add tags to a task
#[derive(Debug, Args)]
pub struct Command {
  /// Task ID or unique prefix
  pub id: String,
  /// Tags to add (space-separated)
  pub tags: Vec<String>,
}

impl Command {
  pub fn call(&self, config: &Config, _theme: &Theme) -> crate::Result<()> {
    let data_dir = config::data_dir(config)?;
    let id = store::resolve_task_id(&data_dir, &self.id, false)?;
    let mut task = store::read_task(&data_dir, &id)?;

    for tag in &self.tags {
      if !task.tags.contains(tag) {
        task.tags.push(tag.clone());
      }
    }

    task.updated_at = Utc::now();
    store::write_task(&data_dir, &task)?;

    TagChange::new("Tagged", "task", &id, &self.tags).write_to(&mut std::io::stdout())?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use chrono::Utc;
  use tempfile::TempDir;

  use super::*;
  use crate::model::{Status, Task};

  mod call {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_tags() {
      let (_dir, config) = setup();
      let task = make_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      store::write_task(_dir.path(), &task).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        tags: vec!["rust".to_string(), "cli".to_string()],
      };
      cmd.call(&config, &Theme::default()).unwrap();

      let loaded = store::read_task(_dir.path(), &task.id).unwrap();
      assert_eq!(loaded.tags, vec!["rust".to_string(), "cli".to_string()]);
    }

    #[test]
    fn it_deduplicates_tags() {
      let (_dir, config) = setup();
      let mut task = make_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      task.tags = vec!["rust".to_string()];
      store::write_task(_dir.path(), &task).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        tags: vec!["rust".to_string(), "cli".to_string()],
      };
      cmd.call(&config, &Theme::default()).unwrap();

      let loaded = store::read_task(_dir.path(), &task.id).unwrap();
      assert_eq!(loaded.tags, vec!["rust".to_string(), "cli".to_string()]);
    }

    #[test]
    fn it_preserves_existing_tags() {
      let (_dir, config) = setup();
      let mut task = make_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      task.tags = vec!["existing".to_string()];
      store::write_task(_dir.path(), &task).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        tags: vec!["new".to_string()],
      };
      cmd.call(&config, &Theme::default()).unwrap();

      let loaded = store::read_task(_dir.path(), &task.id).unwrap();
      assert_eq!(loaded.tags, vec!["existing".to_string(), "new".to_string()]);
    }
  }

  fn make_task(id: &str) -> Task {
    Task {
      resolved_at: None,
      created_at: Utc::now(),
      description: String::new(),
      id: id.parse().unwrap(),
      links: vec![],
      metadata: toml::Table::new(),
      status: Status::Open,
      tags: vec![],
      title: format!("Task {id}"),
      updated_at: Utc::now(),
    }
  }

  fn setup() -> (TempDir, crate::config::Config) {
    let dir = TempDir::new().unwrap();
    let config = crate::config::Config {
      storage: crate::config::StorageConfig {
        data_dir: Some(dir.path().to_path_buf()),
      },
      ..Default::default()
    };
    store::ensure_dirs(dir.path()).unwrap();
    (dir, config)
  }
}
