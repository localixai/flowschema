//! MaskBench: about 10,000 schemas of real LLM tasks (function calling, GitHub's, Kubernetes',
//! JSON Schema Store's) with 36,990 answers labelled valid or not. Each schema through ours and
//! `jsonschema`: verdicts against the labels and each other, what is left unchecked, the time to
//! compile and to check every answer. Writes `results/corpus.{json,md}`, and beside them the
//! answers the two judge apart and the schemas ours is slowest on, as `.tsv`.

use std::collections::BTreeMap;
use std::panic::{AssertUnwindSafe, catch_unwind};

use flowschema::Schema;
use flowschema_bench::{best, counted, data, publish, reference, results, settle, thousands};
use serde_json::{Value, json};

#[derive(Default)]
struct Tally {
    /// Files with a schema; `schemas`: those both compiled.
    files: usize,
    schemas: usize,
    refused: usize,
    panicked: usize,
    unchecked: BTreeMap<&'static str, usize>,
    with_unchecked: usize,
    answers: usize,
    ours_right: usize,
    theirs_right: usize,
    agreed: usize,
    compile: (f64, f64),
    check: (f64, f64),
    timed: usize,
    slower: Vec<(f64, String)>,
    disagreements: Vec<String>,
}

fn main() {
    let dir = (std::env::args().nth(1).map(std::path::PathBuf::from))
        .unwrap_or_else(|| data("jsonschemabench/maskbench/data"));
    let mut files: Vec<_> = (std::fs::read_dir(&dir)
        .expect("run fetch.sh first")
        .flatten())
    .map(|entry| entry.path())
    .collect();
    files.sort();
    settle();
    let mut tally = Tally::default();
    for path in &files {
        let Some(file) = (std::fs::read_to_string(path).ok())
            .and_then(|text| serde_json::from_str::<Value>(&text).ok())
        else {
            continue;
        };
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        one(&mut tally, &name, &file);
    }
    report(&mut tally);
}

/// One schema and its answers.
fn one(tally: &mut Tally, name: &str, file: &Value) {
    let schema = &file["schema"];
    tally.files += 1;
    let Ok(ours) = catch_unwind(AssertUnwindSafe(|| Schema::new(schema.clone()))) else {
        tally.panicked += 1;
        return;
    };
    let Some(theirs) = reference(schema) else {
        tally.refused += 1;
        return;
    };
    tally.schemas += 1;
    let unchecked = ours.unchecked();
    tally.with_unchecked += usize::from(!unchecked.is_empty());
    for keyword in &unchecked {
        *tally.unchecked.entry(keyword).or_default() += 1;
    }
    let answers: Vec<(&Value, bool)> = (file["tests"].as_array().into_iter().flatten())
        .map(|test| (&test["data"], test["valid"].as_bool().unwrap_or(false)))
        .collect();
    for &(answer, label) in &answers {
        let (a, b) = (ours.matches(answer), theirs.is_valid(answer));
        assert_eq!(
            a,
            ours.mismatches(answer).is_empty(),
            "{name}: quick and full differ"
        );
        tally.answers += 1;
        tally.ours_right += usize::from(a == label);
        tally.theirs_right += usize::from(b == label);
        tally.agreed += usize::from(a == b);
        if a != b {
            let why = ours.mismatches(answer).first().cloned().unwrap_or_default();
            (tally.disagreements).push(format!(
                "{name}\tours={a}\tjsonschema={b}\tlabel={label}\t{why}"
            ));
        }
    }
    let copy = best(|| schema.clone());
    tally.compile.0 += best(|| Schema::new(schema.clone())) - copy;
    tally.compile.1 += best(|| reference(schema).is_some());
    // Time is compared where both check the same keywords.
    if !answers.is_empty() && unchecked.is_empty() {
        let a = best(|| {
            answers
                .iter()
                .filter(|(answer, _)| ours.matches(answer))
                .count()
        });
        let b = best(|| {
            answers
                .iter()
                .filter(|(answer, _)| theirs.is_valid(answer))
                .count()
        });
        tally.check.0 += a;
        tally.check.1 += b;
        tally.timed += 1;
        if a > b * 1.05 {
            tally.slower.push((a / b, name.to_string()));
        }
    }
}

fn report(tally: &mut Tally) {
    let share = |part: usize, all: usize| 100.0 * part as f64 / all.max(1) as f64;
    let mut markdown = String::from("| | flowschema | jsonschema 0.60 |\n|---|---|---|\n");
    markdown += &format!(
        "| schemas compiled | {} of {} ({} panics) | {} ({} refused) |\n",
        thousands(tally.files - tally.panicked),
        thousands(tally.files),
        tally.panicked,
        thousands(tally.files - tally.panicked - tally.refused),
        tally.refused
    );
    markdown += &format!(
        "| answers judged as labelled | {} of {} ({:.2}%) | {} ({:.2}%) |\n",
        thousands(tally.ours_right),
        thousands(tally.answers),
        share(tally.ours_right, tally.answers),
        thousands(tally.theirs_right),
        share(tally.theirs_right, tally.answers)
    );
    markdown += &format!(
        "| compiling every schema | {:.1} ms | {:.1} ms (flowschema {:.2}x faster) |\n",
        tally.compile.0 / 1e6,
        tally.compile.1 / 1e6,
        tally.compile.1 / tally.compile.0
    );
    markdown += &format!(
        "| checking every answer of the {} schemas both check whole | {:.2} ms | {:.2} ms \
         (flowschema {:.2}x faster) |\n",
        thousands(tally.timed),
        tally.check.0 / 1e6,
        tally.check.1 / 1e6,
        tally.check.1 / tally.check.0
    );
    markdown += &format!(
        "\nThe two agree on {:.2}% of the answers (`corpus-disagreements.tsv`). {} schemas \
         ({:.2}%) ask something not checked here: {}. flowschema is slower by over 5% on {} \
         schemas (`corpus-slower.tsv`).\n",
        share(tally.agreed, tally.answers),
        tally.with_unchecked,
        share(tally.with_unchecked, tally.schemas),
        counted(&tally.unchecked),
        tally.slower.len()
    );
    tally.slower.sort_by(|a, b| b.0.total_cmp(&a.0));
    let slower: Vec<String> = (tally.slower.iter())
        .map(|(r, n)| format!("{r:.3}\t{n}"))
        .collect();
    let _ = std::fs::create_dir_all(results(""));
    let _ = std::fs::write(results("corpus-slower.tsv"), slower.join("\n") + "\n");
    let _ = std::fs::write(
        results("corpus-disagreements.tsv"),
        tally.disagreements.join("\n") + "\n",
    );
    let json = json!({
        "files": tally.files, "schemas": tally.schemas, "refused": tally.refused, "panicked": tally.panicked,
        "withUnchecked": tally.with_unchecked, "unchecked": tally.unchecked,
        "answers": tally.answers, "oursRight": tally.ours_right,
        "jsonschemaRight": tally.theirs_right, "agreed": tally.agreed,
        "compileMs": {"ours": tally.compile.0 / 1e6, "jsonschema": tally.compile.1 / 1e6},
        "checkMs": {"ours": tally.check.0 / 1e6, "jsonschema": tally.check.1 / 1e6},
        "timed": tally.timed, "slower": tally.slower.len(),
    });
    publish("corpus", &json, &markdown);
}
