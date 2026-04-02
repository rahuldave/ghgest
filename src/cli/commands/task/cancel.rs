use clap::Args;

use crate::{
  cli::{self, AppContext},
  model::{TaskPatch, event::AuthorInfo, note::AuthorType, task::Status},
  store,
  ui::views::task::TaskUpdateView,
};

/// Cancel a task (shortcut for `task update <id> --status cancelled`).
#[derive(Debug, Args)]
pub struct Command {
  /// Task ID or unique prefix.
  pub id: String,
}

impl Command {
  /// Mark the task as cancelled and print the confirmation view.
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
      status: Some(Status::Cancelled),
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
