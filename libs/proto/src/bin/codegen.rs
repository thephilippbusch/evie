//! Generates the Rust code for every `.proto` file in `definitions/`.
//!
//! Usage: `cargo run --features codegen --bin codegen -- <out_dir>`. The
//! `codegen` moon task runs it; see the crate's `AGENTS.md`.

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, bail};

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args_os().skip(1);
    let (Some(out_dir), None) = (args.next(), args.next()) else {
        bail!("usage: codegen <out_dir>");
    };
    let out_dir = PathBuf::from(out_dir);
    let definitions = Path::new(env!("CARGO_MANIFEST_DIR")).join("definitions");

    let mut files = Vec::new();
    collect_protos(&definitions, &definitions, &mut files)?;
    // Sorted, so the output doesn't depend on the order the file system lists
    // the files in.
    files.sort();

    let fds = protox::compile(&files, [&definitions]).context("failed to parse the definitions")?;

    // Start from an empty folder, so a deleted .proto file leaves no stale
    // output behind.
    if out_dir.exists() {
        fs::remove_dir_all(&out_dir)
            .with_context(|| format!("failed to clear {}", out_dir.display()))?;
    }
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("failed to create {}", out_dir.display()))?;

    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(&out_dir)
        .include_file("mod.rs")
        // This isn't a build script, so cargo doesn't read the directives.
        .emit_rerun_if_changed(false)
        .compile_fds(fds)
        .context("failed to generate the code")?;

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
