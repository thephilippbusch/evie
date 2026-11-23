//! Absolute paths and the protected-path rules of
//! `.claude/hooks/protected-paths.txt`.

use std::path::Path;
use std::process::{Command, Stdio};

use super::pattern;

pub const PROTECTED_PATHS: &str = ".claude/hooks/protected-paths.txt";
const ASK: &str = "If this file really needs a change, stop and ask the user.";

/// The absolute form of `path`, relative to `base`, with `.`, `..` and `//`
/// removed and the deepest existing directory resolved through symlinks
/// (`/tmp` becomes `/private/tmp`). `~/` starts at `home`.
pub fn absolute(path: &str, base: &str, home: &str) -> String {
    // Only the first line counts, as with `read` in a shell.
    let path = path.split('\n').next().unwrap_or_default();
    let joined = if path.starts_with('/') {
        path.to_owned()
    } else if let Some(rest) = path.strip_prefix("~/") {
        format!("{home}/{rest}")
    } else {
        format!("{base}/{path}")
    };

    let mut parts: Vec<&str> = Vec::new();
    for part in joined.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            part => parts.push(part),
        }
    }
    let normal = format!("/{}", parts.join("/"));

    let mut dir = normal.as_str();
    while dir != "/" && !Path::new(dir).is_dir() {
        dir = match dir.rsplit_once('/') {
            Some(("", _)) | None => "/",
            Some((parent, _)) => parent,
        };
    }
    let rest = &normal[dir.len()..];
    let real = std::fs::canonicalize(dir)
        .map(|real| real.to_string_lossy().into_owned())
        .unwrap_or_else(|_| dir.to_owned());
    let real = if real == "/" { "" } else { real.as_str() };
    let rest = if dir == "/" && !rest.is_empty() && !rest.starts_with('/') {
        format!("/{rest}")
    } else {
        rest.to_owned()
    };
    format!("{real}{rest}")
}

/// One line of `protected-paths.txt`: a glob and the reason shown to the agent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pattern: String,
    reason: String,
}

/// The rules of a `protected-paths.txt`. A file that can't be read protects
/// nothing.
pub fn read_rules(file: &Path) -> Vec<Rule> {
    std::fs::read_to_string(file)
        .map(|content| parse_rules(&content))
        .unwrap_or_default()
}

fn parse_rules(content: &str) -> Vec<Rule> {
    content
        .lines()
        .map(|line| line.trim_start_matches(is_space))
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let (pattern, reason) = line.split_once(is_space).unwrap_or((line, ""));
            Rule {
                pattern: pattern.strip_prefix('/').unwrap_or(pattern).to_owned(),
                reason: reason.trim_start_matches(is_space).to_owned(),
            }
        })
        .collect()
}

fn is_space(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\x0b' | '\x0c' | '\r')
}

/// The message for the agent when `absolute` is a protected path of the git
/// repository it lies in.
pub fn protected(absolute: &str, rules: &[Rule]) -> Option<String> {
    if rules.is_empty() {
        return None;
    }
    let mut dir = absolute.rsplit_once('/').map_or(absolute, |(dir, _)| dir);
    while !dir.is_empty() && !Path::new(dir).is_dir() {
        dir = dir.rsplit_once('/').map_or("", |(parent, _)| parent);
    }
    let top = git_toplevel(if dir.is_empty() { "/" } else { dir })?;
    let rel = absolute.strip_prefix(&format!("{top}/"))?;
    let name = rel.rsplit('/').next().unwrap_or(rel);

    let rule = rules.iter().find(|rule| {
        let pattern = rule.pattern.as_str();
        if pattern.ends_with('/') {
            pattern::matches(&format!("{pattern}*"), rel, true)
        } else if pattern.contains('/') {
            // "**/" also matches no directory.
            let mut zero = pattern.strip_prefix("**/").unwrap_or(pattern).to_owned();
            while zero.contains("/**/") {
                zero = zero.replacen("/**/", "/", 1);
            }
            pattern::matches(pattern, rel, true) || pattern::matches(&zero, rel, true)
        } else {
            pattern::matches(pattern, name, true)
        }
    })?;
    let reason = if rule.reason.is_empty() {
        "It must not be edited by hand."
    } else {
        &rule.reason
    };
    Some(format!(
        "`{rel}` is protected (rule `{}` in {PROTECTED_PATHS}). {reason} {ASK}",
        rule.pattern
    ))
}

fn git_toplevel(dir: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["-C", dir, "rev-parse", "--show-toplevel"])
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    output.status.success().then(|| {
        String::from_utf8_lossy(&output.stdout)
            .trim_end_matches('\n')
            .to_owned()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absolute_removes_dots_and_resolves_home() {
        assert_eq!(
            absolute("a/./b/../c", "/nonexistent", "/h"),
            "/nonexistent/a/c"
        );
        assert_eq!(absolute("/x//y/", "/base", "/h"), "/x/y");
        assert_eq!(
            absolute("~/f", "/base", "/nonexistent-home"),
            "/nonexistent-home/f"
        );
        assert_eq!(absolute("../../../..", "/a", "/h"), "");
    }

    #[test]
    fn absolute_resolves_the_deepest_existing_directory() {
        let dir = tempfile::tempdir().unwrap();
        let real = std::fs::canonicalize(dir.path()).unwrap();

        let path = absolute("new/file", dir.path().to_str().unwrap(), "/h");

        assert_eq!(path, format!("{}/new/file", real.display()));
    }

    #[test]
    fn rules_skip_comments_and_keep_the_reason() {
        let rules =
            parse_rules("# comment\n\n  /CHANGELOG.md   Owned by release-please.\nlibs/gen/\n");

        assert_eq!(
            rules,
            [
                Rule {
                    pattern: "CHANGELOG.md".to_owned(),
                    reason: "Owned by release-please.".to_owned()
                },
                Rule {
                    pattern: "libs/gen/".to_owned(),
                    reason: String::new()
                },
            ]
        );
    }
}
