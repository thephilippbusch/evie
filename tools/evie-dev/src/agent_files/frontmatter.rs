//! Reads the YAML frontmatter of skills and subagents.
//!
//! Two readers, on purpose. [`field`] reads one top-level key line by line,
//! for subagents and evie's ASCII name rule. [`parse_skill`] parses the YAML
//! like `skills-ref` does with strictyaml: every scalar is a string, and flow
//! style, anchors, aliases, tags and duplicate keys are errors.

use yaml_rust2::Event;
use yaml_rust2::parser::Parser;
use yaml_rust2::scanner::{Scanner, TokenType};

/// The frontmatter starts with a `---` line and has a closing `---` line.
pub fn has_frontmatter(content: &str) -> bool {
    let mut lines = content.split('\n');
    lines.next() == Some("---") && lines.any(|line| line == "---")
}

/// The value of a top-level key in the frontmatter, unquoted, with the
/// lines of a block scalar (`|` or `>`) joined by spaces. Empty when the key
/// or the frontmatter is missing.
pub fn field(content: &str, key: &str) -> String {
    let prefix = format!("{key}:");
    let mut value: Option<String> = None;
    for (index, line) in content.split('\n').enumerate() {
        if index == 0 {
            if line == "---" {
                continue;
            }
            break;
        }
        if line == "---" {
            break;
        }
        if let Some(value) = value.as_mut() {
            if line.starts_with([' ', '\t']) {
                value.push(' ');
                value.push_str(line.trim_start_matches([' ', '\t']));
                continue;
            }
            break;
        }
        if let Some(rest) = line.strip_prefix(&prefix) {
            let rest = rest.trim_start_matches([' ', '\t']);
            value = Some(if is_block_indicator(rest) {
                String::new()
            } else {
                rest.to_owned()
            });
        }
    }
    let value = value.unwrap_or_default();
    let value = value.trim_matches([' ', '\t']);
    let unquoted = ['"', '\'']
        .iter()
        .find_map(|quote| value.strip_prefix(*quote)?.strip_suffix(*quote));
    unquoted.unwrap_or(value).to_owned()
}

/// `|`, `>`, `|-`, `>+2` and the like, with optional trailing blanks.
fn is_block_indicator(value: &str) -> bool {
    let mut chars = value.chars();
    matches!(chars.next(), Some('|' | '>'))
        && chars
            .as_str()
            .trim_start_matches(|c: char| c == '-' || c == '+' || c.is_ascii_digit())
            .chars()
            .all(|c| c == ' ' || c == '\t')
}

/// A YAML value as strictyaml sees it: scalars stay strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Scalar(String),
    Sequence(Vec<Node>),
    Mapping(Vec<(String, Node)>),
}

/// Why a `SKILL.md` frontmatter can't be read, with the `skills-ref` message.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum FrontmatterError {
    #[error("SKILL.md must start with YAML frontmatter (---)")]
    Missing,

    #[error("SKILL.md frontmatter not properly closed with ---")]
    NotClosed,

    #[error("Invalid YAML in frontmatter: {0}")]
    InvalidYaml(String),

    #[error("SKILL.md frontmatter must be a YAML mapping")]
    NotAMapping,
}

/// Parses the frontmatter of a `SKILL.md` into its top-level keys.
pub fn parse_skill(content: &str) -> Result<Vec<(String, Node)>, FrontmatterError> {
    // skills-ref splits at the first two `---` anywhere in the file, not at
    // lines, so a `---` inside a value ends the frontmatter too.
    let rest = content
        .strip_prefix("---")
        .ok_or(FrontmatterError::Missing)?;
    let end = rest.find("---").ok_or(FrontmatterError::NotClosed)?;
    match parse_yaml(&rest[..end])? {
        Some(Node::Mapping(entries)) => Ok(entries),
        _ => Err(FrontmatterError::NotAMapping),
    }
}

fn parse_yaml(source: &str) -> Result<Option<Node>, FrontmatterError> {
    let invalid = |message: String| FrontmatterError::InvalidYaml(message);
    let mut scanner = Scanner::new(source.chars());
    for token in scanner.by_ref() {
        let line = token.0.line();
        let disallowed = match token.1 {
            TokenType::FlowSequenceStart | TokenType::FlowMappingStart => "flow style",
            TokenType::Anchor(_) | TokenType::Alias(_) => "anchors and aliases",
            TokenType::Tag(..) => "tags",
            _ => continue,
        };
        return Err(invalid(format!("{disallowed} not allowed (line {line})")));
    }
    if let Some(err) = scanner.get_error() {
        return Err(invalid(err.to_string()));
    }

    let mut parser = Parser::new_from_str(source);
    let mut next = move || {
        parser
            .next_token()
            .map(|(event, _)| event)
            .map_err(|err| invalid(err.to_string()))
    };
    loop {
        match next()? {
            Event::StreamStart | Event::DocumentStart => {}
            Event::StreamEnd => return Ok(None),
            event => return build(event, &mut next).map(Some),
        }
    }
}

fn build(
    event: Event,
    next: &mut impl FnMut() -> Result<Event, FrontmatterError>,
) -> Result<Node, FrontmatterError> {
    match event {
        Event::Scalar(value, ..) => Ok(Node::Scalar(value)),
        Event::SequenceStart(..) => {
            let mut items = Vec::new();
            loop {
                match next()? {
                    Event::SequenceEnd => return Ok(Node::Sequence(items)),
                    event => items.push(build(event, next)?),
                }
            }
        }
        Event::MappingStart(..) => {
            let mut entries: Vec<(String, Node)> = Vec::new();
            loop {
                let key = match next()? {
                    Event::MappingEnd => return Ok(Node::Mapping(entries)),
                    Event::Scalar(key, ..) => key,
                    _ => {
                        return Err(FrontmatterError::InvalidYaml(
                            "mapping keys must be strings".to_owned(),
                        ));
                    }
                };
                if entries.iter().any(|(existing, _)| *existing == key) {
                    return Err(FrontmatterError::InvalidYaml(format!(
                        "duplicate key '{key}'"
                    )));
                }
                let event = next()?;
                entries.push((key, build(event, next)?));
            }
        }
        _ => Err(FrontmatterError::InvalidYaml(
            "unexpected YAML event".to_owned(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_reads_plain_quoted_and_block_values() {
        let content = "---\nname: x\ntitle: \"quoted\"\nother: 'single'\ndescription: >-\n  first\n  second\n---\nname: body\n";

        assert_eq!(field(content, "name"), "x");
        assert_eq!(field(content, "title"), "quoted");
        assert_eq!(field(content, "other"), "single");
        assert_eq!(field(content, "description"), "first second");
        assert_eq!(field(content, "missing"), "");
    }

    #[test]
    fn field_is_empty_without_frontmatter() {
        assert_eq!(field("name: x\n", "name"), "");
    }

    #[test]
    fn has_frontmatter_needs_both_delimiter_lines() {
        assert!(has_frontmatter("---\nname: x\n---\n"));
        assert!(!has_frontmatter("---\nname: x\n"));
        assert!(!has_frontmatter("name: x\n---\n"));
        assert!(!has_frontmatter(""));
    }

    #[test]
    fn parse_skill_keeps_every_scalar_a_string() {
        let entries = parse_skill("---\nname: 123\nempty:\nmetadata:\n  a: true\n---\n").unwrap();

        assert_eq!(
            entries,
            [
                ("name".to_owned(), Node::Scalar("123".to_owned())),
                ("empty".to_owned(), Node::Scalar(String::new())),
                (
                    "metadata".to_owned(),
                    Node::Mapping(vec![("a".to_owned(), Node::Scalar("true".to_owned()))])
                ),
            ]
        );
    }

    #[test]
    fn parse_skill_rejects_what_strictyaml_rejects() {
        for content in [
            "---\nallowed-tools: [Read, Bash]\n---\n",
            "---\nmetadata: {a: b}\n---\n",
            "---\nname: &a x\ndescription: *a\n---\n",
            "---\nname: !!str x\n---\n",
            "---\nname: x\nname: y\n---\n",
            "---\nname: x\n  bad: indent\n---\n",
        ] {
            assert!(
                matches!(parse_skill(content), Err(FrontmatterError::InvalidYaml(_))),
                "{content:?}"
            );
        }
    }

    #[test]
    fn parse_skill_needs_a_mapping_between_the_delimiters() {
        assert!(matches!(
            parse_skill("# x\n"),
            Err(FrontmatterError::Missing)
        ));
        assert!(matches!(
            parse_skill("---\nname: x\n"),
            Err(FrontmatterError::NotClosed)
        ));
        assert!(matches!(
            parse_skill("---\n- a\n---\n"),
            Err(FrontmatterError::NotAMapping)
        ));
        assert!(matches!(
            parse_skill("---\n---\n"),
            Err(FrontmatterError::NotAMapping)
        ));
    }
}
