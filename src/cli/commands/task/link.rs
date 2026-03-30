use std::path::Path;

use chrono::Utc;
use clap::Args;

use crate::{
  cli,
  model::link::{Link, RelationshipType},
  store,
  ui::{composites::success_message::SuccessMessage, theme::Theme},
};

/// Create a relationship between a task and another task or artifact.
#[derive(Debug, Args)]
pub struct Command {
  /// Source task ID or unique prefix.
  pub id: String,
  /// Relationship type (e.g. blocks, blocked-by, relates-to).
  #[arg(value_enum)]
  pub rel: RelationshipType,
  /// Target task or artifact ID or unique prefix.
  pub target_id: String,
  /// Target is an artifact instead of a task (no reciprocal link is created).
  #[arg(long)]
  pub artifact: bool,
}

impl Command {
  /// Add a link on the source task, and a reciprocal link on the target when both are tasks.
  pub fn call(&self, data_dir: &Path, theme: &Theme) -> cli::Result<()> {
    let id = store::resolve_task_id(data_dir, &self.id, false)?;

    let target_id = if self.artifact {
      store::resolve_artifact_id(data_dir, &self.target_id, true)?
    } else {
      store::resolve_task_id(data_dir, &self.target_id, true)?
    };

    let ref_path = if self.artifact {
      format!("artifacts/{target_id}")
    } else {
      format!("tasks/{target_id}")
    };

    let mut task = store::read_task(data_dir, &id)?;
    task.links.push(Link {
      ref_: ref_path,
      rel: self.rel.clone(),
    });
    task.updated_at = Utc::now();
    store::write_task(data_dir, &task)?;

    if !self.artifact {
      let mut target_task = store::read_task(data_dir, &target_id)?;
      target_task.links.push(Link {
        ref_: format!("tasks/{id}"),
        rel: self.rel.inverse(),
      });
      target_task.updated_at = Utc::now();
      store::write_task(data_dir, &target_task)?;
    }

    let msg = format!("Linked {} --{}--\u{003e} {}", id, self.rel, target_id);
    println!("{}", SuccessMessage::new(&msg, theme));
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use tempfile::TempDir;

  use super::*;
  use crate::test_helpers::{make_test_artifact, make_test_config, make_test_task};

  mod call {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_appends_multiple_links() {
      let (dir, data_dir) = setup();
      let source = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      let target1 = make_test_task("kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk");
      let target2 = make_test_task("nnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnn");
      store::write_task(&data_dir, &source).unwrap();
      store::write_task(&data_dir, &target1).unwrap();
      store::write_task(&data_dir, &target2).unwrap();

      let cmd1 = Command {
        id: "zyxw".to_string(),
        rel: RelationshipType::Blocks,
        target_id: "kkkk".to_string(),
        artifact: false,
      };
      cmd1.call(&data_dir, &Theme::default()).unwrap();

      let cmd2 = Command {
        id: "zyxw".to_string(),
        rel: RelationshipType::RelatesTo,
        target_id: "nnnn".to_string(),
        artifact: false,
      };
      cmd2.call(&data_dir, &Theme::default()).unwrap();

      let loaded = store::read_task(&data_dir, &source.id).unwrap();
      assert_eq!(loaded.links.len(), 2);
      let _ = dir;
    }

    #[test]
    fn it_links_task_to_artifact() {
      let (dir, data_dir) = setup();
      let source = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      let target = make_test_artifact("kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk");
      store::write_task(&data_dir, &source).unwrap();
      store::write_artifact(&data_dir, &target).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        rel: RelationshipType::RelatesTo,
        target_id: "kkkk".to_string(),
        artifact: true,
      };
      cmd.call(&data_dir, &Theme::default()).unwrap();

      let loaded = store::read_task(&data_dir, &source.id).unwrap();
      assert_eq!(loaded.links.len(), 1);
      assert_eq!(loaded.links[0].ref_, "artifacts/kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk");
      let _ = dir;
    }

    #[test]
    fn it_links_task_to_task_with_reciprocal() {
      let (dir, data_dir) = setup();
      let source = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      let target = make_test_task("kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk");
      store::write_task(&data_dir, &source).unwrap();
      store::write_task(&data_dir, &target).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        rel: RelationshipType::Blocks,
        target_id: "kkkk".to_string(),
        artifact: false,
      };
      cmd.call(&data_dir, &Theme::default()).unwrap();

      let loaded = store::read_task(&data_dir, &source.id).unwrap();
      assert_eq!(loaded.links[0].rel, RelationshipType::Blocks);

      let loaded_target = store::read_task(&data_dir, &target.id).unwrap();
      assert_eq!(loaded_target.links[0].rel, RelationshipType::BlockedBy);
      let _ = dir;
    }
  }

  fn setup() -> (TempDir, std::path::PathBuf) {
    let dir = TempDir::new().unwrap();
    let config = make_test_config(dir.path().to_path_buf());
    let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();
    (dir, data_dir)
  }
}
