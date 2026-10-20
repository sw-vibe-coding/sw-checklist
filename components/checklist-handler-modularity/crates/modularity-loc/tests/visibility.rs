//! Every function is measured whatever its visibility: `pub(crate) fn`
//! was invisible to the Function LOC rule, so a 61-line helper passed
//! here and failed on a build that counts it.

use modularity_loc::check_function_locs;

/// A function of `lines` body lines with the head `head`.
fn function(head: &str, lines: usize) -> String {
    let body: String = (0..lines)
        .map(|i| format!("    let _x{i} = {i};\n"))
        .collect();
    format!("{head} {{\n{body}}}\n")
}

fn fails(src: &str) -> Vec<String> {
    let dir = std::env::temp_dir().join(format!(
        "modularity-loc-{}-{}",
        std::process::id(),
        src.len()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("lib.rs"), src).unwrap();
    let results = check_function_locs(&dir, "t").unwrap();
    std::fs::remove_dir_all(&dir).unwrap();
    results
        .iter()
        .filter(|r| !r.status.passed())
        .map(|r| r.message.clone())
        .collect()
}

#[test]
fn restricted_visibility_and_modifiers_are_measured() {
    for head in [
        "pub(crate) fn a()",
        "pub(super) fn a()",
        "pub(in crate::x) fn a()",
        "pub(crate) async fn a()",
        "pub const fn a()",
        "unsafe fn a()",
        "pub fn a()",
        "fn a()",
    ] {
        let got = fails(&function(head, 60));
        assert_eq!(got.len(), 1, "{head}: {got:?}");
        assert!(got[0].contains("62 lines"), "{head}: {}", got[0]);
    }
}

#[test]
fn other_lines_are_not_functions() {
    for line in [
        "pub(crate) struct A {",
        "pub fnord := 1;",
        "// fn not(code)",
        "pub(crate) use x::fn_like;",
    ] {
        assert!(fails(&format!("{line}\n}}\n")).is_empty(), "{line}");
    }
}
