//! Formats the staged files with dprint and stages the result.
//!
//! Files that also have unstaged changes are only checked, never rewritten,
//! so the commit never picks up changes that weren't staged. dprint decides
//! which files it formats (see `dprint.json`) and skips the rest.

use std::collections::HashSet;
use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FormatStagedError {
    #[error("dprint is not available. In the repository root, run: proto trust && proto install")]
    DprintMissing,

    #[error(
        "These files are partly staged, so they were checked but not formatted.\nRun `moon run root:format` (or `dprint fmt <file>`), stage the result and commit again."
    )]
    PartlyStagedNotFormatted,

    /// The command printed its own error, so the message only names it.
    #[error("`{command}` failed")]
    Failed { command: String, code: i32 },

    #[error("could not run `{command}`")]
    Spawn {
        command: String,
        #[source]
        source: std::io::Error,
    },
}

impl FormatStagedError {
    /// The exit code: a failed command's own code, as `set -e` would give.
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Failed { code, .. } => *code,
            _ => 1,
        }
    }
}

/// Formats the files staged in `repo`, using the given dprint executable.
pub fn run(repo: &Path, dprint: &OsStr) -> Result<(), FormatStagedError> {
    let staged = output_lines(
        repo,
        "git",
        &[
            "-c",
            "core.quotePath=false",
            "diff",
            "--cached",
            "--name-only",
            "--diff-filter=ACMR",
        ],
    )?;
    if staged.is_empty() {
        return Ok(());
    }

    let dprint_runs = Command::new(dprint)
        .arg("--version")
        .current_dir(repo)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success());
    if !dprint_runs {
        return Err(FormatStagedError::DprintMissing);
    }

    let unstaged: HashSet<String> = output_lines(
        repo,
        "git",
        &["-c", "core.quotePath=false", "diff", "--name-only"],
    )?
    .into_iter()
    .collect();
    let (check, format): (Vec<String>, Vec<String>) =
        staged.into_iter().partition(|file| unstaged.contains(file));

    if !format.is_empty() {
        run_with_files(
            repo,
            dprint,
            &["fmt", "--allow-no-files", "--stdin-files"],
            &format,
        )?;
        run_with_files(
            repo,
            OsStr::new("git"),
            &["--literal-pathspecs", "add", "--pathspec-from-file=-"],
            &format,
        )?;
    }

    if !check.is_empty() {
        let checked = run_with_files(
            repo,
            dprint,
            &["check", "--allow-no-files", "--stdin-files"],
            &check,
        );
        if let Err(FormatStagedError::Failed { .. }) = checked {
            return Err(FormatStagedError::PartlyStagedNotFormatted);
        }
        checked?;
    }
    Ok(())
}

fn describe(program: &OsStr, args: &[&str]) -> String {
    format!("{} {}", program.to_string_lossy(), args.join(" "))
}

/// Runs a command and returns its stdout lines. Its stderr goes through.
fn output_lines(
    repo: &Path,
    program: &str,
    args: &[&str],
) -> Result<Vec<String>, FormatStagedError> {
    let command = || describe(OsStr::new(program), args);
    let output = Command::new(program)
        .args(args)
        .current_dir(repo)
        .stderr(Stdio::inherit())
        .output()
        .map_err(|source| FormatStagedError::Spawn {
            command: command(),
            source,
        })?;
    if !output.status.success() {
        return Err(FormatStagedError::Failed {
            command: command(),
            code: output.status.code().unwrap_or(1),
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect())
}

/// Runs a command with one file name per line on stdin.
fn run_with_files(
    repo: &Path,
    program: &OsStr,
    args: &[&str],
    files: &[String],
) -> Result<(), FormatStagedError> {
    let command = || describe(program, args);
    let spawn_error = |source| FormatStagedError::Spawn {
        command: command(),
        source,
    };
    let mut child = Command::new(program)
        .args(args)
        .current_dir(repo)
        .stdin(Stdio::piped())
        .spawn()
        .map_err(spawn_error)?;
    let mut input = files.join("\n");
    input.push('\n');
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_bytes()).map_err(spawn_error)?;
    }
    let status = child.wait().map_err(spawn_error)?;
    if !status.success() {
        return Err(FormatStagedError::Failed {
            command: command(),
            code: status.code().unwrap_or(1),
        });
    }
    Ok(())
}
