use clap::Args;

use crate::{
  cli::{self, AppContext},
  store,
};

/// Remove tags from an iteration.
#[derive(Debug, Args)]
pub struct Command {
  /// Iteration ID or unique prefix.
  pub id: String,
  /// Tags to remove (space-separated).
  pub tags: Vec<String>,
}

impl Command {
  /// Remove the specified tags from the iteration's tag set.
  pub fn call(&self, ctx: &AppContext) -> cli::Result<()> {
    super::super::tags::untag_entity(
      ctx,
      &self.id,
      &self.tags,
      "iteration",
      store::resolve_iteration_id,
      store::read_iteration,
      |i| &mut i.tags,
      |i, t| i.updated_at = t,
      store::write_iteration,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helpers::{make_test_context, make_test_iteration};

  mod call {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_handles_nonexistent_tags_gracefully() {
      let dir = tempfile::tempdir().unwrap();
      let ctx = make_test_context(dir.path());
      let mut iteration = make_test_iteration("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      iteration.tags = vec!["sprint".to_string()];
      store::write_iteration(&ctx.settings, &iteration).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        tags: vec!["nonexistent".to_string()],
      };
      cmd.call(&ctx).unwrap();

      let loaded = store::read_iteration(&ctx.settings, &iteration.id).unwrap();
      assert_eq!(loaded.tags, vec!["sprint".to_string()]);
    }

    #[test]
    fn it_removes_tags() {
      let dir = tempfile::tempdir().unwrap();
      let ctx = make_test_context(dir.path());
      let mut iteration = make_test_iteration("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      iteration.tags = vec!["sprint".to_string(), "q1".to_string(), "keep".to_string()];
      store::write_iteration(&ctx.settings, &iteration).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        tags: vec!["sprint".to_string(), "q1".to_string()],
      };
      cmd.call(&ctx).unwrap();

      let loaded = store::read_iteration(&ctx.settings, &iteration.id).unwrap();
      assert_eq!(loaded.tags, vec!["keep".to_string()]);
    }
  }
}
