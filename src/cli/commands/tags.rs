/// Append tags from `to_add` into `tags`, skipping duplicates.
pub fn apply_tags(tags: &mut Vec<String>, to_add: &[String]) {
  for tag in to_add {
    if !tags.contains(tag) {
      tags.push(tag.clone());
    }
  }
}

/// Remove all entries in `to_remove` from `tags`.
pub fn remove_tags(tags: &mut Vec<String>, to_remove: &[String]) {
  tags.retain(|t| !to_remove.contains(t));
}

#[cfg(test)]
mod tests {
  use super::*;

  mod apply_tags {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_new_tags() {
      let mut tags = vec!["a".to_string()];
      apply_tags(&mut tags, &["b".to_string(), "c".to_string()]);
      assert_eq!(tags, vec!["a", "b", "c"]);
    }

    #[test]
    fn it_skips_duplicates() {
      let mut tags = vec!["a".to_string(), "b".to_string()];
      apply_tags(&mut tags, &["b".to_string(), "c".to_string()]);
      assert_eq!(tags, vec!["a", "b", "c"]);
    }
  }

  mod remove_tags {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_filters_matching_tags() {
      let mut tags = vec!["a".to_string(), "b".to_string(), "c".to_string()];
      remove_tags(&mut tags, &["b".to_string()]);
      assert_eq!(tags, vec!["a", "c"]);
    }

    #[test]
    fn it_ignores_absent_tags() {
      let mut tags = vec!["a".to_string()];
      remove_tags(&mut tags, &["z".to_string()]);
      assert_eq!(tags, vec!["a"]);
    }
  }
}
