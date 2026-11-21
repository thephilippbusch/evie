//! Generates the Rust and TypeScript code of `libs/proto` from the `.proto`
//! files in its `definitions/` folder.
//!
//! Usage, from `libs/proto`: `cargo run -p proto-codegen -- <root>`. It writes
//! `<root>/gen/rust` and `<root>/gen/ts`. The `codegen` task passes `.`, and
//! `scripts/check-codegen.sh` a temporary folder to compare with `gen/`.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, bail, ensure};

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args_os().skip(1);
    let (Some(root), None) = (args.next(), args.next()) else {
        bail!("usage: proto-codegen <root>");
    };
    let root = PathBuf::from(root);
    ensure!(
        Path::new("buf.gen.yaml").is_file(),
        "run proto-codegen from libs/proto, where buf.gen.yaml is"
    );

    generate_rust(Path::new("definitions"), &root.join("gen/rust"))?;
    generate_ts(&root)
}

/// Writes the messages and the tonic server and client of every service into
/// `out_dir`, one file per package, with `mod.rs` nesting them.
fn generate_rust(definitions: &Path, out_dir: &Path) -> anyhow::Result<()> {
    let mut files = Vec::new();
    collect_protos(definitions, definitions, &mut files)?;
    // Sorted, so the output doesn't depend on the order the file system lists
    // the files in.
    files.sort();

    let fds = protox::compile(&files, [definitions]).context("failed to parse the definitions")?;

    // Start from an empty folder, so a deleted .proto file leaves no stale
    // output behind.
    if out_dir.exists() {
        fs::remove_dir_all(out_dir)
            .with_context(|| format!("failed to clear {}", out_dir.display()))?;
    }
    fs::create_dir_all(out_dir)
        .with_context(|| format!("failed to create {}", out_dir.display()))?;

    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(out_dir)
        .include_file("mod.rs")
        // This isn't a build script, so cargo doesn't read the directives.
        .emit_rerun_if_changed(false)
        .compile_fds(fds)
        .context("failed to generate the Rust code")
}

/// Runs `buf generate`, which writes the TypeScript code below `root` as
/// `buf.gen.yaml` says and clears its output folder first. buf comes from the
/// package's dev dependencies, which moon puts on the `PATH`.
fn generate_ts(root: &Path) -> anyhow::Result<()> {
    let status = Command::new("buf")
        .arg("generate")
        .arg("--output")
        .arg(root)
        .status()
        .context("failed to run buf; run `bun install` at the repository root")?;
    ensure!(status.success(), "buf generate failed with {status}");
    Ok(())
}

/// Adds every `.proto` file below `dir` to `files`, as a path relative to
/// `root`, the include folder, which is what protox expects.
fn collect_protos(root: &Path, dir: &Path, files: &mut Vec<PathBuf>) -> anyhow::Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("failed to read {}", dir.display()))? {
        let path = entry?.path();
        if path.is_dir() {
            collect_protos(root, &path, files)?;
        } else if path.extension().is_some_and(|ext| ext == "proto") {
            files.push(path.strip_prefix(root)?.to_path_buf());
        }
    }
    Ok(())
}
