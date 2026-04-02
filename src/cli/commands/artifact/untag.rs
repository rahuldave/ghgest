use clap::Args;

use crate::{
  action,
  cli::{self, AppContext},
  model::Artifact,
  ui::composites::success_message::SuccessMessage,
};

/// Remove tags from an artifact.
#[derive(Debug, Args)]
pub struct Command {
  /// Artifact ID or unique prefix.
  pub id: String,
  /// Output the artifact as JSON after untagging.
  #[arg(short, long, conflicts_with = "quiet")]
  pub json: bool,
  /// Output only the artifact ID.
  #[arg(short, long, conflicts_with = "json")]
  pub quiet: bool,
  /// Tags to remove (space or comma-separated).
  #[arg(value_delimiter = ',')]
  pub tags: Vec<String>,
}

impl Command {
  /// Remove the given tags from the artifact's tag list and persist.
  pub fn call(&self, ctx: &AppContext) -> cli::Result<()> {
    let artifact = action::untag::<Artifact>(&ctx.settings, &self.id, &self.tags)?;

    if self.json {
      println!("{}", serde_json::to_string_pretty(&artifact)?);
    } else if self.quiet {
      println!("{}", artifact.id);
    } else {
      let msg = format!("Untagged artifact {} from {}", artifact.id, self.tags.join(", "));
      println!("{}", SuccessMessage::new(&msg, &ctx.theme));
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helpers::{make_test_artifact, make_test_context};

  mod call {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::store;

    #[test]
    fn it_handles_nonexistent_tags_gracefully() {
      let dir = tempfile::tempdir().unwrap();
      let ctx = make_test_context(dir.path());
      let mut artifact = make_test_artifact("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      artifact.tags = vec!["spec".to_string()];
      store::write_artifact(&ctx.settings, &artifact).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        json: false,
        quiet: false,
        tags: vec!["nonexistent".to_string()],
      };
      cmd.call(&ctx).unwrap();

      let loaded = store::read_artifact(&ctx.settings, &artifact.id).unwrap();

      assert_eq!(loaded.tags, vec!["spec".to_string()]);
    }

    #[test]
    fn it_removes_tags() {
      let dir = tempfile::tempdir().unwrap();
      let ctx = make_test_context(dir.path());
      let mut artifact = make_test_artifact("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      artifact.tags = vec!["spec".to_string(), "backend".to_string(), "keep".to_string()];
      store::write_artifact(&ctx.settings, &artifact).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        json: false,
        quiet: false,
        tags: vec!["spec".to_string(), "backend".to_string()],
      };
      cmd.call(&ctx).unwrap();

      let loaded = store::read_artifact(&ctx.settings, &artifact.id).unwrap();

      assert_eq!(loaded.tags, vec!["keep".to_string()]);
    }
  }
}
