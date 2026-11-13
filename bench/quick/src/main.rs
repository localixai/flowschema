//! The quick loop: what one change to the library did, in seconds, and the same on every run.
//! Every check `micro` times, the compiles it times, and a fixed sample of MaskBench (every
//! eighth file of each source, in name order), split into the schemas agents send (function
//! calling, MCP, handwritten and synthesized ones) and documents' (GitHub, Kubernetes, JSON Schema
//! Store and the rest). Each is measured in instructions retired, the same on every run, in CPU
//! cycles, the fewest of several rounds of a fixed number of calls on the fast cores, and in
//! allocations, counted exactly. A fingerprint of every verdict and message says whether what
//! the library answers changed.
//!
//! ```sh
//! cargo run --release                # in bench/quick: this tree against the run saved last
//! cargo run --release -- --save      # and keep this run as the one to compare with
//! cargo run --release -- --save-as start      # keep it under a name of its own
//! cargo run --release -- --against start      # compare with that one instead
//! ```
//!
//! The saved runs are `target/quick-<name>.json` (`base` by default): the machine's own, never
//! committed. Exit 2 when the
//! fingerprint differs from the saved one. `--spin <compile|check|messages> [agents|documents]`
//! runs one part over and over for a profiler (`profile.sh`).

use std::collections::BTreeMap;
use std::hint::black_box;
use std::time::Instant;

use flowschema::Schema;
// The bench crate's counters and fixtures, as modules of this crate: not all of either is used here.
#[allow(dead_code)]
#[path = "../../src/counting.rs"]
mod counting;
#[allow(dead_code)]
#[path = "../../src/fixtures.rs"]
mod fixtures;

use counting::{Cost, allocations, cost_per, fast_cores};
use serde_json::{Value, json};

/// Where `fetch.sh` puts the corpora, beside the bench crate.
fn data(path: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../data")
        .join(path)
}

/// The sources of MaskBench whose schemas are what an agent sends: tools and answers.
const AGENTS: [&str; 5] = [
    "Glaiveai2K",
    "BFCL",
    "MCPspec",
    "Handwritten",
    "Synthesized",
];
/// One file in this many of each source is in the sample.
const EVERY: usize = 8;

struct Sampled {
    schema: Value,
    answers: Vec<Value>,
}

/// A file's source: its name before `---`, or before a trailing `_<number>`.
fn source(file: &str) -> &str {
    let stem = file.strip_suffix(".json").unwrap_or(file);
    let stem = stem.split("---").next().unwrap_or(stem);
    match stem.rsplit_once('_') {
        Some((head, tail)) if tail.bytes().all(|b| b.is_ascii_digit()) => head,
        Some(_) | None => stem,
    }
}

/// The sample, as (agents', documents'): every `EVERY`th file of each source in name order.
fn sample() -> (Vec<Sampled>, Vec<Sampled>) {
    let dir = data("jsonschemabench/maskbench/data");
    let mut names: Vec<String> = (std::fs::read_dir(&dir)
        .expect("run fetch.sh first")
        .flatten())
    .map(|entry| entry.file_name().to_string_lossy().into_owned())
    .collect();
    names.sort();
    let mut by_source: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for name in names {
        by_source
            .entry(source(&name).to_string())
            .or_default()
            .push(name);
    }
    let (mut agents, mut documents) = (vec![], vec![]);
    for (source, files) in by_source {
        let agent = AGENTS.iter().any(|a| source.starts_with(a));
        for file in files.iter().step_by(EVERY) {
            let text = std::fs::read_to_string(dir.join(file)).expect("a corpus file");
            let Ok(document) = serde_json::from_str::<Value>(&text) else {
                continue;
            };
            let answers = (document["tests"].as_array().into_iter().flatten())
                .map(|test| test["data"].clone())
                .collect();
            let sampled = Sampled {
                schema: document["schema"].clone(),
                answers,
            };
            match agent {
                true => agents.push(sampled),
                false => documents.push(sampled),
            }
        }
    }
    (agents, documents)
}

/// FNV-1a over everything the library answered: verdicts, messages, what it left unchecked.
struct Fingerprint(u64);

impl Fingerprint {
    fn add(&mut self, text: &str) {
        for byte in text.bytes().chain(std::iter::once(0)) {
            self.0 = (self.0 ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3);
        }
    }

    fn answer(&mut self, schema: &Schema, data: &Value) {
        let mismatches = schema.mismatches(data);
        self.add(if schema.matches(data) { "ok" } else { "no" });
        mismatches.iter().for_each(|m| self.add(m));
    }
}

/// One measured row: the cost of a call (or of a pass over a set) and its allocations.
struct Row {
    name: String,
    cost: Cost,
    allocations: u64,
}

impl Row {
    fn get(&self, counter: &str) -> f64 {
        match counter {
            "instructions" => self.cost.instructions,
            _ => self.cost.cycles,
        }
    }
}

/// The allocations one call of `f` makes.
fn allocated<T>(f: impl FnOnce() -> T) -> u64 {
    let before = allocations();
    black_box(f());
    allocations() - before
}

/// How many JSON values `value` holds: a large datum is checked in fewer calls a round.
fn size(value: &Value) -> usize {
    1 + match value {
        Value::Array(items) => items.iter().map(size).sum(),
        Value::Object(fields) => fields.values().map(size).sum(),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => 0,
    }
}

fn checks(rows: &mut Vec<Row>, print: &mut Fingerprint) {
    for (name, schema, data) in fixtures::checks() {
        let compiled = Schema::new(schema);
        print.answer(&compiled, &data);
        let calls = if size(&data) > 100 { 64 } else { 20_000 };
        rows.push(Row {
            name: format!("check    {name}"),
            cost: cost_per(calls, 11, || compiled.matches(&data)),
            allocations: allocated(|| compiled.matches(&data)),
        });
        if !compiled.matches(&data) {
            rows.push(Row {
                name: format!("messages {name}"),
                cost: cost_per(calls.min(2_000), 11, || compiled.mismatches(&data)),
                allocations: allocated(|| compiled.mismatches(&data)),
            });
        }
    }
}

fn compiles(rows: &mut Vec<Row>) {
    for (name, schema) in fixtures::compiled() {
        let copy = cost_per(500, 11, || schema.clone());
        let copied = allocated(|| schema.clone());
        rows.push(Row {
            name: format!("compile  {name}"),
            cost: cost_per(500, 11, || Schema::new(schema.clone())) - copy,
            allocations: allocated(|| Schema::new(schema.clone())) - copied,
        });
    }
}

fn corpus(rows: &mut Vec<Row>, print: &mut Fingerprint, label: &str, set: &[Sampled]) {
    let compile_all = || {
        for one in set {
            black_box(Schema::new(one.schema.clone()));
        }
    };
    let copy_all = || {
        for one in set {
            black_box(one.schema.clone());
        }
    };
    rows.push(Row {
        name: format!("{label} compile {} schemas", set.len()),
        cost: cost_per(1, 5, compile_all) - cost_per(1, 5, copy_all),
        allocations: allocated(compile_all) - allocated(copy_all),
    });
    let compiled: Vec<(Schema, &Vec<Value>)> = (set.iter())
        .map(|one| (Schema::new(one.schema.clone()), &one.answers))
        .collect();
    let mut answers = 0;
    let mut invalid: Vec<(&Schema, &Value)> = vec![];
    for (schema, list) in &compiled {
        schema.unchecked().iter().for_each(|note| print.add(note));
        for answer in *list {
            answers += 1;
            print.answer(schema, answer);
            if !schema.matches(answer) {
                invalid.push((schema, answer));
            }
        }
    }
    let check_all = || {
        (compiled.iter())
            .map(|(schema, list)| list.iter().filter(|a| schema.matches(a)).count())
            .sum::<usize>()
    };
    rows.push(Row {
        name: format!("{label} check {answers} answers"),
        cost: cost_per(1, 9, check_all),
        allocations: allocated(check_all),
    });
    let say_all = || {
        (invalid.iter())
            .map(|(schema, answer)| schema.mismatches(answer).len())
            .sum::<usize>()
    };
    rows.push(Row {
        name: format!("{label} messages {} invalid", invalid.len()),
        cost: cost_per(1, 9, say_all),
        allocations: allocated(say_all),
    });
}

/// A saved run: `target/quick-<name>.json`, `base` the one a run is compared with.
fn saved_path(name: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("target/quick-{name}.json"))
}

/// The value after `flag` on the command line.
fn option(flag: &str) -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    let at = args.iter().position(|arg| arg == flag)?;
    args.get(at + 1).cloned()
}

/// The geometric mean of the ratios now/saved, of `counter`, over the rows whose name starts with
/// `prefix`.
fn mean_change(rows: &[Row], saved: &Value, prefix: &str, counter: &str) -> Option<f64> {
    let ratios: Vec<f64> = (rows.iter())
        .filter(|row| row.name.starts_with(prefix))
        .filter_map(|row| Some(row.get(counter) / saved["rows"][&row.name][counter].as_f64()?))
        .filter(|ratio| ratio.is_finite() && *ratio > 0.0)
        .collect();
    (!ratios.is_empty())
        .then(|| (ratios.iter().map(|r| r.ln()).sum::<f64>() / ratios.len() as f64).exp() - 1.0)
}

/// `--spin <what> [agents|documents]`: one part of the loop, again and again for `seconds`, for
/// a profiler to watch (`profile.sh`).
fn spin(what: &str, set: &[Sampled], seconds: u64) {
    // A fixture of `micro` by its name (`--spin fixture "review verdict, valid"`): its check alone.
    if what == "fixture" {
        let wanted = option("--fixture").unwrap_or_default();
        let found = (fixtures::checks().into_iter()).find(|(name, _, _)| *name == wanted);
        let Some((_, schema, data)) = found else {
            eprintln!("no fixture {wanted:?}");
            return;
        };
        let schema = Schema::new(schema);
        let until = Instant::now() + std::time::Duration::from_secs(seconds);
        while Instant::now() < until {
            for _ in 0..1_000 {
                black_box(schema.matches(black_box(&data)));
            }
        }
        return;
    }
    let compiled: Vec<(Schema, &Vec<Value>)> = (set.iter())
        .map(|one| (Schema::new(one.schema.clone()), &one.answers))
        .collect();
    let until = Instant::now() + std::time::Duration::from_secs(seconds);
    while Instant::now() < until {
        for (schema, answers) in &compiled {
            match what {
                "compile" => drop(black_box(Schema::new(schema.json().clone()))),
                "check" => answers.iter().for_each(|a| {
                    black_box(schema.matches(a));
                }),
                _ => answers
                    .iter()
                    .for_each(|a| drop(black_box(schema.mismatches(a)))),
            }
        }
    }
}

fn main() {
    let started = Instant::now();
    let save = std::env::args().any(|arg| arg == "--save" || arg == "--save-as");
    fast_cores();
    let (agents, documents) = sample();
    let args: Vec<String> = std::env::args().collect();
    if let Some(at) = args.iter().position(|arg| arg == "--spin") {
        let what = args.get(at + 1).map_or("compile", String::as_str);
        let set = match args.get(at + 2).map(String::as_str) {
            Some("documents") => &documents,
            Some(_) | None => &agents,
        };
        return spin(what, set, 6);
    }
    let mut rows = vec![];
    let mut print = Fingerprint(0xcbf2_9ce4_8422_2325);
    checks(&mut rows, &mut print);
    compiles(&mut rows);
    corpus(&mut rows, &mut print, "agents   ", &agents);
    corpus(&mut rows, &mut print, "documents", &documents);
    let fingerprint = format!("{:016x}", print.0);
    let against = option("--against").unwrap_or_else(|| "base".into());
    let saved: Value = (std::fs::read_to_string(saved_path(&against)).ok())
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or(Value::Null);
    println!(
        "{:<52} {:>14} {:>8} {:>14} {:>8} {:>7}",
        "", "instructions", "", "cycles", "", "allocs"
    );
    let change = |row: &Row, counter: &str| {
        (saved["rows"][&row.name][counter].as_f64()).map_or(String::new(), |b| {
            format!("{:+.2}%", (row.get(counter) / b - 1.0) * 100.0)
        })
    };
    for row in &rows {
        let allocs = match saved["rows"][&row.name]["allocations"].as_u64() {
            Some(b) if b != row.allocations => format!("{} (was {b})", row.allocations),
            Some(_) | None => row.allocations.to_string(),
        };
        let name: String = row.name.chars().take(52).collect();
        println!(
            "{name:<52} {:>14.1} {:>8} {:>14.1} {:>8} {allocs:>7}",
            row.cost.instructions,
            change(row, "instructions"),
            row.cost.cycles,
            change(row, "cycles"),
        );
    }
    for counter in ["instructions", "cycles"] {
        let means: Vec<String> = [
            ("checks", "check "),
            ("messages", "messages"),
            ("compiles", "compile "),
            ("agents", "agents"),
            ("documents", "documents"),
        ]
        .iter()
        .filter_map(|(label, prefix)| {
            let change = mean_change(&rows, &saved, prefix, counter)?;
            Some(format!("{label} {:+.2}%", change * 100.0))
        })
        .collect();
        if !means.is_empty() {
            println!(
                "{counter:>12} against {against} (geometric mean): {}",
                means.join(", ")
            );
        }
    }
    let was = saved["fingerprint"].as_str();
    let same = was.is_none_or(|was| was == fingerprint);
    println!(
        "fingerprint {fingerprint}: {}",
        match was {
            None => "no run saved yet".to_string(),
            Some(_) if same => "the same verdicts and messages as the saved run".to_string(),
            Some(was) => format!("CHANGED from {was}: the library answers differently"),
        }
    );
    println!("{:.1} s", started.elapsed().as_secs_f64());
    if save {
        let json = json!({
            "fingerprint": fingerprint,
            "rows": rows.iter().map(|row| (row.name.clone(), json!({
                "instructions": row.cost.instructions,
                "cycles": row.cost.cycles,
                "allocations": row.allocations,
            }))).collect::<serde_json::Map<String, Value>>(),
        });
        let text = serde_json::to_string_pretty(&json).unwrap_or_default();
        let path = saved_path(&option("--save-as").unwrap_or_else(|| "base".into()));
        match std::fs::write(&path, text + "\n") {
            Ok(()) => println!("saved: {}", path.display()),
            Err(why) => eprintln!("not saved: {why}"),
        }
    }
    if !same {
        std::process::exit(2);
    }
}
