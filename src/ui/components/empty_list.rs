use std::io;

/// Empty-list message for task list.
///
/// Produces output: `No tasks found.`
pub struct EmptyList {
  entity: String,
}

impl EmptyList {
  pub fn new(entity: &str) -> Self {
    Self {
      entity: entity.to_string(),
    }
  }

  /// Write the empty-list message to the given writer.
  pub fn write_to(&self, w: &mut impl io::Write) -> io::Result<()> {
    writeln!(w, "No {} found.", self.entity)
  }
}

crate::ui::macros::impl_display_via_write_to!(EmptyList);

#[cfg(test)]
mod tests {
  use super::*;

  mod display {
    use super::*;

    #[test]
    fn it_delegates_to_write_to() {
      let msg = EmptyList::new("tasks");
      let display = msg.to_string();
      assert!(display.contains("No tasks found."));
    }
  }

  mod write_to {
    use super::*;

    #[test]
    fn it_writes_empty_list_message() {
      let msg = EmptyList::new("tasks");
      let mut buf = Vec::new();
      msg.write_to(&mut buf).unwrap();
      let output = String::from_utf8(buf).unwrap();
      assert!(output.contains("No tasks found."));
    }
  }
}
