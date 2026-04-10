use clap::Args;

use crate::{
  AppContext,
  cli::{Error, limit::LimitArgs},
  store::{
    model::{
      primitives::{EntityType, Id, TaskStatus},
      task::Filter,
    },
    repo,
  },
  ui::{
    components::{TaskEntry, TaskListView},
    envelope::Envelope,
    json,
  },
};

/// List tasks in the current project.
#[derive(Args, Debug)]
pub struct Command {
  /// Show all tasks, including resolved.
  #[arg(long, short)]
  all: bool,
  /// Filter by assigned author name.
  #[arg(long)]
  assigned_to: Option<String>,
  #[command(flatten)]
  limit: LimitArgs,
  /// Filter by status.
  #[arg(long, short)]
  status: Option<TaskStatus>,
  /// Filter by tag.
  #[arg(long, short)]
  tag: Option<String>,
  #[command(flatten)]
  output: json::Flags,
}

impl Command {
  /// Query tasks with the requested filters and render them as a table, JSON, or plain IDs.
  pub async fn call(&self, context: &AppContext) -> Result<(), Error> {
    log::debug!("task list: entry");
    let project_id = context.project_id().as_ref().ok_or(Error::UninitializedProject)?;
    let conn = context.store().connect().await?;

    let filter = Filter {
      all: self.all,
      assigned_to: self.assigned_to.clone(),
      status: self.status,
      tag: self.tag.clone(),
    };

    let mut tasks = repo::task::all(&conn, project_id, &filter).await?;
    self.limit.apply(&mut tasks);

    if self.output.quiet {
      for task in &tasks {
        println!("{}", task.id().short());
      }
      return Ok(());
    }

    if self.output.json {
      let pairs: Vec<(Id, _)> = tasks.iter().map(|t| (t.id().clone(), t.clone())).collect();
      let envelopes = Envelope::load_many(&conn, EntityType::Task, &pairs, false).await?;
      self.output.print_envelopes(
        &envelopes,
        || unreachable!("json flag is set"),
        || unreachable!("json flag is set"),
      )?;
      return Ok(());
    }

    // Pick the prefix pool to match the resolver's view of the world:
    // when the listing includes any terminal rows (`--all` or a terminal
    // status filter), highlight against the project-wide pool; otherwise
    // use the active pool so prefixes line up with `task show <prefix>`.
    let includes_terminal = self.all || self.status.map(|s| s.is_terminal()).unwrap_or(false);
    let prefix_len = if includes_terminal {
      repo::task::shortest_all_prefix(&conn, project_id).await?
    } else {
      repo::task::shortest_active_prefix(&conn, project_id).await?
    };

    let id_shorts: Vec<String> = tasks.iter().map(|t| t.id().short().to_string()).collect();
    let mut entries = Vec::new();
    for (task, id_short) in tasks.iter().zip(id_shorts.iter()) {
      let tags = repo::tag::for_entity(&conn, EntityType::Task, task.id()).await?;
      entries.push(TaskEntry {
        blocked_by: None,
        blocking: false,
        id: id_short.clone(),
        priority: task.priority(),
        status: task.status().to_string(),
        tags,
        title: task.title().to_string(),
      });
    }

    crate::io::pager::page(&format!("{}\n", TaskListView::new(entries, prefix_len)), context)?;

    Ok(())
  }
}
