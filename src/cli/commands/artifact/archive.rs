use clap::Args;

use crate::{
  cli::{self, AppContext},
  store,
  ui::composites::success_message::SuccessMessage,
};

/// Move an artifact to the archive by setting its `archived_at` timestamp.
#[derive(Debug, Args)]
pub struct Command {
  /// Artifact ID or unique prefix.
  pub id: String,
  /// Output as JSON.
  #[arg(short, long, conflicts_with = "quiet")]
  pub json: bool,
  /// Print only the artifact ID.
  #[arg(short, long, conflicts_with = "json")]
  pub quiet: bool,
}

impl Command {
  /// Archive the artifact matching `self.id` and print a confirmation.
  pub fn call(&self, ctx: &AppContext) -> cli::Result<()> {
    let config = &ctx.settings;
    let theme = &ctx.theme;
    let id = store::resolve_artifact_id(config, &self.id, false)?;
    store::archive_artifact(config, &id)?;

    if self.json {
      let artifact = store::read_artifact(config, &id)?;
      let json = serde_json::to_string_pretty(&artifact)?;
      println!("{json}");
      return Ok(());
    }

    if self.quiet {
      println!("{id}");
      return Ok(());
    }

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
    test_helpers::{make_test_artifact, make_test_context},
  };

  mod call {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_archives_an_artifact() {
      let dir = tempfile::tempdir().unwrap();
      let ctx = make_test_context(dir.path());
      let artifact = make_test_artifact("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      store::write_artifact(&ctx.settings, &artifact).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        json: false,
        quiet: false,
      };
      cmd.call(&ctx).unwrap();

      let filter = ArtifactFilter::default();
      let artifacts = store::list_artifacts(&ctx.settings, &filter).unwrap();

      assert_eq!(artifacts.len(), 0);

      let filter = ArtifactFilter {
        all: true,
        ..Default::default()
      };
      let artifacts = store::list_artifacts(&ctx.settings, &filter).unwrap();
      assert_eq!(artifacts.len(), 1);
      assert!(artifacts[0].archived_at.is_some());
    }
  }
}
