//! The repository's own rules (AGENTS.md), checked on its files: the size budget (F3), a file
//! that says what it is (F4), unit tests beside the code, never inside it (F4), English only (F4),
//! no I/O (F10), and a compiled schema shared across threads (F12).

use std::path::{Path, PathBuf};

/// Lines of working code in the library, at most. Hard: when a change does not fit, code is
/// removed. It was 5,878 when the library left Localix CLI.
const LIMIT: usize = 7_500;

/// Lines of working code in one file, at most: a file is one idea, read whole.
const FILE_LIMIT: usize = 300;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// `path` as the repository names it, from its root.
fn shown(path: &Path) -> String {
    path.strip_prefix(root())
        .unwrap_or(path)
        .display()
        .to_string()
}

/// Every file under `dir` that `keep` takes, its subdirectories walked, build output, fetched
/// corpora and git's own files left out.
fn walk(dir: &Path, keep: &dyn Fn(&Path) -> bool, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = entry.path();
        let name = entry.file_name();
        if path.is_dir() {
            if !matches!(name.to_str(), Some("target" | ".git" | "data" | "out")) {
                walk(&path, keep, out);
            }
        } else if keep(&path) {
            out.push(path);
        }
    }
}

/// Every Rust source of the library.
fn sources() -> Vec<PathBuf> {
    let mut out = vec![];
    walk(
        &root().join("src"),
        &|path| path.extension().is_some_and(|ext| ext == "rs"),
        &mut out,
    );
    out
}

/// Working code: a source that is not a test beside the code.
fn working(path: &Path) -> bool {
    !path.to_string_lossy().ends_with("_tests.rs")
}

/// Lines that are code: not blank and not a comment.
fn code_lines(text: &str) -> usize {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .count()
}

#[test]
fn the_library_stays_under_its_size_limit() {
    let mut files: Vec<(usize, PathBuf)> = (sources().into_iter())
        .filter(|path| working(path))
        .map(|path| (code_lines(&std::fs::read_to_string(&path).unwrap()), path))
        .collect();
    files.sort_by_key(|file| std::cmp::Reverse(file.0));
    let total: usize = files.iter().map(|file| file.0).sum();
    let largest: Vec<String> = (files.iter().take(5))
        .map(|(lines, path)| format!("  {lines:>5}  {}", shown(path)))
        .collect();
    assert!(
        total <= LIMIT,
        "{total} lines of working code, the limit is {LIMIT}; remove code to make room:\n{}",
        largest.join("\n")
    );
    let over: Vec<String> = (files.iter())
        .filter(|(lines, _)| *lines > FILE_LIMIT)
        .map(|(lines, path)| format!("  {lines:>5}  {}", shown(path)))
        .collect();
    assert!(
        over.is_empty(),
        "files over {FILE_LIMIT} lines of working code; split each by what it does:\n{}",
        over.join("\n")
    );
}

#[test]
fn every_working_file_says_what_it_is() {
    let silent: Vec<String> = (sources().iter())
        .filter(|path| working(path))
        .filter(|path| !std::fs::read_to_string(path).unwrap().starts_with("//!"))
        .map(|path| shown(path))
        .collect();
    assert!(
        silent.is_empty(),
        "open each with a `//!` line on what it is: {silent:?}"
    );
}

/// `x.rs` ends with `#[cfg(test)] #[path = "x_tests.rs"] mod tests;` and its tests are in
/// `x_tests.rs`.
#[test]
fn tests_live_in_their_own_files() {
    let sources = sources();
    assert!(sources.len() > 20, "found only {} sources", sources.len());
    for path in &sources {
        let text = std::fs::read_to_string(path).unwrap();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if let Some(stem) = name.strip_suffix("_tests.rs") {
            let code = path.with_file_name(format!("{stem}.rs"));
            let wired = std::fs::read_to_string(&code)
                .is_ok_and(|code| code.contains(&format!("#[path = \"{name}\"]")));
            assert!(wired, "{} is not wired from {stem}.rs", shown(path));
            continue;
        }
        assert!(
            !text.contains("#[test]") && !text.contains("mod tests {"),
            "{} has tests inside: move them to its _tests.rs",
            shown(path)
        );
    }
}

#[test]
fn the_repository_is_english_only() {
    let mut files = vec![];
    walk(&root(), &|_| true, &mut files);
    assert!(files.len() > 20, "found only {} files", files.len());
    let found: Vec<String> = (files.iter())
        .filter(|path| {
            std::fs::read_to_string(path)
                .is_ok_and(|text| text.chars().any(|c| ('\u{400}'..='\u{52f}').contains(&c)))
        })
        .map(|path| shown(path))
        .collect();
    assert!(found.is_empty(), "Cyrillic in {found:?}");
}

#[test]
fn comments_and_blanks_do_not_count() {
    assert_eq!(
        code_lines("//! doc\n\nfn main() {\n    // why\n    *x = 1;\n}\n"),
        3
    );
}

/// What working code names when it does I/O, waits or reads the world around it.
const EFFECTS: [&str; 9] = [
    "std::fs",
    "std::net",
    "std::process",
    "std::thread",
    "std::env",
    "std::io",
    "Instant::now",
    "SystemTime",
    "std::time",
];

#[test]
fn working_code_does_no_io() {
    let mut found = vec![];
    for path in sources().iter().filter(|path| working(path)) {
        let text = std::fs::read_to_string(path).unwrap();
        for effect in EFFECTS {
            if text.contains(effect) {
                found.push(format!("{}: {effect}", shown(path)));
            }
        }
    }
    assert!(found.is_empty(), "I/O in working code (F10): {found:?}");
}

#[test]
fn a_schema_is_shared_across_threads() {
    fn shared<T: Send + Sync>() {}
    shared::<flowschema::Schema>();
}
