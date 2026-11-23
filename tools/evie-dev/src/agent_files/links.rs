//! Finds the relative links in a Markdown file.
//!
//! A small line-based reader, not a full CommonMark parser: it knows fenced
//! code blocks, code spans, inline links and images, and reference
//! definitions, which is what the agent files use.

/// A relative link: the 1-based line and the target path, without its
/// `#anchor` or `?query`, and with `%20` decoded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub line: usize,
    pub target: String,
}

/// Every relative link outside code blocks and code spans. URLs, other
/// schemes and in-page anchors are skipped.
pub fn relative_links(content: &str) -> Vec<Link> {
    let mut links = Vec::new();
    let mut open_fence: Option<&str> = None;
    for (index, line) in content.split('\n').enumerate() {
        let number = index + 1;
        let mut emit = |raw: &str| {
            if let Some(target) = target(raw) {
                links.push(Link {
                    line: number,
                    target,
                });
            }
        };

        let indented = strip_indent(line);
        if let Some(fence) = ["```", "~~~"]
            .into_iter()
            .find(|fence| indented.is_some_and(|rest| rest.starts_with(fence)))
        {
            match open_fence {
                None => {
                    open_fence = Some(fence);
                    continue;
                }
                Some(open) if open == fence => {
                    open_fence = None;
                    continue;
                }
                Some(_) => {}
            }
        }
        if open_fence.is_some() {
            continue;
        }

        if let Some(definition) = indented.and_then(reference_definition) {
            emit(definition);
            continue;
        }

        let stripped = strip_code_spans(line);
        let mut rest = stripped.as_str();
        while let Some(start) = rest.find("](") {
            let after = &rest[start + 2..];
            let Some(end) = after.find(')') else {
                break;
            };
            emit(&after[..end]);
            rest = &after[end + 1..];
        }
    }
    links
}

/// The line without up to three leading spaces, or `None` with four or more.
fn strip_indent(line: &str) -> Option<&str> {
    let spaces = line.len() - line.trim_start_matches(' ').len();
    (spaces <= 3).then(|| &line[spaces..])
}

/// For `[label]: target`, the text after the colon.
fn reference_definition(line: &str) -> Option<&str> {
    let label = line.strip_prefix('[')?;
    let end = label.find(']')?;
    if end == 0 {
        return None;
    }
    label[end + 1..].strip_prefix(':')
}

/// Removes code spans: a run of backticks up to the next run of the same
/// length. An unclosed run and the rest of the line stay.
fn strip_code_spans(line: &str) -> String {
    let mut out = String::new();
    let mut rest = line;
    while let Some(start) = rest.find('`') {
        out.push_str(&rest[..start]);
        let ticks = rest[start..].len() - rest[start..].trim_start_matches('`').len();
        let run = &rest[start..start + ticks];
        let after = &rest[start + ticks..];
        match after.find(run) {
            Some(close) => rest = &after[close + ticks..],
            None => {
                out.push_str(&rest[start..]);
                return out;
            }
        }
    }
    out.push_str(rest);
    out
}

/// The path of a link destination, or `None` when it isn't a relative path.
fn target(raw: &str) -> Option<String> {
    let raw = raw.trim_start_matches([' ', '\t']);
    let path = match raw.strip_prefix('<') {
        Some(bracketed) => bracketed.split('>').next().unwrap_or_default(),
        None => raw.split([' ', '\t']).next().unwrap_or_default(),
    };
    let path = path.split(['#', '?']).next().unwrap_or_default();
    let path = path.replace("%20", " ");
    (!path.is_empty() && !has_scheme(&path)).then_some(path)
}

/// `https:`, `mailto:` and the like.
fn has_scheme(path: &str) -> bool {
    let Some((scheme, _)) = path.split_once(':') else {
        return false;
    };
    let mut chars = scheme.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '.' | '-'))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn targets(content: &str) -> Vec<(usize, String)> {
        relative_links(content)
            .into_iter()
            .map(|link| (link.line, link.target))
            .collect()
    }

    #[test]
    fn finds_inline_links_images_and_reference_definitions() {
        let content = "See [a](a.md) and ![b](img/b.png \"title\").\n[ref]: ref.md\n   [ref2]: <with space.md>\n";

        assert_eq!(
            targets(content),
            [
                (1, "a.md".to_owned()),
                (1, "img/b.png".to_owned()),
                (2, "ref.md".to_owned()),
                (3, "with space.md".to_owned()),
            ]
        );
    }

    #[test]
    fn skips_urls_anchors_and_schemes() {
        let content = "[a](https://x.y) [b](#top) [c](mailto:x@y.z) [d]()\n";

        assert!(targets(content).is_empty());
    }

    #[test]
    fn strips_anchors_and_queries_and_decodes_spaces() {
        assert_eq!(
            targets("[a](my%20file.md#part) [b](c.md?x=1)\n"),
            [(1, "my file.md".to_owned()), (1, "c.md".to_owned())]
        );
    }

    #[test]
    fn skips_fenced_code_blocks_and_code_spans() {
        let content = "```md\n[a](in-fence.md)\n~~~\n[b](still-in-fence.md)\n```\n`[c](span.md)` ``[d](`x`.md)`` [e](real.md)\n    ```\n[f](after-indented-fence.md)\n";

        assert_eq!(
            targets(content),
            [
                (6, "real.md".to_owned()),
                (8, "after-indented-fence.md".to_owned())
            ]
        );
    }

    #[test]
    fn keeps_the_rest_of_the_line_after_an_unclosed_code_span() {
        assert_eq!(targets("`open [a](a.md)\n"), [(1, "a.md".to_owned())]);
    }
}
