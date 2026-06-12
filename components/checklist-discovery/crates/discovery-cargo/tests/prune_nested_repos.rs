//! `find_cargo_tomls` prunes build dirs and nested git repositories
//! (submodules / vendored repos) so a conformance check grades the
//! current repo, not the third-party repos embedded in it.

use std::fs;
use std::path::{Path, PathBuf};

use discovery_cargo::find_cargo_tomls;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sw-checklist-find-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn touch(path: PathBuf) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, "").unwrap();
}

fn rel(found: &[PathBuf], root: &Path) -> Vec<String> {
    let mut v: Vec<String> = found
        .iter()
        .map(|p| p.strip_prefix(root).unwrap().to_string_lossy().into_owned())
        .collect();
    v.sort();
    v
}

#[test]
fn finds_first_party_crates_but_prunes_nested_repos_and_target() {
    let root = scratch("prune");
    // First-party crates -- found.
    touch(root.join("Cargo.toml"));
    touch(root.join("sub/Cargo.toml"));
    // A nested git repo (submodule has a `.git` file; vendored repo a
    // `.git` dir) -- the whole subtree is pruned.
    touch(root.join("vendored/.git"));
    touch(root.join("vendored/Cargo.toml"));
    touch(root.join("vendored/inner/Cargo.toml"));
    // A vendored crates.io package (has `Cargo.toml.orig`) -- pruned.
    touch(root.join("vendor/dep/Cargo.toml.orig"));
    touch(root.join("vendor/dep/Cargo.toml"));
    // A build dir -- pruned.
    touch(root.join("target/debug/Cargo.toml"));

    assert_eq!(
        rel(&find_cargo_tomls(&root), &root),
        vec!["Cargo.toml".to_string(), "sub/Cargo.toml".to_string()],
        "first-party crates only; nested .git repo, vendored package, and target/ pruned"
    );

    let _ = fs::remove_dir_all(&root);
}
