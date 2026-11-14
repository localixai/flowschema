//! The fixed schemas and data every timing benchmark reads (`micro`, `quick`): typical checks
//! of what a model writes, each a name, a schema and the data checked against it, in the order
//! the tables list them.

use serde_json::{Map, Value, json};

/// A code review's verdict, as an agent's structured output asks for one.
pub fn verdict() -> Value {
    json!({"type": "object", "required": ["verdict", "summary", "issues"],
        "additionalProperties": false,
        "properties": {
            "verdict": {"enum": ["approve", "request_changes", "comment"]},
            "summary": {"type": "string", "minLength": 1, "maxLength": 2000},
            "issues": {"type": "array", "maxItems": 50, "items": {"type": "object",
                "required": ["file", "line", "severity", "message"],
                "properties": {"file": {"type": "string"}, "line": {"type": "integer", "minimum": 1},
                    "severity": {"enum": ["low", "medium", "high"]},
                    "message": {"type": "string"}}}}}})
}

pub fn issues(count: usize, broken: bool) -> Value {
    let list: Vec<Value> = (0..count)
        .map(|i| match broken && i % 7 == 0 {
            true => json!({"file": "src/a.rs", "line": 0, "severity": "urgent", "message": 5}),
            false => json!({"file": format!("src/f{i}.rs"), "line": i + 1,
                "severity": (["low", "medium", "high"][i % 3]), "message": "Something to fix."}),
        })
        .collect();
    json!({"verdict": "request_changes", "summary": "Two problems in the parser.", "issues": list})
}

/// Rows of a table an answer lists.
pub fn rows() -> (Value, Value) {
    let schema = json!({"type": "array", "items": {"type": "object",
        "required": ["id", "name", "price", "tags"],
        "properties": {"id": {"type": "integer"}, "name": {"type": "string"},
            "price": {"type": "number"}, "stock": {"type": ["integer", "null"]},
            "tags": {"type": "array", "items": {"type": "string"}},
            "kind": {"enum": ["a", "b", "c"]}}}});
    let data = (0..1000)
        .map(|i| {
            json!({"id": i, "name": format!("item {i}"), "price": f64::from(i) * 1.5,
            "stock": if i % 2 == 0 { json!(i) } else { Value::Null },
            "tags": ["x", "y", "z"], "kind": (["a", "b", "c"][(i % 3) as usize])})
        })
        .collect();
    (schema, Value::Array(data))
}

/// Objects of twenty fields each, all required, nothing else allowed.
pub fn wide() -> (Value, Value) {
    let name = |i: usize| format!("field_{i:02}");
    let properties: Map<String, Value> = (0..20)
        .map(|i| {
            (
                name(i),
                json!({"type": if i % 2 == 0 { "string" } else { "integer" }}),
            )
        })
        .collect();
    let schema = json!({"type": "array", "items": {"type": "object",
        "required": (0..20).map(name).collect::<Vec<_>>(),
        "additionalProperties": false, "properties": properties}});
    let row = |r: usize| -> Value {
        let fields: Map<String, Value> = (0..20)
            .map(|i| {
                (
                    name(i),
                    if i % 2 == 0 {
                        json!(format!("v{r}"))
                    } else {
                        json!(r * i)
                    },
                )
            })
            .collect();
        Value::Object(fields)
    };
    (schema, Value::Array((0..500).map(row).collect()))
}

/// A tree through a recursive `$ref`.
pub fn tree() -> (Value, Value) {
    fn node(depth: usize) -> Value {
        match depth {
            0 => json!({"name": "leaf"}),
            _ => {
                json!({"name": format!("n{depth}"), "children": [node(depth - 1), node(depth - 1)]})
            }
        }
    }
    let schema = json!({"$defs": {"node": {"type": "object", "required": ["name"],
        "properties": {"name": {"type": "string"},
            "children": {"type": "array", "items": {"$ref": "#/$defs/node"}}}}},
        "$ref": "#/$defs/node"});
    (schema, node(8))
}

/// A union of tool calls told apart by `type`, as function calling writes them.
pub fn calls() -> (Value, Value) {
    let tools = [
        "read", "write", "run", "search", "ask", "fetch", "list", "delete",
    ];
    let branches: Vec<Value> = tools
        .iter()
        .map(|tool| {
            json!({"type": "object", "required": ["type", "arguments"],
            "additionalProperties": false,
            "properties": {"type": {"const": tool}, "arguments": {"type": "object",
                "required": ["target"], "properties": {"target": {"type": "string"},
                    "limit": {"type": "integer", "minimum": 0}}}}})
        })
        .collect();
    let schema = json!({"type": "array", "items": {"oneOf": branches}});
    let data = (0..64)
        .map(|i| json!({"type": tools[i % tools.len()], "arguments": {"target": format!("t{i}")}}))
        .collect();
    (schema, Value::Array(data))
}

pub fn strings(prefix: &str, count: usize) -> Vec<Value> {
    (0..count).map(|i| json!(format!("{prefix}-{i}"))).collect()
}

/// Every check `micro` times, in its table's order: a name, the schema, the data.
pub fn checks() -> Vec<(String, Value, Value)> {
    let mut out = vec![];
    let review = verdict();
    out.push((
        "review verdict, valid".into(),
        review.clone(),
        issues(5, false),
    ));
    out.push(("review verdict, invalid".into(), review, issues(5, true)));
    for (name, (schema, data)) in [
        ("1,000 rows", rows()),
        ("500 objects × 20 fields", wide()),
        ("tree of 511 nodes by `$ref`", tree()),
        ("64 tool calls, `oneOf` of 8", calls()),
    ] {
        out.push((name.into(), schema, data));
    }
    out.push((
        "`uniqueItems`, 1,000 strings".into(),
        json!({"type": "array", "uniqueItems": true}),
        Value::Array(strings("item", 1000)),
    ));
    out.push((
        "`enum` of 1,000, the last".into(),
        json!({"enum": strings("option", 1000)}),
        json!("option-999"),
    ));
    for (format, value) in [
        ("date-time", "2026-11-07T18:39:00Z"),
        ("email", "a.b@example.com"),
        ("uuid", "550e8400-e29b-41d4-a716-446655440000"),
        ("ipv4", "192.168.0.1"),
        ("uri", "https://example.com/a?b=c#d"),
    ] {
        out.push((
            format!("`format: {format}`"),
            json!({"format": format}),
            json!(value),
        ));
    }
    // A pattern as the `regex` format reads it: what this matcher cannot run (a lookaround, a
    // back-reference, a property it has no table of) is still a regular expression.
    for value in [
        "^[a-z]+\\d$",
        "^\\p{L}+$",
        "^\\P{ASCII}+$",
        "[\\P{Letter}]",
        "(?=a)b",
        "(a)\\1",
        "(?<n>a)\\k<n>",
    ] {
        out.push((
            format!("`format: regex`, `{value}`"),
            json!({"format": "regex"}),
            json!(value),
        ));
    }
    for (pattern, value) in [
        ("^[A-Z]{3}$", "USD"),
        ("^\\d{4}-\\d{2}-\\d{2}$", "2026-11-07"),
        (
            "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$",
            "a.b@example.com",
        ),
        ("^(small|medium|large)$", "medium"),
    ] {
        out.push((
            format!("`pattern: {pattern}`"),
            json!({"pattern": pattern}),
            json!(value),
        ));
    }
    out
}

/// The schemas `micro` times the compile of: a structured answer and a union of tool calls.
pub fn compiled() -> Vec<(&'static str, Value)> {
    vec![("review verdict", verdict()), ("tool calls", calls().0)]
}

/// What one request brings: a schema that arrives with it, and the one answer checked against it
/// before both are dropped. A structured answer's, and a tool's parameters with one call of it,
/// shaped as a coding agent's calls are: a flat object of a few fields, most of them strings.
pub fn requests() -> Vec<(&'static str, Value, Value)> {
    let tool = json!({"type": "object", "additionalProperties": false, "required": ["command"],
        "properties": {
            "command": {"type": "string", "minLength": 1, "description": "The command to run."},
            "description": {"type": "string", "description": "What the command does."},
            "timeout": {"type": "integer", "minimum": 1, "maximum": 600_000,
                "description": "Milliseconds to wait."},
            "background": {"type": "boolean", "description": "Whether to run it detached."}}});
    let call = json!({"command": "cargo test --locked --quiet 2>&1 | grep -E 'test result|FAILED' | head -20",
        "description": "Run the tests and show what failed", "timeout": 120_000});
    vec![
        ("review verdict", verdict(), issues(5, false)),
        ("one tool call", tool, call),
    ]
}
