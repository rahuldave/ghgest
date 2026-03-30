use std::{io::IsTerminal, path::Path};

use clap::Args;

use crate::{
  cli,
  model::NewArtifact,
  store,
  ui::{theme::Theme, views::artifact::ArtifactCreateView},
};

/// Create a new artifact from inline text, a source file, an editor, or stdin.
#[derive(Debug, Args)]
pub struct Command {
  /// Artifact title.
  pub title: String,
  /// Body content as an inline string (skips editor and stdin).
  #[arg(short, long)]
  pub body: Option<String>,
  /// Artifact type (e.g. spec, adr, rfc, note).
  #[arg(short = 'k', long = "type")]
  pub kind: Option<String>,
  /// Key=value metadata pairs (repeatable).
  #[arg(short, long)]
  pub metadata: Vec<String>,
  /// Read body content from a file path.
  #[arg(short, long)]
  pub source: Option<String>,
  /// Comma-separated list of tags.
  #[arg(long)]
  pub tags: Option<String>,
}

impl Command {
  /// Build a `NewArtifact`, persist it, and print the creation summary.
  pub fn call(&self, data_dir: &Path, theme: &Theme) -> cli::Result<()> {
    let metadata = {
      let pairs = crate::cli::helpers::split_key_value_pairs(&self.metadata)?;
      let mut mapping = yaml_serde::Mapping::new();
      for (key, value) in pairs {
        mapping.insert(yaml_serde::Value::String(key), yaml_serde::Value::String(value));
      }
      mapping
    };

    let tags = self
      .tags
      .as_deref()
      .map(crate::cli::helpers::parse_tags)
      .unwrap_or_default();

    let body = self.read_body()?;

    let new = NewArtifact {
      body,
      kind: self.kind.clone(),
      metadata,
      tags,
      title: self.title.clone(),
    };

    let artifact = store::create_artifact(data_dir, new)?;

    let id_str = artifact.id.to_string();
    let mut view = ArtifactCreateView::new(&id_str, &artifact.title, theme);
    if let Some(ref src) = self.source {
      view = view.source(src);
    }
    println!("{view}");
    Ok(())
  }

  /// Resolve body content from `--source`, `--body`, `$EDITOR`, or empty fallback.
  fn read_body(&self) -> cli::Result<String> {
    if let Some(ref src) = self.source {
      return std::fs::read_to_string(src).map_err(cli::Error::from);
    }

    if let Some(ref body) = self.body {
      return Ok(body.clone());
    }

    if std::io::stdin().is_terminal()
      && let Some(_editor) = crate::cli::editor::resolve_editor()
    {
      let content = crate::cli::editor::edit_temp(None, ".md")?;
      if content.trim().is_empty() {
        return Err(cli::Error::generic("Aborting: empty body"));
      }
      return Ok(content);
    }

    Ok(String::new())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod call {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::test_helpers::make_test_config;

    #[test]
    fn it_creates_an_artifact_from_source_file() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();

      let source_path = dir.path().join("source.md");
      std::fs::write(&source_path, "# From File\n\nFile content.").unwrap();

      let cmd = Command {
        title: "Sourced Artifact".to_string(),
        body: None,
        kind: None,
        metadata: vec![],
        source: Some(source_path.to_string_lossy().to_string()),
        tags: None,
      };

      cmd.call(&data_dir, &Theme::default()).unwrap();

      let filter = crate::model::ArtifactFilter::default();
      let artifacts = store::list_artifacts(&data_dir, &filter).unwrap();
      assert_eq!(artifacts.len(), 1);
      assert_eq!(artifacts[0].body, "# From File\n\nFile content.");
    }

    #[test]
    fn it_creates_an_artifact_with_all_flags() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();

      let cmd = Command {
        title: "Full Artifact".to_string(),
        body: Some("# Content\n\nSome body text.".to_string()),
        kind: Some("spec".to_string()),
        metadata: vec!["version=1".to_string()],
        source: None,
        tags: Some("rust,cli".to_string()),
      };

      cmd.call(&data_dir, &Theme::default()).unwrap();

      let filter = crate::model::ArtifactFilter::default();
      let artifacts = store::list_artifacts(&data_dir, &filter).unwrap();
      assert_eq!(artifacts.len(), 1);

      let artifact = &artifacts[0];
      assert_eq!(artifact.title, "Full Artifact");
      assert_eq!(artifact.body, "# Content\n\nSome body text.");
      assert_eq!(artifact.kind.as_deref(), Some("spec"));
      assert_eq!(artifact.tags, vec!["rust", "cli"]);
    }

    #[test]
    fn it_creates_an_artifact_with_defaults() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();

      let cmd = Command {
        title: "My Artifact".to_string(),
        body: None,
        kind: None,
        metadata: vec![],
        source: None,
        tags: None,
      };

      cmd.call(&data_dir, &Theme::default()).unwrap();

      let filter = crate::model::ArtifactFilter::default();
      let artifacts = store::list_artifacts(&data_dir, &filter).unwrap();
      assert_eq!(artifacts.len(), 1);
      assert_eq!(artifacts[0].title, "My Artifact");
    }
  }
}
