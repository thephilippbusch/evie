//! The repository's development tool: the commit hooks, the agent file check
//! and the Claude Code guardrails, one subcommand each. `main.rs` only parses
//! the arguments and maps the results to exit codes.

pub mod agent_files;
pub mod commit_msg;
pub mod format_staged;
pub mod pre_tool_use;
