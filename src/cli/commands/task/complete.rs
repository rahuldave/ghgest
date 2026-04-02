use clap::Args;

use crate::{
  cli::{self, AppContext},
  model::{TaskPatch, event::AuthorInfo, note::AuthorType, task::Status},
  store,
  ui::views::task::TaskUpdateView,
};

/// Mark a task as done.
#[derive(Debug, Args)]
pub struct Command {
  /// Task ID or unique prefix.
  pub id: String,
}

impl Command {
  /// Set the task's status to done and print the confirmation view.
  pub fn call(&self, ctx: &AppContext) -> cli::Result<()> {
    let config = &ctx.settings;
    let theme = &ctx.theme;
    let id = store::resolve_task_id(config, &self.id, true)?;

    let patch = TaskPatch {
      assigned_to: None,
      description: None,
      links: None,
      metadata: None,
      phase: None,
      priority: None,
      status: Some(Status::Done),
      tags: None,
      title: None,
    };

    let author = match crate::cli::git::resolve_author() {
      Some(a) => AuthorInfo {
        author: a.name,
        author_email: a.email,
        author_type: AuthorType::Human,
      },
      None => AuthorInfo {
        author: "unknown".to_string(),
        author_email: None,
        author_type: AuthorType::Human,
      },
    };
    let task = store::update_task(config, &id, patch, Some(&author))?;
    let id_str = task.id.to_string();

    let view = TaskUpdateView {
      id: &id_str,
      fields: Vec::new(),
      status: Some(task.status.as_str()),
      theme,
    };
    println!("{view}");
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helpers::{make_test_context, make_test_task};

  mod call {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_marks_task_as_done() {
      let dir = tempfile::tempdir().unwrap();
      let ctx = make_test_context(dir.path());
      let task = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      store::write_task(&ctx.settings, &task).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
      };
      cmd.call(&ctx).unwrap();

      let updated = store::read_task(&ctx.settings, &task.id).unwrap();

      assert_eq!(updated.status, Status::Done);
    }
  }
}
