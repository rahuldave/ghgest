use clap::Args;

use crate::{
  config,
  config::Config,
  store,
  ui::{components::Confirmation, theme::Theme},
};

/// Move an artifact to the archive
#[derive(Debug, Args)]
pub struct Command {
  /// Artifact ID or unique prefix
  pub id: String,
}

impl Command {
  pub fn call(&self, config: &Config, theme: &Theme) -> crate::Result<()> {
    log::info!("archiving artifact with prefix '{}'", self.id);
    let data_dir = config::data_dir(config)?;
    log::debug!("resolving artifact ID from prefix '{}'", self.id);
    let id = store::resolve_artifact_id(&data_dir, &self.id, false)?;
    log::debug!("resolved artifact ID: {id}");
    store::archive_artifact(&data_dir, &id)?;
    log::trace!("artifact {id} archived successfully");
    Confirmation::new("Archived", "artifact", &id).write_to(&mut std::io::stdout(), theme)?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    model::ArtifactFilter,
    store,
    test_helpers::{make_test_artifact, make_test_config},
  };

  mod call {
    use super::*;

    #[test]
    fn it_archives_an_artifact() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path());
      let artifact = make_test_artifact("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      store::write_artifact(dir.path(), &artifact).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
      };
      cmd.call(&config, &Theme::default()).unwrap();

      // Active list should be empty
      let filter = ArtifactFilter::default();
      let active = store::list_artifacts(dir.path(), &filter).unwrap();
      assert!(active.is_empty());
    }

    #[test]
    fn it_sets_archived_at_timestamp() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path());
      let artifact = make_test_artifact("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      store::write_artifact(dir.path(), &artifact).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
      };
      cmd.call(&config, &Theme::default()).unwrap();

      // Should still be readable (from archive) with archived_at set
      let loaded = store::read_artifact(dir.path(), &artifact.id).unwrap();
      assert!(loaded.archived_at.is_some());
    }

    #[test]
    fn it_appears_in_archived_list() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path());
      let artifact = make_test_artifact("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      store::write_artifact(dir.path(), &artifact).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
      };
      cmd.call(&config, &Theme::default()).unwrap();

      let filter = ArtifactFilter {
        only_archived: true,
        ..Default::default()
      };
      let archived = store::list_artifacts(dir.path(), &filter).unwrap();
      assert_eq!(archived.len(), 1);
    }
  }
}
