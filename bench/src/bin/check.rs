//! The last run held to `budgets.json`: each rule read against the results it names
//! (`results/*.json`), one line each, and a failure exit when any is broken or has no results.

use flowschema_bench::results;
use serde_json::Value;

fn read(name: &str) -> Option<Value> {
    let text = std::fs::read_to_string(results(name)).ok()?;
    serde_json::from_str(&text).ok()
}

fn number(value: &Value, path: &[&str]) -> f64 {
    path.iter()
        .fold(value, |at, key| &at[*key])
        .as_f64()
        .unwrap_or(f64::NAN)
}

/// Whether `rule` holds, and what was found; `None` when its results are missing.
fn judge(rule: &Value) -> Option<(bool, String)> {
    let id = rule["id"].as_str().unwrap_or("");
    Some(match id {
        "no-allocation" => {
            let micro = read("micro.json")?;
            let max = rule["max"].as_u64().unwrap_or(0);
            let over: Vec<String> = (micro["checks"].as_array()?.iter())
                .filter(|check| {
                    let name = check["name"].as_str().unwrap_or("");
                    let allowed = rule["except"][name].as_u64().unwrap_or(max);
                    check["ours"]["allocations"].as_u64().unwrap_or(u64::MAX) > allowed
                })
                .map(|check| check["name"].to_string())
                .collect();
            (over.is_empty(), format!("allocating: {over:?}"))
        }
        "never-slower" => {
            let micro = read("micro.json")?;
            let min = number(rule, &["minRatio"]);
            let rows = (micro["checks"].as_array()?.iter())
                .chain(micro["compiled"].as_array()?.iter())
                .chain(micro["requests"].as_array()?.iter())
                .chain(std::iter::once(&micro["messages"]).filter(|row| !row.is_null()));
            let slower: Vec<String> = rows
                .filter(|row| number(row, &["ratio"]) < min)
                .map(|row| format!("{} {:.2}x", row["name"], number(row, &["ratio"])))
                .collect();
            (slower.is_empty(), format!("below {min}x: {slower:?}"))
        }
        "suite" => {
            let suite = read("suite.json")?;
            let failures = suite["failures"].as_array()?.len();
            let max = rule["maxFailures"].as_u64().unwrap_or(0);
            (failures as u64 <= max, format!("{failures} failures"))
        }
        "corpus-labels" => {
            let corpus = read("corpus.json")?;
            let share = 100.0 * number(&corpus, &["oursRight"]) / number(&corpus, &["answers"]);
            let min = number(rule, &["minShare"]);
            (share >= min, format!("{share:.2}% (at least {min}%)"))
        }
        "corpus-panics" => {
            let corpus = read("corpus.json")?;
            let panicked = number(&corpus, &["panicked"]);
            (
                panicked <= number(rule, &["max"]),
                format!("{panicked} panics"),
            )
        }
        "corpus-speed" => {
            let corpus = read("corpus.json")?;
            let ratio = |what: &str| {
                number(&corpus, &[what, "jsonschema"]) / number(&corpus, &[what, "ours"])
            };
            let (compile, check) = (ratio("compileMs"), ratio("checkMs"));
            let holds = compile >= number(rule, &["minCompileRatio"])
                && check >= number(rule, &["minCheckRatio"]);
            (holds, format!("compile {compile:.2}x, check {check:.2}x"))
        }
        "size" => {
            let size = read("size.json")?;
            let added = number(&size, &["flowschema"]);
            let max = number(rule, &["maxBytes"]);
            (added <= max, format!("{added} bytes (at most {max})"))
        }
        _ => (false, format!("no check for the rule {id:?}")),
    })
}

fn main() {
    let budgets = std::fs::read_to_string(results("../budgets.json"))
        .ok()
        .and_then(|text| serde_json::from_str::<Value>(&text).ok())
        .unwrap_or(Value::Null);
    let rules = budgets["rules"].as_array().cloned().unwrap_or_default();
    let mut broken = rules.is_empty();
    for rule in &rules {
        let id = rule["id"].as_str().unwrap_or("?");
        match judge(rule) {
            Some((true, found)) => println!("ok      {id}: {found}"),
            Some((false, found)) => {
                broken = true;
                println!("BROKEN  {id}: {found}");
            }
            None => {
                broken = true;
                println!("MISSING {id}: no results; run the benchmark that writes them");
            }
        }
    }
    if broken {
        std::process::exit(1);
    }
}
