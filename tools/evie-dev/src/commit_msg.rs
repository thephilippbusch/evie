//! Checks that a commit message or a PR title is a Conventional Commit.

use thiserror::Error;

/// The types a header may start with: the standard Conventional Commit types
/// plus `ai` for agent configuration.
pub const TYPES: [&str; 12] = [
    "feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore", "revert",
    "ai",
];

pub const MAX_HEADER_LENGTH: usize = 100;

/// Headers that git and GitHub write themselves. A commit message may start
/// with them; a PR title may not, since it becomes the squash commit on `main`.
const GENERATED_PREFIXES: [&str; 5] = ["Merge ", "Revert \"", "fixup! ", "squash! ", "amend! "];

/// What is checked, which changes the bypass rules and the wording.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Subject {
    CommitMessage,
    PrTitle,
}

impl Subject {
    fn noun(self) -> &'static str {
        match self {
            Self::CommitMessage => "commit message",
            Self::PrTitle => "PR title",
        }
    }
}

#[derive(Debug, Error)]
pub enum CommitMsgError {
    #[error("the message is empty")]
    Empty,

    #[error("the header does not match the format")]
    Format { header: String },

    #[error("the header is longer than {MAX_HEADER_LENGTH} characters")]
    TooLong { header: String },

    #[error("the header must be followed by an empty line")]
    NoBlankLine { header: String },
}

impl CommitMsgError {
    /// The header the error is about, as the report shows it.
    pub fn header(&self) -> &str {
        match self {
            Self::Empty => "(empty)",
            Self::Format { header } | Self::TooLong { header } | Self::NoBlankLine { header } => {
                header
            }
        }
    }

    /// The text for stderr: the problem, the header and the expected format.
    pub fn report(&self, subject: Subject) -> String {
        format!(
            "Invalid {}: {self}\n\n  {}\n\nExpected a Conventional Commit: <type>[(scope)][!]: <description>\nAllowed types: {}\nExample: fix(auth): reject expired sessions\n",
            subject.noun(),
            self.header(),
            TYPES.join(", "),
        )
    }
}

/// Checks a raw commit message, as git passes it to the commit-msg hook, or
/// a PR title.
pub fn check(raw: &str, subject: Subject) -> Result<(), CommitMsgError> {
    let lines = clean_up(raw);
    let Some(header_index) = lines
        .iter()
        .position(|line| line.chars().any(|c| !is_space(c)))
    else {
        return Err(CommitMsgError::Empty);
    };
    let header = lines[header_index];

    if subject == Subject::CommitMessage
        && GENERATED_PREFIXES
            .iter()
            .any(|prefix| header.starts_with(prefix))
    {
        return Ok(());
    }

    let header_owned = || header.to_owned();
    if !matches_format(header) {
        return Err(CommitMsgError::Format {
            header: header_owned(),
        });
    }
    if header.chars().count() > MAX_HEADER_LENGTH {
        return Err(CommitMsgError::TooLong {
            header: header_owned(),
        });
    }
    if lines
        .get(header_index + 1)
        .is_some_and(|line| !line.is_empty())
    {
        return Err(CommitMsgError::NoBlankLine {
            header: header_owned(),
        });
    }
    Ok(())
}

/// Drops everything from the scissors line of `git commit -v` on and every
/// comment line, as git does, and the trailing empty lines.
fn clean_up(raw: &str) -> Vec<&str> {
    let mut lines: Vec<&str> = raw
        .split('\n')
        .take_while(|line| !is_scissors(line))
        .filter(|line| !line.starts_with('#'))
        .collect();
    while lines.last().is_some_and(|line| line.is_empty()) {
        lines.pop();
    }
    lines
}

/// `# ------------------------ >8 ------------------------`, with any number
/// of dashes on either side.
fn is_scissors(line: &str) -> bool {
    line.strip_prefix("# ")
        .map(|rest| rest.trim_start_matches('-'))
        .and_then(|rest| rest.strip_prefix(" >8 "))
        .is_some_and(|rest| rest.chars().all(|c| c == '-'))
}

/// `<type>[(scope)][!]: <description>`, where the scope has no spaces or
/// parentheses and the description starts with a non-space character.
fn matches_format(header: &str) -> bool {
    let Some(rest) = TYPES.iter().find_map(|t| header.strip_prefix(t)) else {
        return false;
    };
    let rest = match rest.strip_prefix('(') {
        Some(scoped) => {
            let Some(end) = scoped.find(')') else {
                return false;
            };
            let scope = &scoped[..end];
            if scope.is_empty() || scope.chars().any(|c| c == '(' || is_space(c)) {
                return false;
            }
            &scoped[end + 1..]
        }
        None => rest,
    };
    let rest = rest.strip_prefix('!').unwrap_or(rest);
    rest.strip_prefix(": ")
        .and_then(|description| description.chars().next())
        .is_some_and(|c| !is_space(c))
}

/// The POSIX `[:space:]` class.
fn is_space(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\x0b' | '\x0c' | '\r')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scissors_line_takes_any_number_of_dashes() {
        assert!(is_scissors("#  >8 "));
        assert!(!is_scissors("# >8 "));
        assert!(is_scissors(
            "# ------------------------ >8 ------------------------"
        ));
        assert!(!is_scissors("# ---- >8 ---- x"));
        assert!(!is_scissors("#---- >8 ----"));
    }

    #[test]
    fn clean_up_drops_comments_and_everything_below_the_scissors() {
        let lines = clean_up("feat: x\n# comment\n\nbody\n# ---- >8 ----\ndiff\n\n");

        assert_eq!(lines, ["feat: x", "", "body"]);
    }
}
