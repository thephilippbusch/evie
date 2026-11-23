use std::ffi::OsStr;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::Context;
use clap::{Parser, Subcommand};
use evie_dev::agent_files;
use evie_dev::commit_msg::{self, Subject};
use evie_dev::format_staged::{self, FormatStagedError};

/// The repository's development tool. Run it through `tools/evie-dev/bin/evie-dev`,
/// which builds it first when needed.
#[derive(Parser)]
#[command(name = "evie-dev")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check that a commit message is a Conventional Commit. Run by the commit-msg hook.
    CommitMsg {
        /// Check a PR title instead. It becomes the squash commit on main, so
        /// merge, revert and fixup headers are rejected too.
        #[arg(long)]
        pr_title: bool,
        /// The commit message file, as git passes it to the hook. Without it,
        /// or with `-`, the message is read from stdin.
        file: Option<PathBuf>,
    },
    /// Format the staged files with dprint and stage the result. Run by the pre-commit hook.
    FormatStaged,
    /// Check skills, subagents and AGENTS.md files against the authoring-skills
    /// budgets. Run from the repository root.
    ValidateAgentFiles,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::CommitMsg { pr_title, file } => commit_msg(pr_title, file),
        Command::FormatStaged => Ok(format_staged()),
        Command::ValidateAgentFiles => validate_agent_files(),
    };
    result.unwrap_or_else(|err| {
        eprintln!("evie-dev: {err:#}");
        ExitCode::FAILURE
    })
}

fn commit_msg(pr_title: bool, file: Option<PathBuf>) -> anyhow::Result<ExitCode> {
    let subject = if pr_title {
        Subject::PrTitle
    } else {
        Subject::CommitMessage
    };
    let bytes = match file {
        Some(path) if path.as_os_str() != "-" => std::fs::read(&path)
            .with_context(|| format!("reading the commit message from {}", path.display()))?,
        _ => {
            let mut bytes = Vec::new();
            std::io::stdin()
                .read_to_end(&mut bytes)
                .context("reading the commit message from stdin")?;
            bytes
        }
    };
    match commit_msg::check(&String::from_utf8_lossy(&bytes), subject) {
        Ok(()) => Ok(ExitCode::SUCCESS),
        Err(err) => {
            eprint!("{}", err.report(subject));
            Ok(ExitCode::FAILURE)
        }
    }
}

fn format_staged() -> ExitCode {
    let Err(err) = format_staged::run(Path::new("."), OsStr::new("dprint")) else {
        return ExitCode::SUCCESS;
    };
    let code = err.exit_code();
    match err {
        FormatStagedError::DprintMissing => eprintln!("{err}"),
        FormatStagedError::PartlyStagedNotFormatted => eprintln!("\n{err}"),
        // The command printed its own error.
        FormatStagedError::Failed { .. } => {}
        FormatStagedError::Spawn { .. } => eprintln!("evie-dev: {:#}", anyhow::Error::new(err)),
    }
    exit_code(code)
}

fn validate_agent_files() -> anyhow::Result<ExitCode> {
    let problems = agent_files::validate(Path::new(".")).context("listing the agent files")?;
    if problems.is_empty() {
        println!("Agent files are valid.");
        return Ok(ExitCode::SUCCESS);
    }
    for problem in &problems {
        eprintln!("{problem}");
    }
    eprintln!(
        "\n{} problem(s) found. See the budgets in {}/authoring-skills/SKILL.md.",
        problems.len(),
        agent_files::SKILLS_DIR
    );
    Ok(ExitCode::FAILURE)
}

/// A child process's exit code, which is 0 to 255 on every platform it runs on.
fn exit_code(code: i32) -> ExitCode {
    ExitCode::from(u8::try_from(code).unwrap_or(1))
}
