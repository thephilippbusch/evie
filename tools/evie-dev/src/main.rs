use std::ffi::OsStr;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::Context;
use clap::{Parser, Subcommand};
use evie_dev::agent_files;
use evie_dev::commit_msg::{self, Subject};
use evie_dev::format_staged;
use evie_dev::pre_tool_use::{self, Blocked};

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
    /// Check a Claude Code tool call, read as JSON from stdin, against the
    /// guardrails. Exits 2 with the reason on stderr to block it.
    PreToolUse,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::CommitMsg { pr_title, file } => commit_msg(pr_title, file),
        Command::FormatStaged => Ok(format_staged()),
        Command::ValidateAgentFiles => validate_agent_files(),
        Command::PreToolUse => Ok(pre_tool_use()),
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
    eprint!("{}", err.report());
    exit_code(err.exit_code())
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
    eprintln!("\n{}", agent_files::summary(problems.len()));
    Ok(ExitCode::FAILURE)
}

/// Fails closed: unreadable input and panics block the call too, since any
/// exit code but 2 would let it through.
fn pre_tool_use() -> ExitCode {
    let mut input = String::new();
    let result = if std::io::stdin().read_to_string(&mut input).is_err() {
        Err(Blocked::UnreadableInput)
    } else {
        let env = pre_tool_use::Env::from_process();
        std::panic::catch_unwind(|| pre_tool_use::check(&input, &env))
            .unwrap_or(Err(Blocked::Crashed))
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(blocked) => {
            eprintln!("{}", blocked.report());
            ExitCode::from(Blocked::EXIT_CODE)
        }
    }
}

/// A child process's exit code, which is 0 to 255 on every platform it runs on.
fn exit_code(code: i32) -> ExitCode {
    ExitCode::from(u8::try_from(code).unwrap_or(1))
}
