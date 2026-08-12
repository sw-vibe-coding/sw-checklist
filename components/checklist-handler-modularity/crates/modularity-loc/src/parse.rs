//! Function parsing utilities

/// Find all functions and their line counts in source code
pub fn find_functions(content: &str) -> Vec<(String, usize)> {
    let lines: Vec<&str> = content.lines().collect();
    let mut results = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        if is_fn_def(lines[i].trim()) {
            let name = extract_fn_name(lines[i].trim());
            if let Some(loc) = count_fn_lines(&lines, i) {
                results.push((name, loc));
                i += loc;
                continue;
            }
        }
        i += 1;
    }
    results
}

/// Keywords allowed between the visibility modifier and `fn`
const FN_MODIFIERS: [&str; 4] = ["const", "async", "unsafe", "extern"];

/// True if a trimmed source line starts a function definition.
///
/// Recognizes every visibility form (`fn`, `pub fn`, `pub(crate) fn`,
/// `pub(super) fn`, `pub(in path) fn`) and any order of the
/// `const`/`async`/`unsafe`/`extern "ABI"` qualifiers. Visibility must not
/// affect whether a function is measured: a 50-line `pub(crate)` function is
/// exactly as unreadable as a 50-line private one.
pub fn is_fn_def(line: &str) -> bool {
    let rest = match line.trim_start().strip_prefix("pub") {
        // Restricted visibility: skip the whole `(crate)`/`(super)`/`(in path)` group
        Some(after) if after.starts_with('(') => match after.split_once(')') {
            Some((_, tail)) => tail,
            None => return false,
        },
        Some(after) => after,
        None => line.trim_start(),
    };

    for word in rest.split_whitespace() {
        if word == "fn" {
            return true;
        }
        // `extern "C"` contributes an ABI string between the keyword and `fn`
        if !FN_MODIFIERS.contains(&word) && !word.starts_with('"') {
            return false;
        }
    }
    false
}

fn extract_fn_name(line: &str) -> String {
    let parts: Vec<&str> = line.split_whitespace().collect();
    for (i, &part) in parts.iter().enumerate() {
        if part == "fn" && i + 1 < parts.len() {
            let name = parts[i + 1];
            return name
                .split(['(', '<'])
                .next()
                .unwrap_or("unknown")
                .to_string();
        }
    }
    "unknown".to_string()
}

fn count_fn_lines(lines: &[&str], start: usize) -> Option<usize> {
    let mut brace_line = start;
    while brace_line < lines.len() && brace_line < start + 10 && !lines[brace_line].contains('{') {
        brace_line += 1;
    }
    if brace_line >= lines.len() || !lines[brace_line].contains('{') {
        return None;
    }

    let mut brace_count = 0;
    for (idx, line) in lines.iter().enumerate().skip(brace_line) {
        for ch in line.chars() {
            if ch == '{' {
                brace_count += 1;
            } else if ch == '}' {
                brace_count -= 1;
                if brace_count == 0 {
                    return Some(idx - start + 1);
                }
            }
        }
    }
    None
}
