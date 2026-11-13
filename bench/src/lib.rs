//! What the benchmarks share: a counting allocator, a timer that takes the median of many rounds,
//! the reference validator built as ours is (formats asserted), the corpora's places, and where a
//! run's results are written (`results/`, committed: every figure the README gives comes from
//! there).

mod counting;
pub mod fixtures;

pub use counting::{Cost, allocations, cost_per, counters, fast_cores};

use std::hint::black_box;
use std::time::Instant;

use serde_json::Value;

/// A measured call: its median time, and how many allocations one call made.
#[derive(Clone, Copy)]
pub struct Timing {
    pub ns: f64,
    pub allocations: u64,
}

/// `f` timed: rounds of about 0.2 ms each, the median of 31, after one call that counts what it
/// allocates. The inputs are fixed, so the counts are the same on every run.
pub fn measure<T>(mut f: impl FnMut() -> T) -> Timing {
    let before = allocations();
    black_box(f());
    let allocations = allocations() - before;
    let started = Instant::now();
    let mut calls = 0u64;
    while started.elapsed().as_micros() < 200 {
        black_box(f());
        calls += 1;
    }
    let calls = calls.max(1);
    let mut rounds: Vec<f64> = (0..31)
        .map(|_| {
            let round = Instant::now();
            for _ in 0..calls {
                black_box(f());
            }
            round.elapsed().as_nanos() as f64 / calls as f64
        })
        .collect();
    rounds.sort_by(f64::total_cmp);
    Timing {
        ns: rounds[15],
        allocations,
    }
}

/// `f` timed by its best of five rounds of about a millisecond: steadier where many schemas
/// are each timed once, as in the corpus.
pub fn best<T>(mut f: impl FnMut() -> T) -> f64 {
    let started = Instant::now();
    let mut calls = 0u64;
    while started.elapsed().as_micros() < 200 {
        black_box(f());
        calls += 1;
    }
    let calls = (calls * 5).max(1);
    (0..5)
        .map(|_| {
            let round = Instant::now();
            for _ in 0..calls {
                black_box(f());
            }
            round.elapsed().as_nanos() as f64 / calls as f64
        })
        .fold(f64::INFINITY, f64::min)
}

/// The reference: `jsonschema` with `format` asserted, as this package checks it.
pub fn reference(schema: &Value) -> Option<jsonschema::Validator> {
    jsonschema::options()
        .should_validate_formats(true)
        .build(schema)
        .ok()
}

/// Where `fetch.sh` puts the corpora, beside this crate.
pub fn data(path: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join(path)
}

/// The file `name` under `results/`.
pub fn results(name: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("results")
        .join(name)
}

/// One benchmark's results: `results/<name>.json`, which `check` holds to `budgets.json`, and
/// `results/<name>.md`, the tables the documentation copies; the Markdown is printed too.
pub fn publish(name: &str, json: &Value, markdown: &str) {
    let written = std::fs::create_dir_all(results(""))
        .and_then(|()| {
            let text = serde_json::to_string_pretty(json).unwrap_or_default();
            std::fs::write(results(&format!("{name}.json")), text + "\n")
        })
        .and_then(|()| std::fs::write(results(&format!("{name}.md")), markdown));
    if let Err(why) = written {
        eprintln!("results/{name}: not written ({why})");
    }
    print!("{markdown}");
}

/// Counts by what they count, as a sentence lists them: `format (375), pattern (76)`, largest first.
pub fn counted(counts: &std::collections::BTreeMap<&str, usize>) -> String {
    let mut counts: Vec<(&&str, &usize)> = counts.iter().collect();
    counts.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    let parts: Vec<String> = counts
        .iter()
        .map(|(what, n)| format!("`{what}` ({n})"))
        .collect();
    match parts.is_empty() {
        true => "nothing".into(),
        false => parts.join(", "),
    }
}

/// A count as prose writes it: `11,306`.
pub fn thousands(n: usize) -> String {
    let digits = n.to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}

/// The machine settled before anything is timed: a second of work, so the scheduler has moved the
/// process to a fast core and its clock is up when the first measurement starts.
pub fn settle() {
    let started = Instant::now();
    let mut x = 0u64;
    while started.elapsed().as_millis() < 1_000 {
        x = black_box(x.wrapping_mul(6364136223846793005).wrapping_add(1));
    }
    black_box(x);
}

/// `text` as a cell of a Markdown table: a `|` in it would end the cell.
pub fn cell(text: &str) -> String {
    text.replace('|', "\\|")
}

/// A time as a table says it.
pub fn shown(ns: f64) -> String {
    match ns {
        n if n < 1_000.0 => format!("{n:.0} ns"),
        n if n < 1_000_000.0 => format!("{:.2} µs", n / 1e3),
        n => format!("{:.2} ms", n / 1e6),
    }
}
