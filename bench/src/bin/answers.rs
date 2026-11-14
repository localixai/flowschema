//! Each answer of MaskBench on both validators, counted in instructions: where the totals of
//! `corpus` say who is faster over all of them, this says on how many answers and schemas
//! each does less work. Counted, not timed, so the same on every run of one build (on macOS;
//! elsewhere the counters are a clock's). Writes `results/answers.{json,md}`.

use std::collections::BTreeMap;

use flowschema::Schema;
use flowschema_bench::{cost_per, data, fast_cores, publish, reference, thousands};
use serde_json::{Value, json};

/// An answer one validator runs over 5% more instructions on than the other.
const MORE: f64 = 1.05;

#[derive(Default)]
struct Tally {
    answers: usize,
    ours: f64,
    theirs: f64,
    /// Answers flowschema runs more instructions on, and those `jsonschema` does.
    more: usize,
    fewer: usize,
}

impl Tally {
    fn add(&mut self, ours: f64, theirs: f64) {
        self.answers += 1;
        self.ours += ours;
        self.theirs += theirs;
        self.more += usize::from(ours > theirs * MORE);
        self.fewer += usize::from(theirs > ours * MORE);
    }

    fn row(&self, name: &str) -> String {
        let share = |n: usize| 100.0 * n as f64 / self.answers.max(1) as f64;
        format!(
            "| {name} | {} | {:.2}x | {} ({:.1}%) | {} ({:.1}%) |",
            thousands(self.answers),
            self.theirs / self.ours.max(1.0),
            thousands(self.fewer),
            share(self.fewer),
            thousands(self.more),
            share(self.more),
        )
    }

    fn json(&self) -> Value {
        json!({"answers": self.answers, "ours": self.ours, "jsonschema": self.theirs,
            "more": self.more, "fewer": self.fewer})
    }
}

fn main() {
    fast_cores();
    let dir = data("jsonschemabench/maskbench/data");
    let mut files: Vec<_> = (std::fs::read_dir(&dir).expect("run fetch.sh first"))
        .flatten()
        .map(|entry| entry.path())
        .collect();
    files.sort();
    let (mut valid, mut invalid) = (Tally::default(), Tally::default());
    let mut schemas: BTreeMap<String, (f64, f64)> = BTreeMap::new();
    for path in &files {
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        let Ok(file) = serde_json::from_str::<Value>(&text) else {
            continue;
        };
        let schema = &file["schema"];
        let Ok(ours) = std::panic::catch_unwind(|| Schema::new(schema)) else {
            continue;
        };
        let Some(theirs) = reference(schema) else {
            continue;
        };
        // Counted where both check the same keywords, as `corpus` times them.
        if !ours.unchecked().is_empty() {
            continue;
        }
        let name = path.file_name().map(|n| n.to_string_lossy().to_string());
        let mut of_schema = (0.0, 0.0);
        for test in file["tests"].as_array().into_iter().flatten() {
            let answer = &test["data"];
            let a = cost_per(200, 2, || ours.matches(answer)).instructions;
            let b = cost_per(200, 2, || theirs.is_valid(answer)).instructions;
            match ours.matches(answer) {
                true => valid.add(a, b),
                false => invalid.add(a, b),
            }
            of_schema = (of_schema.0 + a, of_schema.1 + b);
        }
        // A schema with no answer is in neither count.
        if of_schema.1 > 0.0 {
            schemas.insert(name.unwrap_or_default(), of_schema);
        }
    }
    let more = schemas.values().filter(|(a, b)| a > &(b * MORE)).count();
    let fewer = schemas.values().filter(|(a, b)| b > &(a * MORE)).count();
    let share = |n: usize| 100.0 * n as f64 / schemas.len().max(1) as f64;
    let markdown = format!(
        "| answers of MaskBench | answers | instructions, jsonschema's over flowschema's | \
         flowschema runs over 5% fewer on | over 5% more on |\n|---|---|---|---|---|\n{}\n{}\n\n\
         Of the {} schemas both check whole that have answers, flowschema runs over 5% fewer instructions on the \
         answers of {} ({:.1}%) and over 5% more on those of {} ({:.1}%). An answer is one \
         `matches` and one `is_valid`, counted in the processor's instructions: the same on \
         every run of one build, and not a time.\n",
        valid.row("that match"),
        invalid.row("that do not"),
        thousands(schemas.len()),
        thousands(fewer),
        share(fewer),
        thousands(more),
        share(more),
    );
    print!("{markdown}");
    let json = json!({"valid": valid.json(), "invalid": invalid.json(),
        "schemas": {"all": schemas.len(), "more": more, "fewer": fewer}});
    publish("answers", &json, &markdown);
}
