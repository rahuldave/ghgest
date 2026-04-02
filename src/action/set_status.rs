//! Generic status mutation action.

use crate::{
  action::HasStatus,
  config::Settings,
  model::{Id, event::AuthorInfo},
  store,
};

/// Change the status of any entity that supports status transitions.
///
/// Delegates to the entity's [`HasStatus::update_status`] implementation which calls the
/// appropriate store update function for lifecycle management (file movement, timestamps,
/// event recording).
pub fn set_status<T: HasStatus>(
  config: &Settings,
  id: &Id,
  status: T::Status,
  author: Option<&AuthorInfo>,
) -> store::Result<T> {
  T::update_status(config, id, status, author)
}
