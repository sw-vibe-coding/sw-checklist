//! Find Cargo.toml files in a project

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Find all Cargo.toml files in a directory tree.
///
/// Prunes (subtree-skips) any directory other than the scan root that is
/// not first-party source: a build dir (`target/`), a nested git repo
/// (`.git` -- a submodule or embedded repo), or a vendored crates.io
/// package (`Cargo.toml.orig` -- the upstream pre-normalization manifest
/// that `cargo vendor` / crates.io packaging leaves behind). A conformance
/// check grades the current repo, not the third-party code embedded in it.
pub fn find_cargo_tomls(path: &Path) -> Vec<PathBuf> {
    let root = path.to_path_buf();
    WalkDir::new(path)
        .into_iter()
        .filter_entry(move |e| {
            e.path() == root
                || !e.file_type().is_dir()
                || !(e.file_name() == "target"
                    || e.file_name() == ".git"
                    || e.path().join(".git").exists()
                    || e.path().join("Cargo.toml.orig").exists())
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() == "Cargo.toml")
        .map(|e| e.path().to_path_buf())
        .collect()
}
