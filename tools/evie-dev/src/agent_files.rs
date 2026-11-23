//! Checks the agent files: skills, subagents, `AGENTS.md` and `CLAUDE.md`.
//!
//! The rules are the "CI: yes" rows of the Budgets table in
//! `.agents/skills/authoring-skills/SKILL.md`, which stays their source of
//! truth:
//!
//! - each skill in `.agents/skills/` follows the Agent Skills spec
//!   (frontmatter keys, name, description, compatibility), and its name is
//!   ASCII only
//! - `SKILL.md` files are under 500 lines, `AGENTS.md` files under 200 lines
//!   and subagent files in `.claude/agents/` under 100 lines
//! - each subagent has a valid name that equals its file name, and a
//!   non-empty description
//! - relative links in skills, subagents and `AGENTS.md` files resolve
//! - symlinks resolve; every `CLAUDE.md` is a symlink to the `AGENTS.md`
//!   beside it, every `AGENTS.md` has one, and `.claude/skills` points to
//!   `.agents/skills`
//!
//! The files are those git tracks plus untracked files that aren't ignored.

mod frontmatter;
mod links;
mod skill_spec;

use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Command;

use thiserror::Error;

use frontmatter::{field, has_frontmatter};
use links::relative_links;

pub(crate) const SKILLS_DIR: &str = ".agents/skills";
pub(crate) const AGENTS_DIR: &str = ".claude/agents";

const SKILL_MD_LINES: usize = 500;
const SUBAGENT_LINES: usize = 100;
const AGENTS_MD_LINES: usize = 200;
const MAX_SUBAGENT_NAME_LENGTH: usize = 64;

#[derive(Debug, Error)]
pub enum AgentFilesError {
    #[error("could not run git ls-files")]
    Spawn(#[source] std::io::Error),

    #[error("git ls-files failed: {0}")]
    ListFiles(String),
}

/// One broken rule: the path (with `:line` for a link) and what is wrong.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    pub path: String,
    pub message: String,
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.path, self.message)
    }
}

/// The last line of a failed check, which points to the budgets.
pub fn summary(problems: usize) -> String {
    format!(
        "{problems} problem(s) found. See the budgets in {SKILLS_DIR}/authoring-skills/SKILL.md."
    )
}

/// Checks the agent files of the repository at `root`, in the order the
/// problems are reported.
pub fn validate(root: &Path) -> Result<Vec<Problem>, AgentFilesError> {
    let files = list_files(root)?;
    let mut check = Checker {
        root,
        problems: Vec::new(),
    };

    let regular = |path: &&String| root.join(path).is_file();
    let agents_md: Vec<&String> = files
        .iter()
        .filter(|f| file_name(f) == "AGENTS.md")
        .filter(regular)
        .collect();
    let skill_md: Vec<&String> = files
        .iter()
        .filter(|f| is_markdown_below(f, SKILLS_DIR))
        .filter(regular)
        .collect();
    let subagents: Vec<&String> = files
        .iter()
        .filter(|f| is_markdown_below(f, AGENTS_DIR))
        .filter(regular)
        .collect();
    let claude_md: Vec<&String> = files
        .iter()
        .filter(|f| file_name(f) == "CLAUDE.md")
        .collect();

    for dir in skill_dirs(root) {
        check.skill(&dir);
    }
    for file in skill_md {
        check.links(file);
    }
    for file in subagents {
        check.subagent(file);
    }
    for file in agents_md {
        check.agents_md(file);
    }
    for file in claude_md {
        check.claude_md(file);
    }
    for file in &files {
        let path = root.join(file);
        if path.is_symlink() && !path.exists() {
            let target = read_link(&path);
            check.problem(file, format!("broken symlink to {target}"));
        }
    }
    check.claude_skills_link();

    Ok(check.problems)
}

/// Tracked files plus untracked files that aren't ignored, that exist.
fn list_files(root: &Path) -> Result<Vec<String>, AgentFilesError> {
    let output = Command::new("git")
        .args([
            "-c",
            "core.quotePath=false",
            "ls-files",
            "--cached",
            "--others",
            "--exclude-standard",
        ])
        .current_dir(root)
        .output()
        .map_err(AgentFilesError::Spawn)?;
    if !output.status.success() {
        return Err(AgentFilesError::ListFiles(
            String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        ));
    }
    let mut files: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| {
            let path = root.join(line);
            path.exists() || path.is_symlink()
        })
        .map(str::to_owned)
        .collect();
    files.sort();
    files.dedup();
    Ok(files)
}

fn file_name(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

fn parent(path: &str) -> &str {
    path.rsplit_once('/').map_or(".", |(dir, _)| dir)
}

/// A `.md` file at any depth below `dir`.
fn is_markdown_below(path: &str, dir: &str) -> bool {
    path.strip_prefix(dir)
        .and_then(|rest| rest.strip_prefix('/'))
        .is_some_and(|rest| rest.len() > ".md".len() && rest.ends_with(".md"))
}

/// The skill directories, as `.agents/skills/<name>`, sorted.
fn skill_dirs(root: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(root.join(SKILLS_DIR)) else {
        return Vec::new();
    };
    let mut dirs: Vec<String> = entries
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| !name.starts_with('.'))
        .map(|name| format!("{SKILLS_DIR}/{name}"))
        .collect();
    dirs.sort();
    dirs
}

fn read_link(path: &Path) -> String {
    std::fs::read_link(path)
        .map(|target| target.display().to_string())
        .unwrap_or_default()
}

/// The text of a file, or empty when it can't be read.
fn read(path: &Path) -> String {
    std::fs::read(path)
        .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
        .unwrap_or_default()
}

/// `[a-z0-9]+(-[a-z0-9]+)*`
fn is_kebab_name(name: &str) -> bool {
    !name.is_empty()
        && name.split('-').all(|part| {
            !part.is_empty() && part.bytes().all(|b| matches!(b, b'a'..=b'z' | b'0'..=b'9'))
        })
}

struct Checker<'a> {
    root: &'a Path,
    problems: Vec<Problem>,
}

impl Checker<'_> {
    fn problem(&mut self, path: impl Into<String>, message: impl Into<String>) {
        self.problems.push(Problem {
            path: path.into(),
            message: message.into(),
        });
    }

    fn path(&self, path: &str) -> PathBuf {
        self.root.join(path)
    }

    fn skill(&mut self, dir: &str) {
        let skill_md = format!("{dir}/SKILL.md");
        let spec_errors = skill_spec::validate(&self.path(dir));
        if spec_errors.is_empty() {
            let name = field(&read(&self.path(&skill_md)), "name");
            if !is_kebab_name(&name) {
                self.problem(
                    &skill_md,
                    format!("skill name '{name}' may only use a-z, 0-9 and single hyphens"),
                );
            }
        }
        for message in spec_errors {
            self.problem(&skill_md, message);
        }
        if self.path(&skill_md).is_file() {
            self.lines(&skill_md, SKILL_MD_LINES);
        }
    }

    /// The file has fewer than `limit` lines, counted as `wc -l` does.
    fn lines(&mut self, file: &str, limit: usize) {
        let lines = std::fs::read(self.path(file))
            .map(|bytes| bytes.iter().filter(|b| **b == b'\n').count())
            .unwrap_or(0);
        if lines >= limit {
            self.problem(
                file,
                format!("has {lines} lines; the budget is under {limit}"),
            );
        }
    }

    /// Every relative link points to an existing path, resolved from the
    /// file's directory, or from the repository root for `/paths`.
    fn links(&mut self, file: &str) {
        let base = parent(file);
        for link in relative_links(&read(&self.path(file))) {
            let path = match link.target.strip_prefix('/') {
                Some(from_root) => self.path(from_root),
                None => self.path(base).join(&link.target),
            };
            if !path.exists() {
                self.problem(
                    format!("{file}:{}", link.line),
                    format!("broken link to {}", link.target),
                );
            }
        }
    }

    fn subagent(&mut self, file: &str) {
        self.lines(file, SUBAGENT_LINES);
        self.links(file);
        let content = read(&self.path(file));
        if !has_frontmatter(&content) {
            self.problem(file, "has no YAML frontmatter between --- lines");
            return;
        }
        let name = field(&content, "name");
        let expected = file_name(file).strip_suffix(".md").unwrap_or(file);
        if name.is_empty() {
            self.problem(file, "frontmatter is missing 'name'");
        } else if !is_kebab_name(&name) || name.chars().count() > MAX_SUBAGENT_NAME_LENGTH {
            self.problem(
                file,
                format!("subagent name '{name}' must be 1-64 chars of a-z, 0-9 and single hyphens"),
            );
        } else if name != expected {
            self.problem(
                file,
                format!("subagent name '{name}' must equal the file name '{expected}'"),
            );
        }
        if field(&content, "description").is_empty() {
            self.problem(file, "frontmatter is missing a non-empty 'description'");
        }
    }

    fn agents_md(&mut self, file: &str) {
        self.lines(file, AGENTS_MD_LINES);
        self.links(file);
        let claude = match file.rsplit_once('/') {
            Some((dir, _)) => format!("{dir}/CLAUDE.md"),
            None => "CLAUDE.md".to_owned(),
        };
        let path = self.path(&claude);
        if !path.is_symlink() && !path.exists() {
            self.problem(
                file,
                format!("has no CLAUDE.md beside it; add one with: ln -s AGENTS.md {claude}"),
            );
        }
    }

    fn claude_md(&mut self, file: &str) {
        let path = self.path(file);
        if !path.is_symlink() {
            self.problem(file, "must be a symlink to AGENTS.md, not a regular file");
            return;
        }
        let target = read_link(&path);
        if target != "AGENTS.md" {
            self.problem(
                file,
                format!("must point to AGENTS.md in the same directory, not {target}"),
            );
        } else if !self.path(parent(file)).join("AGENTS.md").is_file() {
            self.problem(file, "points to AGENTS.md, which doesn't exist");
        }
    }

    fn claude_skills_link(&mut self) {
        let link = self.path(".claude/skills");
        if !link.is_symlink() {
            self.problem(
                ".claude/skills",
                format!("must be a symlink to ../{SKILLS_DIR}"),
            );
            return;
        }
        let points_to_skills = match (link.canonicalize(), self.path(SKILLS_DIR).canonicalize()) {
            (Ok(link), Ok(skills)) => link == skills,
            _ => false,
        };
        if link.exists() && !points_to_skills {
            let target = read_link(&link);
            self.problem(
                ".claude/skills",
                format!("must point to {SKILLS_DIR}, not {target}"),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kebab_names_have_single_hyphens_between_parts() {
        for name in ["a", "a-b", "a1-2b"] {
            assert!(is_kebab_name(name), "{name}");
        }
        for name in ["", "-a", "a-", "a--b", "A", "a_b", "ä"] {
            assert!(!is_kebab_name(name), "{name}");
        }
    }

    #[test]
    fn markdown_below_needs_a_name_before_the_extension() {
        assert!(is_markdown_below(".agents/skills/x/SKILL.md", SKILLS_DIR));
        assert!(is_markdown_below(".claude/agents/a.md", AGENTS_DIR));
        assert!(!is_markdown_below(".claude/agents/.md", AGENTS_DIR));
        assert!(!is_markdown_below(".claude/agents/a.txt", AGENTS_DIR));
        assert!(!is_markdown_below(".claude/agentsx/a.md", AGENTS_DIR));
    }
}
