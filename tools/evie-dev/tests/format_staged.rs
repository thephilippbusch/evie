mod common;

use std::ffi::OsStr;
use std::process::{Command, Output};

use common::TempRepo;
use evie_dev::format_staged::{FormatStagedError, run};

const UNFORMATTED: &str = "{\"a\":1}";
const FORMATTED: &str = "{ \"a\": 1 }\n";
const DPRINT_CONFIG: &str =
    "{ \"json\": {}, \"plugins\": [\"https://plugins.dprint.dev/json-0.26.0.wasm\"] }\n";

/// A repository with a dprint config for JSON, the plugin the root
/// `dprint.json` uses too, so it comes from dprint's cache.
fn repo() -> TempRepo {
    let repo = TempRepo::new();
    repo.write("dprint.json", DPRINT_CONFIG);
    repo.git(&["add", "dprint.json"]);
    repo.git(&["commit", "--quiet", "-m", "chore: init"]);
    repo
}

/// The dprint version from the root `.prototools`. proto ignores the
/// untrusted `.prototools` of a temporary repository, so a proto shim for
/// dprint gets the version from the environment.
fn dprint_version() -> String {
    let prototools =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../.prototools")).unwrap();
    prototools
        .lines()
        .find_map(|line| line.strip_prefix("dprint = "))
        .unwrap()
        .trim_matches('"')
        .to_owned()
}

/// Runs `evie-dev format-staged` in the repository, with dprint on the `PATH`.
fn format_staged(repo: &TempRepo) -> Output {
    Command::new(env!("CARGO_BIN_EXE_evie-dev"))
        .arg("format-staged")
        .current_dir(repo.path())
        .env("PROTO_DPRINT_VERSION", dprint_version())
        .output()
        .unwrap()
}

fn staged(repo: &TempRepo, path: &str) -> String {
    repo.git(&["show", &format!(":{path}")])
}

#[test]
fn does_nothing_without_staged_files() {
    let repo = repo();
    repo.write("a.json", UNFORMATTED);

    run(repo.path(), OsStr::new("no-such-dprint")).unwrap();

    assert_eq!(repo.read("a.json"), UNFORMATTED);
}

#[test]
fn fails_when_dprint_is_missing() {
    let repo = repo();
    repo.write("a.json", UNFORMATTED);
    repo.git(&["add", "a.json"]);

    let err = run(repo.path(), OsStr::new("no-such-dprint")).unwrap_err();

    assert!(matches!(err, FormatStagedError::DprintMissing));
}

#[test]
fn formats_and_stages_fully_staged_files() {
    let repo = repo();
    repo.write("a.json", UNFORMATTED);
    repo.write("dir with space/ä.json", UNFORMATTED);
    repo.git(&["add", "a.json", "dir with space/ä.json"]);

    let output = format_staged(&repo);

    assert_eq!(output.status.code(), Some(0), "{output:?}");
    for path in ["a.json", "dir with space/ä.json"] {
        assert_eq!(repo.read(path), FORMATTED, "{path}");
        assert_eq!(staged(&repo, path), FORMATTED, "{path}");
    }
}

#[test]
fn only_checks_partly_staged_files_and_fails_when_they_need_formatting() {
    let repo = repo();
    repo.write("a.json", UNFORMATTED);
    repo.git(&["add", "a.json"]);
    repo.write("a.json", "{\"a\":2}");

    let output = format_staged(&repo);

    assert_eq!(output.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("These files are partly staged, so they were checked but not formatted.")
    );
    assert_eq!(repo.read("a.json"), "{\"a\":2}");
    assert_eq!(staged(&repo, "a.json"), UNFORMATTED);
}

#[test]
fn passes_partly_staged_files_that_are_formatted() {
    let repo = repo();
    repo.write("a.json", FORMATTED);
    repo.git(&["add", "a.json"]);
    repo.write("a.json", "{ \"a\": 2 }\n");

    let output = format_staged(&repo);

    assert_eq!(output.status.code(), Some(0), "{output:?}");
}

#[test]
fn skips_files_dprint_does_not_format_and_deleted_files() {
    let repo = repo();
    repo.write("main.rs", "fn  main(){}");
    repo.git(&["add", "main.rs"]);
    repo.git(&["rm", "--quiet", "dprint.json"]);
    repo.write("dprint.json", DPRINT_CONFIG);

    let output = format_staged(&repo);

    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(staged(&repo, "main.rs"), "fn  main(){}");
}

#[test]
fn passes_on_the_exit_code_of_a_failing_git() {
    let dir = tempfile::tempdir().unwrap();
    let ceiling = dir.path().parent().unwrap();
    let git = Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .current_dir(dir.path())
        .env("GIT_CEILING_DIRECTORIES", ceiling)
        .output()
        .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_evie-dev"))
        .arg("format-staged")
        .current_dir(dir.path())
        .env("GIT_CEILING_DIRECTORIES", ceiling)
        .output()
        .unwrap();

    assert!(!git.status.success());
    assert_eq!(output.status.code(), git.status.code());
}
