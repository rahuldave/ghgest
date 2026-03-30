use std::path::Path;

use chrono::Utc;
use clap::Args;

use crate::{
  cli, store,
  ui::{composites::success_message::SuccessMessage, theme::Theme},
};

/// Add tags to an iteration.
#[derive(Debug, Args)]
pub struct Command {
  /// Iteration ID or unique prefix.
  pub id: String,
  /// Tags to add (space-separated).
  pub tags: Vec<String>,
}

impl Command {
  /// Merge the provided tags into the iteration's existing tag set, deduplicating.
  pub fn call(&self, data_dir: &Path, theme: &Theme) -> cli::Result<()> {
    let id = store::resolve_iteration_id(data_dir, &self.id, false)?;
    let mut iteration = store::read_iteration(data_dir, &id)?;

    super::super::tags::apply_tags(&mut iteration.tags, &self.tags);

    iteration.updated_at = Utc::now();
    store::write_iteration(data_dir, &iteration)?;

    let tag_list: Vec<&str> = self.tags.iter().map(|s| s.as_str()).collect();
    let msg = format!("Tagged iteration {} with {}", id, tag_list.join(", "));
    println!("{}", SuccessMessage::new(&msg, theme));
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helpers::{make_test_config, make_test_iteration};

  mod call {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_tags() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();
      let iteration = make_test_iteration("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      store::write_iteration(&data_dir, &iteration).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        tags: vec!["sprint".to_string(), "q1".to_string()],
      };
      cmd.call(&data_dir, &Theme::default()).unwrap();

      let loaded = store::read_iteration(&data_dir, &iteration.id).unwrap();
      assert_eq!(loaded.tags, vec!["sprint".to_string(), "q1".to_string()]);
    }

    #[test]
    fn it_deduplicates_tags() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();
      let mut iteration = make_test_iteration("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      iteration.tags = vec!["sprint".to_string()];
      store::write_iteration(&data_dir, &iteration).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        tags: vec!["sprint".to_string(), "q1".to_string()],
      };
      cmd.call(&data_dir, &Theme::default()).unwrap();

      let loaded = store::read_iteration(&data_dir, &iteration.id).unwrap();
      assert_eq!(loaded.tags, vec!["sprint".to_string(), "q1".to_string()]);
    }
  }
}
