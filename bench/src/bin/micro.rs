//! Typical checks of an LLM's structured answer, each against `jsonschema`: fixed schemas and
//! data, the median of 31 rounds, exact allocation counts. Writes `results/micro.{json,md}`.

use flowschema::Schema;
use flowschema_bench::fixtures::{self, checks, issues, verdict};
use flowschema_bench::{cell, measure, publish, reference, settle, shown};
use serde_json::{Value, json};

/// The run's rows, as the table and the JSON both say them.
#[derive(Default)]
struct Run {
    markdown: String,
    checks: Vec<Value>,
}

impl Run {
    /// One row: what the check is, ours and the reference timed, and how far apart they are.
    fn row(&mut self, name: &str, schema: &Value, data: &Value) {
        let ours = Schema::new(schema.clone());
        let Some(theirs) = reference(schema) else {
            self.markdown += &format!(
                "| {} | – | – | jsonschema refused the schema |\n",
                cell(name)
            );
            return;
        };
        let valid = ours.matches(data);
        assert_eq!(valid, theirs.is_valid(data), "{name}: the verdicts differ");
        let a = measure(|| ours.matches(data));
        let b = measure(|| theirs.is_valid(data));
        self.markdown += &format!(
            "| {} | {} ({} alloc) | {} ({} alloc) | {:.2}x |\n",
            cell(name),
            shown(a.ns),
            a.allocations,
            shown(b.ns),
            b.allocations,
            b.ns / a.ns,
        );
        self.checks.push(json!({"name": name, "valid": valid,
            "ours": {"ns": a.ns, "allocations": a.allocations},
            "jsonschema": {"ns": b.ns, "allocations": b.allocations},
            "ratio": b.ns / a.ns}));
    }
}

fn main() {
    settle();
    let mut run = Run::default();
    run.markdown += "| check | flowschema | jsonschema 0.60 | flowschema faster by |\n";
    run.markdown += "|---|---|---|---|\n";
    for (name, schema, data) in checks() {
        run.row(&name, &schema, &data);
    }
    run.markdown += "\n| compiled once | flowschema | jsonschema 0.60 | flowschema faster by |\n";
    run.markdown += "|---|---|---|---|\n";
    let mut compiled = vec![];
    for (name, schema) in fixtures::compiled() {
        let a = measure(|| Schema::new(schema.clone()));
        let b = measure(|| reference(&schema).is_some());
        let copy = measure(|| schema.clone());
        let ours = a.ns - copy.ns;
        run.markdown += &format!(
            "| {name} | {} | {} | {:.2}x |\n",
            shown(ours),
            shown(b.ns),
            b.ns / ours
        );
        compiled.push(
            json!({"name": name, "ours": {"ns": ours}, "jsonschema": {"ns": b.ns},
            "ratio": b.ns / ours}),
        );
    }
    let review = verdict();
    let broken = issues(5, true);
    let ours = Schema::new(review.clone());
    let mut messages = Value::Null;
    if let Some(theirs) = reference(&review) {
        let a = measure(|| ours.mismatches(&broken).len());
        // Both write every message out as text, as the model reads them.
        let b = measure(|| theirs.iter_errors(&broken).map(|e| e.to_string()).count());
        run.markdown += &format!(
            "\nEvery message of an invalid verdict: flowschema {}, jsonschema {} ({:.2}x).\n",
            shown(a.ns),
            shown(b.ns),
            b.ns / a.ns
        );
        messages = json!({"ours": {"ns": a.ns}, "jsonschema": {"ns": b.ns}, "ratio": b.ns / a.ns});
    }
    let json = json!({"checks": run.checks, "compiled": compiled, "messages": messages});
    publish("micro", &json, &run.markdown);
}
