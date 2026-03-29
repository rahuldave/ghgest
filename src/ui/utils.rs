/// Returns the display width of a string after stripping ANSI escape sequences.
///
/// This is useful for column alignment when strings may contain ANSI color codes.
pub fn display_width(s: &str) -> usize {
  let bytes = s.as_bytes();
  let mut width = 0usize;
  let mut i = 0;

  while i < bytes.len() {
    if bytes[i] == b'\x1b' && i + 1 < bytes.len() && bytes[i + 1] == b'[' {
      // Skip the ESC [ ... m sequence
      i += 2;
      while i < bytes.len() && bytes[i] != b'm' {
        i += 1;
      }
      if i < bytes.len() {
        i += 1; // skip 'm'
      }
    } else {
      width += 1;
      i += 1;
    }
  }

  width
}

pub fn shortest_unique_prefixes(ids: &[String]) -> Vec<usize> {
  // Trie node: count of IDs passing through this node, plus children indexed by byte value.
  // Since IDs use only chars k-z (16 chars), we use a HashMap for children.
  struct TrieNode {
    children: [Option<Box<TrieNode>>; 16],
    count: usize,
  }

  impl TrieNode {
    fn new() -> Self {
      const NONE: Option<Box<TrieNode>> = None;
      Self {
        children: [NONE; 16],
        count: 0,
      }
    }
  }

  // Map chars k-z to indices 0-15
  fn char_index(c: u8) -> usize {
    (c - b'k') as usize
  }

  // Build trie
  let mut root = TrieNode::new();
  for id in ids {
    let mut node = &mut root;
    for &b in id.as_bytes() {
      let idx = char_index(b);
      node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
      node.count += 1;
    }
  }

  // For each ID, walk the trie to find shortest unique prefix
  ids
    .iter()
    .map(|id| {
      let mut node = &root;
      let mut len = 0;
      for &b in id.as_bytes() {
        let idx = char_index(b);
        node = node.children[idx].as_ref().unwrap();
        len += 1;
        if node.count == 1 {
          break;
        }
      }
      len
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  mod display_width {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_zero_for_empty_string() {
      assert_eq!(display_width(""), 0);
    }

    #[test]
    fn it_returns_length_for_plain_text() {
      assert_eq!(display_width("hello"), 5);
    }

    #[test]
    fn it_strips_single_ansi_sequence() {
      // "\x1b[31m" = red, "\x1b[0m" = reset
      assert_eq!(display_width("\x1b[31mhello\x1b[0m"), 5);
    }

    #[test]
    fn it_strips_multiple_ansi_sequences() {
      // bold red text followed by reset, then bold blue text followed by reset
      let s = "\x1b[1;31mhello\x1b[0m \x1b[1;34mworld\x1b[0m";
      assert_eq!(display_width(s), 11); // "hello world"
    }

    #[test]
    fn it_handles_ansi_only_string() {
      assert_eq!(display_width("\x1b[31m\x1b[0m"), 0);
    }

    #[test]
    fn it_handles_text_without_reset() {
      assert_eq!(display_width("\x1b[1;4;35mheading"), 7);
    }
  }

  mod shortest_unique_prefixes {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_differentiates_shared_prefix() {
      let ids = vec![
        "zyxwvutsrqponmlkzyxwvutsrqponmlk".to_string(),
        "zyxwkkkkkkkkkkkkkkkkkkkkkkkkkkkkk".to_string(),
      ];
      assert_eq!(shortest_unique_prefixes(&ids), vec![5, 5]);
    }

    #[test]
    fn it_handles_empty_list() {
      let ids: Vec<String> = vec![];
      assert!(shortest_unique_prefixes(&ids).is_empty());
    }

    #[test]
    fn it_handles_no_overlap() {
      let ids = vec![
        "zyxwvutsrqponmlkzyxwvutsrqponmlk".to_string(),
        "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk".to_string(),
      ];
      assert_eq!(shortest_unique_prefixes(&ids), vec![1, 1]);
    }

    #[test]
    fn it_handles_three_ids_with_mixed_overlap() {
      let ids = vec![
        "zyxwvutsrqponmlkzyxwvutsrqponmlk".to_string(),
        "zyxwkkkkkkkkkkkkkkkkkkkkkkkkkkkkk".to_string(),
        "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk".to_string(),
      ];
      assert_eq!(shortest_unique_prefixes(&ids), vec![5, 5, 1]);
    }

    #[test]
    fn it_returns_one_for_single_id() {
      let ids = vec!["zyxwvutsrqponmlkzyxwvutsrqponmlk".to_string()];
      assert_eq!(shortest_unique_prefixes(&ids), vec![1]);
    }
  }
}
