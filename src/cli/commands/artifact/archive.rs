use std::path::Path;

use clap::Args;

use crate::{
  cli, store,
  ui::{composites::success_message::SuccessMessage, theme::Theme},
};

/// Move an artifact to the archive by setting its `archived_at` timestamp.
#[derive(Debug, Args)]
pub struct Command {
  /// Artifact ID or unique prefix.
  pub id: String,
}

impl Command {
  /// Archive the artifact matching `self.id` and print a confirmation.
  pub fn call(&self, data_dir: &Path, theme: &Theme) -> cli::Result<()> {
    let id = store::resolve_artifact_id(data_dir, &self.id, false)?;
    store::archive_artifact(data_dir, &id)?;

    let msg = format!("Archived artifact {id}");
    println!("{}", SuccessMessage::new(&msg, theme));
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
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_archives_an_artifact() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();
      let artifact = make_test_artifact("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      store::write_artifact(&data_dir, &artifact).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
      };
      cmd.call(&data_dir, &Theme::default()).unwrap();

      let filter = ArtifactFilter::default();
      let artifacts = store::list_artifacts(&data_dir, &filter).unwrap();
      assert_eq!(artifacts.len(), 0);

      let filter = ArtifactFilter {
        show_all: true,
        ..Default::default()
      };
      let artifacts = store::list_artifacts(&data_dir, &filter).unwrap();
      assert_eq!(artifacts.len(), 1);
      assert!(artifacts[0].archived_at.is_some());
    }
  }
}
