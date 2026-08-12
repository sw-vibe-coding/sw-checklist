//! Function detection must not depend on visibility.
//!
//! Regression test: `pub(crate) fn`, `pub(super) fn` and `pub(in path) fn`
//! were invisible to the detector, so restricted-visibility functions escaped
//! both the function-LOC limit and the module function count. That turned a
//! parsing gap into an exemption that rewarded folding helpers into one large
//! function, the opposite of the readability goal.

use modularity_loc::{find_functions, is_fn_def};

fn long_body(name_line: &str) -> String {
    let body: String = (0..40).map(|i| format!("    let x{i} = {i};\n")).collect();
    format!("{name_line}\n{body}}}\n")
}

#[test]
fn every_visibility_form_is_a_function() {
    for line in [
        "fn private_fn() {",
        "pub fn public_fn() {",
        "pub(crate) fn crate_fn() {",
        "pub(super) fn super_fn() {",
        "pub(in crate::checks) fn scoped_fn() {",
        "pub(crate)fn no_space_fn() {",
    ] {
        assert!(is_fn_def(line), "not detected: {line}");
    }
}

#[test]
fn qualifiers_combine_with_visibility() {
    for line in [
        "async fn a() {",
        "pub async fn b() {",
        "pub(crate) async fn c() {",
        "const fn d() {",
        "pub(crate) const fn e() {",
        "pub unsafe fn f() {",
        "pub(super) const unsafe fn g() {",
        "pub extern \"C\" fn h() {",
        "pub(crate) unsafe extern \"C\" fn i() {",
    ] {
        assert!(is_fn_def(line), "not detected: {line}");
    }
}

#[test]
fn non_functions_are_not_detected() {
    for line in [
        "pub struct Foo {",
        "pub(crate) struct Bar {",
        "pub const MAX: usize = 5;",
        "pub unsafe trait Unsafe {}",
        "pub use crate::parse::is_fn_def;",
        "pub type Handler = fn(u32) -> u32;",
        "// fn commented_out() {",
        "/// fn in_a_doc_comment() {",
        "publish_all(&items);",
        "let f = |x| x + 1;",
    ] {
        assert!(!is_fn_def(line), "false positive: {line}");
    }
}

#[test]
fn restricted_visibility_functions_are_measured() {
    for vis in ["", "pub ", "pub(crate) ", "pub(super) ", "pub(in crate) "] {
        let content = long_body(&format!("{vis}fn oversized() {{"));
        let found = find_functions(&content);
        assert_eq!(
            found,
            vec![("oversized".to_string(), 42)],
            "visibility {vis:?} was measured as {found:?}"
        );
    }
}
