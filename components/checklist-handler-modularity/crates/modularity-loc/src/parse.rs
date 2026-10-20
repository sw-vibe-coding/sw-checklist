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

fn is_fn_def(line: &str) -> bool {
    // `fn`, `pub fn`, `pub(crate) fn`, `pub(super) fn`, `pub(in ...) fn`,
    // each with or without `async`, `const` or `unsafe` before `fn`.
    let rest = match line.strip_prefix("pub") {
        Some(r) if r.starts_with('(') => r.split_once(')').map_or("", |(_, t)| t),
        Some(r) => r,
        None => line,
    };
    let mut words = rest.split_whitespace();
    loop {
        match words.next() {
            Some("fn") => return true,
            Some("async" | "const" | "unsafe") => continue,
            _ => return false,
        }
    }
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
