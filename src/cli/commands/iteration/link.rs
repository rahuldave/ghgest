use std::path::Path;

use chrono::Utc;
use clap::Args;

use crate::{
  cli,
  model::link::{Link, RelationshipType},
  store,
  ui::{composites::success_message::SuccessMessage, theme::Theme},
};

/// Create a relationship between an iteration and another entity.
#[derive(Debug, Args)]
pub struct Command {
  /// Iteration ID or unique prefix.
  pub id: String,
  /// Relationship type (e.g. blocks, blocked-by, relates-to).
  #[arg(value_enum)]
  pub rel: RelationshipType,
  /// Target iteration or artifact ID or unique prefix.
  pub target_id: String,
  /// Target is an artifact instead of an iteration.
  #[arg(long)]
  pub artifact: bool,
}

impl Command {
  /// Write the link to the source iteration; for iteration targets, also write the reciprocal.
  pub fn call(&self, data_dir: &Path, theme: &Theme) -> cli::Result<()> {
    let id = store::resolve_iteration_id(data_dir, &self.id, false)?;

    let target_id = if self.artifact {
      store::resolve_artifact_id(data_dir, &self.target_id, true)?
    } else {
      store::resolve_iteration_id(data_dir, &self.target_id, true)?
    };

    let ref_path = if self.artifact {
      format!("artifacts/{target_id}")
    } else {
      format!("iterations/{target_id}")
    };

    let mut iteration = store::read_iteration(data_dir, &id)?;
    iteration.links.push(Link {
      ref_: ref_path,
      rel: self.rel.clone(),
    });
    iteration.updated_at = Utc::now();
    store::write_iteration(data_dir, &iteration)?;

    if !self.artifact {
      let mut target = store::read_iteration(data_dir, &target_id)?;
      target.links.push(Link {
        ref_: format!("iterations/{id}"),
        rel: self.rel.inverse(),
      });
      target.updated_at = Utc::now();
      store::write_iteration(data_dir, &target)?;
    }

    let msg = format!("Linked {} --{}--\u{003e} {}", id, self.rel, target_id);
    println!("{}", SuccessMessage::new(&msg, theme));
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helpers::{make_test_artifact, make_test_config, make_test_iteration};

  mod call {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_links_iteration_to_artifact() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();
      let source = make_test_iteration("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      let target = make_test_artifact("kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk");
      store::write_iteration(&data_dir, &source).unwrap();
      store::write_artifact(&data_dir, &target).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        rel: RelationshipType::RelatesTo,
        target_id: "kkkk".to_string(),
        artifact: true,
      };
      cmd.call(&data_dir, &Theme::default()).unwrap();

      let loaded = store::read_iteration(&data_dir, &source.id).unwrap();
      assert_eq!(loaded.links.len(), 1);
      assert_eq!(loaded.links[0].ref_, "artifacts/kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk");
    }

    #[test]
    fn it_links_iteration_to_iteration_with_reciprocal() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();
      let source = make_test_iteration("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      let target = make_test_iteration("kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk");
      store::write_iteration(&data_dir, &source).unwrap();
      store::write_iteration(&data_dir, &target).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        rel: RelationshipType::RelatesTo,
        target_id: "kkkk".to_string(),
        artifact: false,
      };
      cmd.call(&data_dir, &Theme::default()).unwrap();

      let loaded = store::read_iteration(&data_dir, &source.id).unwrap();
      assert_eq!(loaded.links.len(), 1);
      assert_eq!(loaded.links[0].rel, RelationshipType::RelatesTo);

      let loaded_target = store::read_iteration(&data_dir, &target.id).unwrap();
      assert_eq!(loaded_target.links.len(), 1);
      assert_eq!(loaded_target.links[0].rel, RelationshipType::RelatesTo);
    }
  }
}
