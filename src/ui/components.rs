//! Reusable UI components organized by granularity (atoms, molecules, views).

mod atoms;
mod molecules;
mod views;

pub use atoms::{min_unique_prefix, prefix_lengths_two_tier, unique_prefix_lengths};
pub use molecules::{Banner, ConfirmPrompt, ErrorMessage, FieldList, SuccessMessage, UpdateNotice};
pub use views::{
  ArtifactDetail, ArtifactEntry, ArtifactListView, GraphTask, IterationDetail, IterationEntry, IterationGraphView,
  IterationListView, MetaGet, ProjectEntry, ProjectListView, ProjectShow, SearchResults, TaskCounts, TaskDetail,
  TaskEntry, TaskListView,
};
