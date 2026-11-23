//! Checks a Bash command: splits it into simple commands and words,
//! honouring quotes, heredocs and comments, then checks each simple command.
//!
//! This isn't a full shell parser. It errs on the side of finding more
//! commands, not fewer: `$(…)`, backticks, `sh -c` and `eval` are checked
//! as commands of their own, up to a nesting depth of four.

use std::process::{Command, Stdio};

use super::paths::{self, Rules};
use super::pattern;
use super::{Blocked, MSG_DRIZZLE, MSG_FORCE, MSG_MAIN};

const MAX_DEPTH: usize = 4;

/// Commands that run the command after them, and the shell keywords that
/// can start one.
const WRAPPERS: [&str; 21] = [
    "sudo", "env", "nohup", "nice", "time", "timeout", "command", "builtin", "noglob", "exec",
    "xargs", "stdbuf", "if", "then", "else", "elif", "do", "while", "until", "!", "{",
];

const SHELLS: [&str; 5] = ["sh", "bash", "zsh", "dash", "ksh"];
const REDIRECTS: [&str; 5] = [">", ">&", "<", "<&", "<<<"];

pub(crate) struct Analyzer<'a> {
    /// The working directory of the current simple command; `cd` changes it.
    cwd: String,
    home: &'a str,
    rules: &'a Rules,
}

/// The words of the simple command being read.
#[derive(Default)]
struct Words {
    words: Vec<String>,
    word: String,
    started: bool,
}

impl Words {
    fn flush(&mut self) {
        if self.started {
            self.words.push(std::mem::take(&mut self.word));
        }
        self.word.clear();
        self.started = false;
    }

    fn take(&mut self) -> Vec<String> {
        std::mem::take(&mut self.words)
    }
}

impl<'a> Analyzer<'a> {
    pub(crate) fn new(cwd: String, home: &'a str, rules: &'a Rules) -> Self {
        Self { cwd, home, rules }
    }

    pub(crate) fn analyze(&mut self, command: &str, depth: usize) -> Result<(), Blocked> {
        if depth > MAX_DEPTH {
            return Ok(());
        }
        let s: Vec<char> = command.chars().collect();
        let n = s.len();
        let at = |i: usize| s.get(i).copied();
        let mut w = Words::default();
        let mut quote: Option<char> = None;
        let mut heredoc = String::new();
        let mut i = 0;
        while i < n {
            let c = s[i];
            if let Some(q) = quote {
                if c == q {
                    quote = None;
                } else if q == '"' && c == '\\' {
                    i += 1;
                    w.word.extend(at(i));
                } else {
                    w.word.push(c);
                }
                i += 1;
                continue;
            }
            match c {
                '\'' | '"' => {
                    quote = Some(c);
                    w.started = true;
                }
                '\\' => {
                    i += 1;
                    w.word.extend(at(i).filter(|&next| next != '\n'));
                    w.started = true;
                }
                ' ' | '\t' => w.flush(),
                '#' if !w.started => {
                    while i < n && at(i + 1) != Some('\n') {
                        i += 1;
                    }
                }
                '\n' => {
                    w.flush();
                    self.segment(w.take(), depth)?;
                    if !heredoc.is_empty() {
                        i = skip_heredoc(&s, i, &heredoc);
                        heredoc.clear();
                    }
                }
                ';' | '|' | '(' | ')' | '`' => {
                    w.flush();
                    self.segment(w.take(), depth)?;
                }
                '&' => {
                    w.flush();
                    if at(i + 1) != Some('>') {
                        self.segment(w.take(), depth)?;
                    }
                }
                '$' if at(i + 1) == Some('(') => {
                    w.flush();
                    self.segment(w.take(), depth)?;
                    i += 1;
                }
                '>' | '<' => {
                    // `2>` and the like: the number belongs to the redirect.
                    if !w.word.is_empty() && w.word.chars().all(|d| d.is_ascii_digit()) {
                        w.word.clear();
                        w.started = false;
                    } else {
                        w.flush();
                    }
                    if c == '<' && at(i + 1) == Some('<') {
                        if at(i + 2) == Some('<') {
                            // A here-string: the next word is data.
                            i += 2;
                            w.words.push("<<<".to_owned());
                        } else {
                            // A heredoc: remember the delimiter and skip the
                            // body at the next newline.
                            i += 2;
                            if at(i) == Some('-') {
                                i += 1;
                            }
                            while at(i) == Some(' ') {
                                i += 1;
                            }
                            heredoc.clear();
                            while i < n {
                                match s[i] {
                                    ' ' | '\t' | '\n' | ';' | '|' | '&' | ')' => break,
                                    '\'' | '"' => {}
                                    d => heredoc.push(d),
                                }
                                i += 1;
                            }
                            i -= 1;
                        }
                        i += 1;
                        continue;
                    }
                    while matches!(at(i + 1), Some('>' | '|')) {
                        i += 1;
                    }
                    if at(i + 1) == Some('&') {
                        i += 1;
                        w.words.push(format!("{c}&"));
                    } else {
                        w.words.push(c.to_string());
                    }
                }
                _ => {
                    w.word.push(c);
                    w.started = true;
                }
            }
            i += 1;
        }
        w.flush();
        self.segment(w.take(), depth)
    }

    fn segment(&mut self, words: Vec<String>, depth: usize) -> Result<(), Blocked> {
        if words.is_empty() {
            return Ok(());
        }
        self.check_segment(&words, depth)
    }

    fn check_write(&self, path: &str) -> Result<(), Blocked> {
        let absolute = paths::absolute(path, &self.cwd, self.home);
        paths::protected(&absolute, self.rules)
    }

    fn check_segment(&mut self, a: &[String], depth: usize) -> Result<(), Blocked> {
        let m = a.len();
        let arg = |j: usize| a.get(j).map_or("", String::as_str);

        // Every word: secrets, redirect targets, nested command substitutions.
        for (j, word) in a.iter().enumerate() {
            match word.as_str() {
                ">" => self.check_write(arg(j + 1))?,
                ">&" | "<&" | "<<<" | "<" => {}
                w if w.contains(['\n', ' ', '\t']) => {}
                w => check_secret(w)?,
            }
            if let Some(start) = word.find("$(") {
                self.analyze(&word[start + 2..], depth + 1)?;
            } else if let Some(start) = word.find('`') {
                self.analyze(&word[start + 1..], depth + 1)?;
            }
        }

        // Skip assignments, keywords and wrappers to find the command.
        let mut k = 0;
        while k < m {
            if is_assignment(&a[k]) {
                k += 1;
                continue;
            }
            if WRAPPERS.contains(&base_name(&a[k])) {
                k += 1;
                while k < m && (a[k].starts_with('-') || is_duration(&a[k])) {
                    k += 1;
                }
                continue;
            }
            break;
        }
        if k >= m {
            return Ok(());
        }

        match base_name(&a[k]) {
            "cd" | "pushd" => {
                let target = match arg(k + 1) {
                    "" => self.home.to_owned(),
                    target => target.to_owned(),
                };
                let dir = paths::absolute(&target, &self.cwd, self.home);
                if std::path::Path::new(&dir).is_dir() {
                    self.cwd = dir;
                }
            }
            "git" => self.check_git(a, k)?,
            shell if SHELLS.contains(&shell) => {
                if let Some(j) = (k + 1..m).find(|&j| is_command_flag(&a[j])) {
                    self.analyze(arg(j + 1), depth + 1)?;
                }
            }
            "eval" => self.analyze(&a[k + 1..].join(" "), depth + 1)?,
            "tee" | "truncate" | "rm" | "mv" => {
                for word in &a[k + 1..] {
                    if !word.starts_with('-') && !REDIRECTS.contains(&word.as_str()) {
                        self.check_write(word)?;
                    }
                }
            }
            "cp" | "install" | "ln" => self.check_write(&a[m - 1])?,
            "sed" | "perl" => {
                let in_place = a[k + 1..]
                    .iter()
                    .any(|w| w.starts_with("-i") || w.starts_with("--in-place"));
                if in_place {
                    for word in &a[k + 1..] {
                        if !word.starts_with('-') {
                            self.check_write(word)?;
                        }
                    }
                }
            }
            _ => {}
        }

        // drizzle-kit push, also through bunx, npx, bun x, pnpm dlx, ...
        let mut drizzle_kit = false;
        for word in &a[k..] {
            match base_name(word) {
                name if name == "drizzle-kit" || name.starts_with("drizzle-kit@") => {
                    drizzle_kit = true;
                }
                "push" if drizzle_kit => return Err(Blocked::Rule(MSG_DRIZZLE.to_owned())),
                _ => {}
            }
        }
        Ok(())
    }

    fn check_git(&self, a: &[String], k: usize) -> Result<(), Blocked> {
        let m = a.len();
        let mut j = k + 1;
        let mut git_dir = self.cwd.clone();
        // Global options before the subcommand.
        while j < m {
            match a[j].as_str() {
                "-C" => {
                    let dir = a.get(j + 1).map_or("", String::as_str);
                    git_dir = paths::absolute(dir, &self.cwd, self.home);
                    j += 2;
                }
                "-c" | "--git-dir" | "--work-tree" | "--namespace" | "--config-env" => j += 2,
                w if w.starts_with('-') => j += 1,
                _ => break,
            }
        }
        if a.get(j).map(String::as_str) != Some("push") {
            return Ok(());
        }

        let mut force = false;
        let mut all = false;
        let mut options = true;
        let mut positional: Vec<&str> = Vec::new();
        j += 1;
        while j < m {
            let w = a[j].as_str();
            j += 1;
            if options {
                match w {
                    "--" => {
                        options = false;
                        continue;
                    }
                    "--force" | "--force-with-lease" | "--force-if-includes" => {
                        force = true;
                        continue;
                    }
                    w if w.starts_with("--force-with-lease=") => {
                        force = true;
                        continue;
                    }
                    "--mirror" | "--all" | "--branches" => {
                        all = true;
                        continue;
                    }
                    "-o" | "--push-option" | "--repo" | "--receive-pack" | "--exec" => {
                        j += 1;
                        continue;
                    }
                    w if REDIRECTS.contains(&w) => {
                        j += 1;
                        continue;
                    }
                    w if w.starts_with("--") => continue,
                    w if w.starts_with('-') => {
                        force |= w.contains('f');
                        continue;
                    }
                    _ => {}
                }
            }
            positional.push(w);
        }

        let refspecs = positional.get(1..).unwrap_or_default();
        if force {
            return Err(Blocked::Rule(MSG_FORCE.to_owned()));
        }
        if refspecs.iter().any(|spec| spec.starts_with('+')) {
            return Err(Blocked::Rule(format!(
                "{MSG_FORCE} (a `+` refspec forces the update)"
            )));
        }
        if all {
            return Err(Blocked::Rule(format!(
                "{MSG_MAIN} (`--all`, `--branches` and `--mirror` push `main` too)"
            )));
        }
        let branch = git_output(&git_dir, &["branch", "--show-current"]);
        if refspecs.is_empty() {
            if branch == "main" {
                return Err(Blocked::Rule(format!(
                    "{MSG_MAIN} (you are on `main`, so a plain `git push` pushes it)"
                )));
            }
            let upstream = git_output(&git_dir, &["rev-parse", "--abbrev-ref", "@{push}"]);
            if pattern::matches("*/main", &upstream, false) {
                return Err(Blocked::Rule(format!(
                    "{MSG_MAIN} (this branch pushes to `{upstream}`; push with an explicit branch name)"
                )));
            }
            return Ok(());
        }
        for spec in refspecs {
            let spec = spec.strip_prefix('+').unwrap_or(spec);
            let destination = spec.rsplit(':').next().unwrap_or(spec);
            let destination = match destination {
                "HEAD" | "@" => branch.as_str(),
                other => other,
            };
            let destination = destination
                .strip_prefix("refs/heads/")
                .unwrap_or(destination);
            if destination == "main" {
                return Err(Blocked::Rule(MSG_MAIN.to_owned()));
            }
        }
        Ok(())
    }
}

/// Skips a heredoc body that starts after the newline at `i`, and returns
/// the index of the last character skipped.
fn skip_heredoc(s: &[char], i: usize, delimiter: &str) -> usize {
    let rest = &s[i + 1..];
    let delimiter: Vec<char> = delimiter.chars().collect();
    let first_line_end = rest.iter().position(|&c| c == '\n').unwrap_or(rest.len());
    if rest[..first_line_end] == delimiter[..] {
        return i + delimiter.len() + 1;
    }
    // The body ends at the first line that starts with the delimiter.
    let end = (0..rest.len()).find(|&p| rest[p] == '\n' && rest[p + 1..].starts_with(&delimiter));
    match end {
        Some(p) => i + 1 + p + 1 + delimiter.len(),
        None => s.len(),
    }
}

/// `NAME=value`
fn is_assignment(word: &str) -> bool {
    let Some((name, _)) = word.split_once('=') else {
        return false;
    };
    let mut chars = name.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// `10`, `1.5s` or `2m`, as `timeout` and `nice` take them.
fn is_duration(word: &str) -> bool {
    let number = word.strip_suffix(['s', 'm', 'h', 'd']).unwrap_or(word);
    !number.is_empty() && number.chars().all(|c| c.is_ascii_digit() || c == '.')
}

/// `-c`, `-lc`, `-ec` and the like.
fn is_command_flag(word: &str) -> bool {
    word.strip_prefix('-')
        .is_some_and(|flags| flags.contains('c') && flags.chars().all(|c| c.is_ascii_alphabetic()))
}

/// The part after the last `/`.
fn base_name(word: &str) -> &str {
    word.rsplit('/').next().unwrap_or(word)
}

fn git_output(dir: &str, args: &[&str]) -> String {
    Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .trim_end_matches('\n')
                .to_owned()
        })
        .unwrap_or_default()
}

/// Whether a shell word names a `.env` file, a key or credentials.
fn is_secret(word: &str) -> bool {
    // A regex such as '\.env', not a path.
    if word.contains('\\') {
        return false;
    }
    let target = word.rsplit('=').next().unwrap_or(word);
    let target = target.strip_prefix('@').unwrap_or(target);
    let name = base_name(target);
    let any = |patterns: &[&str], text: &str| {
        patterns
            .iter()
            .any(|pattern| pattern::matches(pattern, text, false))
    };
    if any(
        &[
            ".env.example",
            ".env.sample",
            ".env.template",
            ".env.dist",
            "*.env.example",
        ],
        name,
    ) {
        return false;
    }
    any(
        &[
            ".env*",
            "*.env",
            "*.pem",
            "*.key",
            "*.p12",
            "*.pfx",
            "id_rsa*",
            "id_ed25519*",
            "id_ecdsa*",
        ],
        name,
    ) || any(
        &[
            "secrets/*",
            "*/secrets/*",
            "*/secrets",
            ".ssh",
            ".ssh/*",
            "*/.ssh",
            "*/.ssh/*",
            "*/.aws",
            "*/.aws/*",
        ],
        target,
    )
}

fn check_secret(word: &str) -> Result<(), Blocked> {
    if is_secret(word) {
        return Err(Blocked::Rule(format!(
            "`{word}` looks like a secret (.env file, key or credentials). Agents never read, print, copy or edit secrets. Use `.env.example` for variable names, or ask the user."
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secrets_are_env_files_keys_and_credential_folders() {
        for word in [
            ".env",
            ".env.local",
            "config/prod.env",
            "--env-file=.env",
            "@.env",
            "id_rsa.pub",
            "a/secrets/x",
            "~/.ssh",
            "~/.aws/credentials",
            "tls.key",
        ] {
            assert!(is_secret(word), "{word}");
        }
        for word in [
            ".env.example",
            "prod.env.example",
            "\\.env",
            "envoy",
            "secrets",
            "README.md",
        ] {
            assert!(!is_secret(word), "{word}");
        }
    }

    #[test]
    fn assignments_and_durations_are_recognised() {
        assert!(is_assignment("FOO_1=bar"));
        assert!(!is_assignment("1FOO=bar"));
        assert!(!is_assignment("--opt=x"));
        assert!(is_duration("10"));
        assert!(is_duration("1.5s"));
        assert!(!is_duration("s"));
        assert!(!is_duration("10x"));
    }

    #[test]
    fn heredoc_bodies_are_skipped_up_to_the_delimiter() {
        let s: Vec<char> = "cat <<EOF\nbody\nEOF\nnext".chars().collect();

        let end = skip_heredoc(&s, 9, "EOF");

        assert_eq!(s[end + 1..].iter().collect::<String>(), "next");
    }
}
