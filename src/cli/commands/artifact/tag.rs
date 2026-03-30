use std::path::Path;

use chrono::Utc;
use clap::Args;

use crate::{
  cli, store,
  ui::{composites::success_message::SuccessMessage, theme::Theme},
};

/// Add tags to an artifact.
#[derive(Debug, Args)]
pub struct Command {
  /// Artifact ID or unique prefix.
  pub id: String,
  /// Tags to add (space-separated).
  pub tags: Vec<String>,
}

impl Command {
  /// Merge the given tags into the artifact's tag list, deduplicate, and persist.
  pub fn call(&self, data_dir: &Path, theme: &Theme) -> cli::Result<()> {
    let id = store::resolve_artifact_id(data_dir, &self.id, false)?;
    let mut artifact = store::read_artifact(data_dir, &id)?;

    super::super::tags::apply_tags(&mut artifact.tags, &self.tags);

    artifact.updated_at = Utc::now();
    store::write_artifact(data_dir, &artifact)?;

    let tag_list: Vec<&str> = self.tags.iter().map(|s| s.as_str()).collect();
    let msg = format!("Tagged artifact {} with {}", id, tag_list.join(", "));
    println!("{}", SuccessMessage::new(&msg, theme));
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helpers::{make_test_artifact, make_test_config};

  mod call {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_tags() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();
      let artifact = make_test_artifact("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      store::write_artifact(&data_dir, &artifact).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        tags: vec!["spec".to_string(), "backend".to_string()],
      };
      cmd.call(&data_dir, &Theme::default()).unwrap();

      let loaded = store::read_artifact(&data_dir, &artifact.id).unwrap();
      assert_eq!(loaded.tags, vec!["spec".to_string(), "backend".to_string()]);
    }

    #[test]
    fn it_deduplicates_tags() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();
      let mut artifact = make_test_artifact("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      artifact.tags = vec!["spec".to_string()];
      store::write_artifact(&data_dir, &artifact).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        tags: vec!["spec".to_string(), "backend".to_string()],
      };
      cmd.call(&data_dir, &Theme::default()).unwrap();

      let loaded = store::read_artifact(&data_dir, &artifact.id).unwrap();
      assert_eq!(loaded.tags, vec!["spec".to_string(), "backend".to_string()]);
    }
  }
}
