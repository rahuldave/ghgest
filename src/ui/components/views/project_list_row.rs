use std::fmt::{self, Display, Formatter};

use yansi::Paint;

use super::super::molecules::FieldList;
use crate::ui::style;

/// A project summary row for list display.
///
/// Renders the full (untruncated) project ID using the same two-tone styling
/// as the [`Id`](super::super::atoms::Id) atom: the first 2 characters use the
/// `id.prefix` style and the remainder use `id.rest`.
pub struct Component {
  id: String,
  root: String,
}

impl Component {
  pub fn new(id: String, root: String) -> Self {
    Self {
      id,
      root,
    }
  }

  fn styled_id(&self) -> String {
    let theme = style::global();
    let prefix_len = 2.min(self.id.len());
    let (prefix, rest) = self.id.split_at(prefix_len);
    format!("{}{}", prefix.paint(*theme.id_prefix()), rest.paint(*theme.id_rest()),)
  }
}

impl Display for Component {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let fields = FieldList::new()
      .styled_field("id", self.styled_id())
      .field("root", self.root.clone());
    write!(f, "{fields}")
  }
}
