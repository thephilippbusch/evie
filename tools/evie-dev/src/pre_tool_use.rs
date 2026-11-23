//! The Claude Code PreToolUse guardrails, registered in `.claude/settings.json`.
//!
//! A tool call is blocked (exit 2, the reason on stderr) when it would:
//!
//! - edit a path listed in `.claude/hooks/protected-paths.txt` (Edit, Write,
//!   MultiEdit, NotebookEdit),
//! - force-push or push to `main` (Bash),
//! - run `drizzle-kit push` (Bash; humans run it in local dev),
//! - read or touch `.env` files, keys or other secrets (Bash),
//! - write to a protected path through the shell (Bash).
//!
//! It backs up the deny rules in `.claude/settings.json`, which match only
//! the command text as written; this hook also parses compound commands,
//! `sh -c`, `git -C`, refspecs and the current branch.

mod paths;
mod pattern;
mod shell;

use std::fmt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde_json::Value;

pub use paths::PROTECTED_PATHS;

const MSG_FORCE: &str = "force-pushing is not allowed (AGENTS.md rules). Push new commits instead of rewriting pushed history. If a force-push is really needed, ask the user to run it.";
const MSG_MAIN: &str = "pushing to `main` is not allowed (AGENTS.md rules). Work on a branch, push that branch and open a PR.";
const MSG_DRIZZLE: &str = "`drizzle-kit push` is for humans in local dev only. Generate a migration, review it and apply it with `moon run :migrate`, or ask the user to push their local database.";

/// A blocked tool call, with the reason for the agent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blocked(pub String);

impl fmt::Display for Blocked {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Blocked by evie-dev pre-tool-use: {}", self.0)
    }
}

impl std::error::Error for Blocked {}

/// What the hook reads from its environment.
#[derive(Debug, Clone)]
pub struct Env {
    /// `CLAUDE_PROJECT_DIR`: the checkout Claude Code runs in.
    pub project_dir: Option<String>,
    /// `HOME`, for `~/` and a bare `cd`.
    pub home: String,
    /// The process's working directory, used when the input has no `cwd`.
    pub current_dir: String,
}

impl Env {
    pub fn from_process() -> Self {
        let current_dir = std::env::current_dir()
            .map(|dir| dir.to_string_lossy().into_owned())
            .unwrap_or_default();
        Self {
            project_dir: std::env::var("CLAUDE_PROJECT_DIR")
                .ok()
                .filter(|dir| !dir.is_empty()),
            home: std::env::var("HOME").unwrap_or_default(),
            current_dir,
        }
    }

    /// The protected-path list of the project: `CLAUDE_PROJECT_DIR`, or else
    /// the git repository of the working directory.
    fn protected_paths(&self) -> Option<PathBuf> {
        let root = match &self.project_dir {
            Some(dir) => dir.clone(),
            None => {
                let output = Command::new("git")
                    .args(["rev-parse", "--show-toplevel"])
                    .current_dir(&self.current_dir)
                    .stdin(Stdio::null())
                    .stderr(Stdio::null())
                    .output()
                    .ok()
                    .filter(|output| output.status.success())?;
                String::from_utf8_lossy(&output.stdout)
                    .trim_end_matches('\n')
                    .to_owned()
            }
        };
        Some(Path::new(&root).join(PROTECTED_PATHS))
    }
}

/// Checks one tool call, given as the hook's JSON input.
pub fn check(input: &str, env: &Env) -> Result<(), Blocked> {
    let input: Value = serde_json::from_str(input).map_err(|_| {
        Blocked(
            "the hook input is not valid JSON, so the guardrails cannot check this call."
                .to_owned(),
        )
    })?;
    let cwd = Some(text(&input["cwd"]))
        .filter(|cwd| !cwd.is_empty() && Path::new(cwd).is_dir())
        .unwrap_or_else(|| {
            env.project_dir
                .clone()
                .unwrap_or_else(|| env.current_dir.clone())
        });
    let rules = env
        .protected_paths()
        .map(|file| paths::read_rules(&file))
        .unwrap_or_default();

    match text(&input["tool_name"]).as_str() {
        "Edit" | "Write" | "MultiEdit" | "NotebookEdit" => {
            let tool_input = &input["tool_input"];
            let path = Some(text(&tool_input["file_path"]))
                .filter(|path| !path.is_empty())
                .unwrap_or_else(|| text(&tool_input["notebook_path"]));
            if path.is_empty() {
                return Ok(());
            }
            let absolute = paths::absolute(&path, &cwd, &env.home);
            match paths::protected(&absolute, &rules) {
                Some(message) => Err(Blocked(message)),
                None => Ok(()),
            }
        }
        "Bash" => {
            let command = text(&input["tool_input"]["command"]);
            shell::Analyzer::new(cwd, &env.home, &rules).analyze(&command, 0)
        }
        _ => Ok(()),
    }
}

/// A JSON value as `jq -r '.x // empty'` prints it: strings raw, `null`,
/// `false` and missing values empty, anything else as JSON.
fn text(value: &Value) -> String {
    let text = match value {
        Value::Null | Value::Bool(false) => String::new(),
        Value::String(s) => s.clone(),
        other => other.to_string(),
    };
    text.trim_end_matches('\n').to_owned()
}
