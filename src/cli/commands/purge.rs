use std::path::PathBuf;

use chrono::Utc;
use clap::Args;

use crate::{
  AppContext,
  cli::{Error, prompt},
  store::{
    model::primitives::{EntityType, Id},
    repo::{self, purge::Scope},
    sync::tombstone,
  },
  ui::components::SuccessMessage,
};

/// Purge terminal, archived, and orphaned data from the store.
///
/// By default operates on the current project. Pass `--all-projects` to sweep
/// the entire store. When no selector flags are given, all selectors are
/// applied (the confirmation prompt is the safety net).
#[derive(Args, Debug)]
pub struct Command {
  /// Operate across every project in the store instead of the current one.
  #[arg(long)]
  all_projects: bool,
  /// Show what would be purged without making any changes.
  #[arg(long)]
  dry_run: bool,
  /// Purge archived artifacts.
  #[arg(long)]
  artifacts: bool,
  /// Purge archived iterations (terminal: completed/cancelled).
  #[arg(long)]
  iterations: bool,
  /// Purge archived projects (non-undoable).
  #[arg(long)]
  projects: bool,
  /// Purge dangling relationship rows.
  #[arg(long)]
  relationships: bool,
  /// Purge terminal tasks (done/cancelled).
  #[arg(long)]
  tasks: bool,
  /// Purge orphan tombstone files.
  #[arg(long)]
  tombstones: bool,
  /// Skip the interactive confirmation prompt.
  #[arg(long)]
  yes: bool,
}

impl Command {
  /// Execute the purge command.
  pub async fn call(&self, context: &AppContext) -> Result<(), Error> {
    log::debug!("purge: entry");
    let conn = context.store().connect().await?;

    let scope = if self.all_projects {
      Scope::AllProjects
    } else {
      let project_id = context.project_id().as_ref().ok_or(Error::UninitializedProject)?;
      Scope::Project(project_id.clone())
    };

    // When no selector flags are given, default to all selectors.
    let all =
      !(self.tasks || self.iterations || self.artifacts || self.projects || self.relationships || self.tombstones);
    let do_tasks = all || self.tasks;
    let do_iterations = all || self.iterations;
    let do_artifacts = all || self.artifacts;
    let do_projects = all || self.projects;
    let do_relationships = all || self.relationships;
    let do_tombstones = all || self.tombstones;

    // Collect gest dirs for tombstone scanning.
    let gest_dirs = collect_gest_dirs(&conn, &scope).await?;

    // Run all selectors.
    let tasks = if do_tasks {
      repo::purge::terminal_tasks(&conn, &scope).await?
    } else {
      Default::default()
    };
    let iterations = if do_iterations {
      repo::purge::terminal_iterations(&conn, &scope).await?
    } else {
      Default::default()
    };
    let artifacts = if do_artifacts {
      repo::purge::archived_artifacts(&conn, &scope).await?
    } else {
      Default::default()
    };
    let projects = if do_projects {
      repo::purge::archived_projects(&conn, &scope).await?
    } else {
      Default::default()
    };
    let relationships = if do_relationships {
      repo::purge::dangling_relationships(&conn, &scope).await?
    } else {
      Default::default()
    };
    let tombstones = if do_tombstones {
      repo::purge::orphan_tombstones(&conn, &scope, &gest_dirs).await?
    } else {
      Default::default()
    };

    let total =
      tasks.total() + iterations.total() + artifacts.count + projects.count + relationships.count + tombstones.count;

    if total == 0 {
      println!("Nothing to purge.");
      return Ok(());
    }

    // Build summary lines.
    let mut summary_lines = Vec::new();
    if tasks.total() > 0 {
      summary_lines.push(format!(
        "  tasks: {} ({} done, {} cancelled)",
        tasks.total(),
        tasks.done,
        tasks.cancelled
      ));
    }
    if iterations.total() > 0 {
      summary_lines.push(format!(
        "  iterations: {} ({} completed, {} cancelled)",
        iterations.total(),
        iterations.completed,
        iterations.cancelled
      ));
    }
    if artifacts.count > 0 {
      summary_lines.push(format!("  artifacts: {}", artifacts.count));
    }
    if projects.count > 0 {
      summary_lines.push(format!("  projects: {} (non-undoable)", projects.count));
    }
    if relationships.count > 0 {
      summary_lines.push(format!("  relationships: {}", relationships.count));
    }
    if tombstones.count > 0 {
      summary_lines.push(format!("  tombstones: {}", tombstones.count));
    }

    let summary = summary_lines.join("\n");
    println!("Purge summary:\n{summary}");

    if self.dry_run {
      return Ok(());
    }

    if !prompt::confirm_destructive("purge", "the items listed above", self.yes)? {
      log::info!("purge: aborted by user");
      return Ok(());
    }

    // Determine the project id for the transaction. For --all-projects we use
    // the current project if available; otherwise pick the first project.
    let tx_project_id = match &scope {
      Scope::Project(pid) => pid.clone(),
      Scope::AllProjects => {
        if let Some(pid) = context.project_id().as_ref() {
          pid.clone()
        } else {
          let all = repo::project::all(&conn, true).await?;
          all
            .first()
            .ok_or_else(|| Error::Argument("no projects in store".into()))?
            .id()
            .clone()
        }
      }
    };

    let tx = repo::transaction::begin(&conn, &tx_project_id, "purge").await?;
    let deleted_at = Utc::now();

    // Delete terminal tasks.
    for task_id in &tasks.ids {
      repo::entity::delete::delete_with_cascade(&conn, tx.id(), EntityType::Task, task_id).await?;
      tombstone::tombstone_task(context.gest_dir().as_deref(), task_id, deleted_at)?;
    }

    // Delete terminal iterations.
    for iteration_id in &iterations.ids {
      repo::entity::delete::delete_with_cascade(&conn, tx.id(), EntityType::Iteration, iteration_id).await?;
      tombstone::tombstone_iteration(context.gest_dir().as_deref(), iteration_id, deleted_at)?;
    }

    // Delete archived artifacts.
    for artifact_id in &artifacts.ids {
      repo::entity::delete::delete_with_cascade(&conn, tx.id(), EntityType::Artifact, artifact_id).await?;
      tombstone::tombstone_artifact(context.gest_dir().as_deref(), artifact_id, deleted_at)?;
    }

    // Delete archived projects (non-undoable).
    for project_id in &projects.ids {
      let gest_dir = gest_dirs
        .iter()
        .find(|(id, _)| id == project_id)
        .map(|(_, path)| path.as_path());
      repo::project::delete(&conn, project_id, gest_dir, deleted_at).await?;
    }

    // Delete dangling relationships.
    for rel_id in &relationships.ids {
      repo::relationship::delete(&conn, rel_id).await?;
    }

    // Delete orphan tombstone files.
    for path in &tombstones.paths {
      if path.exists() {
        std::fs::remove_file(path)?;
      }
    }

    let message = SuccessMessage::new("Purged").field("total", total.to_string());
    println!("{message}");
    println!("Run `gest undo` to restore.");

    Ok(())
  }
}

/// Collect `(project_id, gest_dir_path)` pairs for tombstone scanning.
async fn collect_gest_dirs(conn: &libsql::Connection, scope: &Scope) -> Result<Vec<(Id, PathBuf)>, Error> {
  let projects = match scope {
    Scope::AllProjects => repo::project::all(conn, true).await?,
    Scope::Project(pid) => match repo::project::find_by_id(conn, pid.clone()).await? {
      Some(p) => vec![p],
      None => vec![],
    },
  };

  let mut dirs = Vec::new();
  for project in projects {
    let root = project.root();
    let gest_dir = root.join(".gest");
    if gest_dir.is_dir() {
      dirs.push((project.id().clone(), gest_dir));
    }
  }
  Ok(dirs)
}
