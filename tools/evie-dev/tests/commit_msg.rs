use std::io::Write;
use std::process::{Command, Stdio};

use evie_dev::commit_msg::{CommitMsgError, Subject, check};

/// Messages every subject accepts.
const VALID: &[&str] = &[
    "feat: add events",
    "fix(auth): reject expired sessions",
    "feat!: drop the v1 api",
    "feat(api)!: drop the v1 api",
    "ai: add the commit skill",
    "chore(evie-dev): add the tool",
    "revert: undo the change",
    "feat: x\n\nThe body explains why.\n",
    "feat: x\n",
    "feat: x\n\n\n",
    "\n\nfeat: leading empty lines are skipped",
    "# a comment first\nfeat: x\n# another comment\n",
    "feat: x\n# comment right after the header\n",
    "feat: x\n# ------------------------ >8 ------------------------\nnot a blank line\n",
    "feat(a.b/c_d): scopes take any non-space character",
    "feat: the description may have trailing spaces  ",
];

/// Messages git or GitHub write, which only a commit message may have.
const GENERATED: &[&str] = &[
    "Merge branch 'main' into feature",
    "Merge pull request #1 from x/y",
    "Revert \"feat: add events\"",
    "fixup! feat: add events",
    "squash! feat: add events",
    "amend! feat: add events",
];

#[derive(Debug, PartialEq)]
enum Rejected {
    Empty,
    Format,
    TooLong,
    NoBlankLine,
}

fn rejected(err: &CommitMsgError) -> Rejected {
    match err {
        CommitMsgError::Empty => Rejected::Empty,
        CommitMsgError::Format { .. } => Rejected::Format,
        CommitMsgError::TooLong { .. } => Rejected::TooLong,
        CommitMsgError::NoBlankLine { .. } => Rejected::NoBlankLine,
    }
}

/// Messages every subject rejects, with the rule each one breaks.
const INVALID: &[(&str, Rejected)] = &[
    ("", Rejected::Empty),
    ("\n  \n\t\n", Rejected::Empty),
    ("# only a comment\n", Rejected::Empty),
    ("# ---- >8 ----\nfeat: x\n", Rejected::Empty),
    ("add events", Rejected::Format),
    ("feature: add events", Rejected::Format),
    ("Feat: add events", Rejected::Format),
    ("feat:add events", Rejected::Format),
    ("feat:  two spaces", Rejected::Format),
    ("feat: ", Rejected::Format),
    ("feat()", Rejected::Format),
    ("feat(): empty scope", Rejected::Format),
    ("feat(a b): space in scope", Rejected::Format),
    ("feat((a)): nested scope", Rejected::Format),
    ("feat(a: unclosed scope", Rejected::Format),
    ("feat!!: two bangs", Rejected::Format),
    ("feat !: space before the bang", Rejected::Format),
    (" feat: leading space", Rejected::Format),
    ("merge branch 'main'", Rejected::Format),
    ("Revert feat: no quote", Rejected::Format),
    ("fixup!feat: no space", Rejected::Format),
    (
        "feat: a header that goes on past the limit of one hundred characters, which the check rejects as too long",
        Rejected::TooLong,
    ),
    ("feat: x\nbody without a blank line", Rejected::NoBlankLine),
    ("feat: x\n \nbody", Rejected::NoBlankLine),
];

#[test]
fn rejects_invalid_messages_with_the_matching_error() {
    for (message, expected) in INVALID {
        for subject in [Subject::CommitMessage, Subject::PrTitle] {
            let err = check(message, subject).expect_err(message);
            assert_eq!(&rejected(&err), expected, "{subject:?}: {message:?}");
        }
    }
}

#[test]
fn accepts_valid_messages_and_titles() {
    for message in VALID {
        for subject in [Subject::CommitMessage, Subject::PrTitle] {
            assert!(check(message, subject).is_ok(), "{subject:?}: {message:?}");
        }
    }
}

#[test]
fn accepts_generated_headers_only_in_commit_messages() {
    for message in GENERATED {
        assert!(
            check(message, Subject::CommitMessage).is_ok(),
            "{message:?}"
        );
        assert!(
            matches!(
                check(message, Subject::PrTitle),
                Err(CommitMsgError::Format { .. })
            ),
            "{message:?}"
        );
    }
}

#[test]
fn allows_a_header_of_exactly_the_maximum_length() {
    let header = format!("feat: {}", "x".repeat(94));

    assert_eq!(header.chars().count(), 100);
    assert!(check(&header, Subject::CommitMessage).is_ok());
}

#[test]
fn counts_the_header_length_in_characters() {
    let header = format!("feat: {}", "ä".repeat(94));

    assert!(check(&header, Subject::CommitMessage).is_ok());
}

fn run(args: &[&str], stdin: &str) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_evie-dev"))
        .arg("commit-msg")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(stdin.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn binary_reads_the_message_file_and_exits_zero_when_valid() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("COMMIT_EDITMSG");
    std::fs::write(&file, "feat: add events\n").unwrap();

    let output = run(&[file.to_str().unwrap()], "");

    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn binary_reads_stdin_for_a_dash_and_reports_the_problem() {
    let output = run(&["--pr-title", "-"], "Merge branch 'main'\n");
    let stderr = String::from_utf8(output.stderr).unwrap();

    assert_eq!(output.status.code(), Some(1));
    assert!(stderr.starts_with(
        "Invalid PR title: the header does not match the format\n\n  Merge branch 'main'\n"
    ));
    assert!(stderr.contains("Allowed types: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert, ai\n"));
}

#[test]
fn binary_fails_when_the_message_file_is_missing() {
    let output = run(&["/nonexistent/COMMIT_EDITMSG"], "");

    assert_eq!(output.status.code(), Some(1));
}
