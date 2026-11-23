#![allow(dead_code)] // each test crate uses a different part

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// A git repository in a temporary folder, removed on drop.
pub struct TempRepo {
    dir: tempfile::TempDir,
}

impl TempRepo {
    pub fn new() -> Self {
        let repo = Self {
            dir: tempfile::tempdir().unwrap(),
        };
        repo.git(&["init", "--quiet", "--initial-branch=main"]);
        repo
    }

    pub fn path(&self) -> &Path {
        self.dir.path()
    }

    pub fn join(&self, path: &str) -> PathBuf {
        self.dir.path().join(path)
    }

    pub fn write(&self, path: &str, content: &str) {
        let path = self.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }

    pub fn read(&self, path: &str) -> String {
        std::fs::read_to_string(self.join(path)).unwrap()
    }

    pub fn symlink(&self, target: &str, link: &str) {
        let link = self.join(link);
        std::fs::create_dir_all(link.parent().unwrap()).unwrap();
        std::os::unix::fs::symlink(target, link).unwrap();
    }

    /// Runs git in the repository, without the user's hooks or identity, and
    /// panics when it fails.
    pub fn git(&self, args: &[&str]) -> String {
        let output = self.try_git(args);
        assert!(
            output.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap()
    }

    pub fn try_git(&self, args: &[&str]) -> Output {
        Command::new("git")
            .args([
                "-c",
                "core.hooksPath=/dev/null",
                "-c",
                "user.name=evie-dev tests",
                "-c",
                "user.email=evie-dev@example.com",
                "-c",
                "commit.gpgSign=false",
            ])
            .args(args)
            .current_dir(self.path())
            .output()
            .unwrap()
    }

    /// Runs the evie-dev binary in the repository.
    pub fn evie_dev(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_evie-dev"))
            .args(args)
            .current_dir(self.path())
            .output()
            .unwrap()
    }
}
