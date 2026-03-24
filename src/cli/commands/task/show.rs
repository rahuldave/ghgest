use clap::Args;

use crate::{
  config,
  config::Config,
  store,
  ui::{components::TaskDetail, theme::Theme},
};

/// Display a task's full details, description, and links
#[derive(Debug, Args)]
pub struct Command {
  /// Task ID or unique prefix
  pub id: String,
  /// Output task details as JSON
  #[arg(short, long)]
  pub json: bool,
}

impl Command {
  pub fn call(&self, config: &Config, theme: &Theme) -> crate::Result<()> {
    let data_dir = config::data_dir(config)?;
    let id = store::resolve_task_id(&data_dir, &self.id, true)?;
    let task = store::read_task(&data_dir, &id)?;

    if self.json {
      let json = serde_json::to_string_pretty(&task)?;
      println!("{json}");
      return Ok(());
    }

    TaskDetail::new(&task).write_to(&mut std::io::stdout(), theme)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use chrono::Utc;

  use super::*;
  use crate::{
    config::{Config, StorageConfig},
    model::{Link, Status, Task},
    store,
  };

  mod call {
    use super::*;

    #[test]
    fn it_resolves_resolved_task() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_config(dir.path());
      let task = make_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      store::write_task(dir.path(), &task).unwrap();
      store::resolve_task(dir.path(), &task.id).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        json: false,
      };

      cmd.call(&config, &Theme::default()).unwrap();
    }

    #[test]
    fn it_shows_task_as_json() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_config(dir.path());
      let task = make_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      store::write_task(dir.path(), &task).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        json: true,
      };

      cmd.call(&config, &Theme::default()).unwrap();
    }

    #[test]
    fn it_shows_task_detail() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_config(dir.path());
      let task = make_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      store::write_task(dir.path(), &task).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        json: false,
      };

      cmd.call(&config, &Theme::default()).unwrap();
    }
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

  fn make_task(id: &str) -> Task {
    let now = Utc::now();
    Task {
      resolved_at: None,
      created_at: now,
      description: "A test description".to_string(),
      id: id.parse().unwrap(),
      links: vec![Link {
        ref_: "https://example.com".to_string(),
        rel: crate::model::RelationshipType::RelatesTo,
      }],
      metadata: toml::Table::new(),
      status: Status::Open,
      tags: vec!["rust".to_string()],
      title: "Test Task".to_string(),
      updated_at: now,
    }
  }
}
