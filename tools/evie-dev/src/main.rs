use std::io::Read;
use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Context;
use clap::{Parser, Subcommand};
use evie_dev::commit_msg::{self, Subject};

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
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::CommitMsg { pr_title, file } => commit_msg(pr_title, file),
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
