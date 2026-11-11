use crate::{ModuleId, Span};
use std::collections::HashMap;

/// Organizes documentation comments for various primitives
#[derive(Debug, Clone)]
pub(super) struct Doccomments {
    // TODO: Move decl doccomments here
    module_comments: HashMap<ModuleId, Vec<Span>>,
}

impl Doccomments {
    pub fn new() -> Self {
        Doccomments {
            module_comments: HashMap::new(),
        }
    }

    pub fn add_module_comments(&mut self, module_id: ModuleId, comments: Vec<Span>) {
        self.module_comments.insert(module_id, comments);
    }

    pub fn get_module_comments(&self, module_id: ModuleId) -> Option<&[Span]> {
        self.module_comments.get(&module_id).map(|v| v.as_ref())
    }

    /// Overwrite own values with the other
    pub fn merge_with(&mut self, other: Doccomments) {
        self.module_comments.extend(other.module_comments);
    }
}

impl Default for Doccomments {
    fn default() -> Self {
        Self::new()
    }
}

/// Position of the first byte after the leading spaces of a `#` comment line.
///
/// Scanning starts after the `#` (index 1) and stops at the first non-space byte, at
/// the end of the line, or at `limit`, whichever comes first.
fn skip_comment_indent(contents: &[u8], limit: usize) -> usize {
    let mut pos = 1;
    while pos < contents.len() && pos < limit && contents[pos] == b' ' {
        pos += 1;
    }
    pos
}

/// Builds a description from `#` comment lines, returning `(brief, extra)`.
///
/// The first line's indentation is removed from every line (later lines lose at most
/// that much), and the text is split into brief and extra parts at the first blank line.
pub(super) fn build_desc(comment_lines: &[&[u8]]) -> (String, String) {
    let mut description = String::new();

    // The indentation of the first line, which later lines are dedented by.
    let mut num_spaces = None;

    // Use the comments to build the item/command description
    for contents in comment_lines {
        let pos = match num_spaces {
            None => {
                let pos = skip_comment_indent(contents, usize::MAX);
                num_spaces = Some(pos);
                pos
            }
            Some(limit) => skip_comment_indent(contents, limit),
        };
        let comment_line = String::from_utf8_lossy(&contents[pos..]).to_string();

        if !description.is_empty() {
            description.push('\n');
        }
        description.push_str(&comment_line);
    }

    if let Some((brief_desc, extra_desc)) = description.split_once("\r\n\r\n") {
        (brief_desc.to_string(), extra_desc.to_string())
    } else if let Some((brief_desc, extra_desc)) = description.split_once("\n\n") {
        (brief_desc.to_string(), extra_desc.to_string())
    } else {
        (description, String::default())
    }
}

#[cfg(test)]
mod tests {
    use super::build_desc;

    #[test]
    fn single_paragraph_is_all_brief() {
        let lines: [&[u8]; 2] = [b"# hello", b"# world"];
        assert_eq!(
            build_desc(&lines),
            ("hello\nworld".to_string(), String::new())
        );
    }

    #[test]
    fn later_lines_lose_at_most_the_first_lines_indent() {
        let lines: [&[u8]; 4] = [b"#   indented", b"#     more", b"#", b"# extra"];
        assert_eq!(
            build_desc(&lines),
            ("indented\n  more".to_string(), "extra".to_string())
        );
    }

    #[test]
    fn no_lines_gives_empty_description() {
        assert_eq!(build_desc(&[]), (String::new(), String::new()));
    }
}
