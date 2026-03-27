use clap::Args;

use crate::{
  cli::commands::json_utils::artifact_to_json,
  config,
  config::Config,
  store,
  ui::{components::ArtifactDetail, theme::Theme},
};

/// Display an artifact's full details and body
#[derive(Debug, Args)]
pub struct Command {
  /// Artifact ID or unique prefix
  pub id: String,
  /// Output artifact details as JSON
  #[arg(short, long)]
  pub json: bool,
}

impl Command {
  pub fn call(&self, config: &Config, theme: &Theme) -> crate::Result<()> {
    log::info!("showing artifact with prefix '{}'", self.id);
    let data_dir = config::data_dir(config)?;
    log::debug!("resolving artifact ID from prefix '{}'", self.id);
    let id = store::resolve_artifact_id(&data_dir, &self.id, true)?;
    log::debug!("resolved artifact ID: {id}");
    let artifact = store::read_artifact(&data_dir, &id)?;
    log::trace!(
      "artifact '{}' loaded, outputting as {}",
      artifact.title,
      if self.json { "json" } else { "detail" }
    );

    if self.json {
      let json = artifact_to_json(&artifact);
      println!("{}", serde_json::to_string_pretty(&json)?);
    } else {
      ArtifactDetail::new(&artifact).write_to(&mut std::io::stdout(), theme)?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    model::Artifact,
    store,
    test_helpers::{make_test_artifact, make_test_config},
  };

  mod call {
    use super::*;

    #[test]
    fn it_shows_artifact_detail() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path());
      let artifact = Artifact {
        title: "Detail Test".to_string(),
        body: "Some body".to_string(),
        ..make_test_artifact("zyxwvutsrqponmlkzyxwvutsrqponmlk")
      };
      store::write_artifact(dir.path(), &artifact).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        json: false,
      };
      // Should not error
      cmd.call(&config, &Theme::default()).unwrap();
    }

    #[test]
    fn it_shows_artifact_as_json() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path());
      let artifact = Artifact {
        title: "JSON Test".to_string(),
        body: "json body".to_string(),
        ..make_test_artifact("zyxwvutsrqponmlkzyxwvutsrqponmlk")
      };
      store::write_artifact(dir.path(), &artifact).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        json: true,
      };
      // Should not error
      cmd.call(&config, &Theme::default()).unwrap();
    }
  }
}
