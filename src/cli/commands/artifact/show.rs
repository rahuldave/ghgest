use std::path::Path;

use clap::Args;

use crate::{
  cli, store,
  ui::{composites::artifact_detail::ArtifactDetail, theme::Theme, views::artifact::ArtifactDetailView},
};

/// Display an artifact's full details and rendered body.
#[derive(Debug, Args)]
pub struct Command {
  /// Artifact ID or unique prefix.
  pub id: String,
  /// Output as JSON instead of formatted detail.
  #[arg(short, long)]
  pub json: bool,
}

impl Command {
  /// Resolve the artifact and print its detail view or JSON representation.
  pub fn call(&self, data_dir: &Path, theme: &Theme) -> cli::Result<()> {
    let id = store::resolve_artifact_id(data_dir, &self.id, true)?;
    let artifact = store::read_artifact(data_dir, &id)?;

    if self.json {
      let json = serde_json::to_string_pretty(&artifact).map_err(|e| cli::Error::generic(e.to_string()))?;
      println!("{json}");
      return Ok(());
    }

    let id_str = artifact.id.to_string();
    let created = artifact.created_at.format("%Y-%m-%d").to_string();
    let updated = artifact.updated_at.format("%Y-%m-%d").to_string();

    let body = if artifact.body.is_empty() {
      None
    } else {
      Some(artifact.body.as_str())
    };

    let detail = ArtifactDetail::new(&id_str, &artifact.title, &artifact.tags, &created, &updated, theme).body(body);
    let view = ArtifactDetailView::new(detail);
    println!("{view}");

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    store,
    test_helpers::{make_test_artifact, make_test_config},
  };

  mod call {
    use super::*;

    #[test]
    fn it_shows_archived_artifact() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();
      let artifact = make_test_artifact("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      store::write_artifact(&data_dir, &artifact).unwrap();
      store::archive_artifact(&data_dir, &artifact.id).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        json: false,
      };

      cmd.call(&data_dir, &Theme::default()).unwrap();
    }

    #[test]
    fn it_shows_artifact_as_json() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();
      let artifact = make_test_artifact("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      store::write_artifact(&data_dir, &artifact).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        json: true,
      };

      cmd.call(&data_dir, &Theme::default()).unwrap();
    }

    #[test]
    fn it_shows_artifact_detail() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();
      let mut artifact = make_test_artifact("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      artifact.title = "Test Artifact".to_string();
      artifact.body = "# Hello\n\nSome content.".to_string();
      artifact.tags = vec!["spec".to_string()];
      store::write_artifact(&data_dir, &artifact).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        json: false,
      };

      cmd.call(&data_dir, &Theme::default()).unwrap();
    }
  }
}
