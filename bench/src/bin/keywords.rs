//! What the schemas of MaskBench ask, by where they come from: for each keyword, the share of
//! the schemas that say it anywhere, in the function-calling sources (BFCL, Glaive), in the
//! Model Context Protocol's own schemas and in the documents (GitHub's, Kubernetes', JSON Schema
//! Store's and the rest). Writes `results/keywords.{json,md}`: what an agent's schemas hold,
//! counted, not guessed.

use std::collections::{BTreeMap, BTreeSet};

use flowschema_bench::{data, publish};
use serde_json::{Value, json};

/// The keywords counted, in the order the table lists them.
const KEYWORDS: [&str; 30] = [
    "type",
    "properties",
    "required",
    "additionalProperties",
    "items",
    "enum",
    "anyOf",
    "format",
    "$ref",
    "const",
    "oneOf",
    "allOf",
    "minimum",
    "maximum",
    "pattern",
    "minLength",
    "maxLength",
    "minItems",
    "maxItems",
    "patternProperties",
    "uniqueItems",
    "multipleOf",
    "not",
    "if",
    "dependencies",
    "contains",
    "propertyNames",
    "prefixItems",
    "unevaluatedProperties",
    "dependentRequired",
];

/// The groups of sources, in the order of the table's columns.
const GROUPS: [&str; 3] = ["function calling", "Model Context Protocol", "documents"];

/// The group a corpus file belongs to; none for the sources written as tests of a validator.
fn group(file: &str) -> Option<usize> {
    if file.starts_with("BFCL") || file.starts_with("Glaiveai2K") {
        Some(0)
    } else if file.starts_with("MCPspec") {
        Some(1)
    } else if file.starts_with("Handwritten") || file.starts_with("Synthesized") {
        None
    } else {
        Some(2)
    }
}

/// Every key any object of `value` has, and how many objects it holds.
fn keys(value: &Value, seen: &mut BTreeSet<String>, objects: &mut usize) {
    match value {
        Value::Object(fields) => {
            *objects += 1;
            for (key, inner) in fields {
                seen.insert(key.clone());
                keys(inner, seen, objects);
            }
        }
        Value::Array(items) => items.iter().for_each(|item| keys(item, seen, objects)),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
}

fn main() {
    let dir = data("jsonschemabench/maskbench/data");
    let mut files: Vec<_> = (std::fs::read_dir(&dir)
        .expect("run fetch.sh first")
        .flatten())
    .map(|entry| entry.file_name().to_string_lossy().into_owned())
    .collect();
    files.sort();
    let mut schemas = [0_usize; 3];
    let mut sizes: [Vec<usize>; 3] = Default::default();
    let mut using: BTreeMap<&str, [usize; 3]> = BTreeMap::new();
    for file in &files {
        let Some(at) = group(file) else { continue };
        let text = std::fs::read_to_string(dir.join(file)).expect("a corpus file");
        let Ok(document) = serde_json::from_str::<Value>(&text) else {
            continue;
        };
        let (mut seen, mut objects) = (BTreeSet::new(), 0);
        keys(&document["schema"], &mut seen, &mut objects);
        schemas[at] += 1;
        sizes[at].push(objects);
        for keyword in KEYWORDS {
            if seen.contains(keyword) {
                using.entry(keyword).or_default()[at] += 1;
            }
        }
    }
    let share = |keyword: &str, at: usize| {
        100.0 * using.get(keyword).map_or(0, |by| by[at]) as f64 / schemas[at].max(1) as f64
    };
    let median = |at: usize| {
        let mut sorted = sizes[at].clone();
        sorted.sort_unstable();
        sorted.get(sorted.len() / 2).copied().unwrap_or(0)
    };
    let mut markdown = format!(
        "| keyword | {} | {} | {} |\n|---|---|---|---|\n",
        GROUPS[0], GROUPS[1], GROUPS[2]
    );
    markdown += &format!(
        "| schemas | {} | {} | {} |\n",
        schemas[0], schemas[1], schemas[2]
    );
    markdown += &format!(
        "| objects in a schema, the median | {} | {} | {} |\n",
        median(0),
        median(1),
        median(2)
    );
    let mut rows = vec![];
    for keyword in KEYWORDS {
        let shares = [share(keyword, 0), share(keyword, 1), share(keyword, 2)];
        markdown += &format!(
            "| `{keyword}` | {:.1}% | {:.1}% | {:.1}% |\n",
            shares[0], shares[1], shares[2]
        );
        rows.push(json!({"keyword": keyword, "share": shares}));
    }
    markdown += "\nThe share of each group's schemas that say the keyword anywhere. The sources \
        written to test a validator (Handwritten, Synthesized) are left out.\n";
    let json = json!({"groups": GROUPS, "schemas": schemas, "keywords": rows});
    publish("keywords", &json, &markdown);
    print!("{markdown}");
}
