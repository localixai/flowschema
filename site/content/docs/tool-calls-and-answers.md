# Tool calls and answers

An agent meets JSON Schema in two places: a tool's parameters, which every call the model writes must fit before the tool runs, and the shape of a structured answer, which the program reads back. In both the schema comes at run time, the data comes from a model, and what the program does with a failure is tell the model what to fix.

## A tool call, before the tool runs

Compile each tool's parameters when the tool is registered, and check every call against them. A call that does not fit is not run: its messages go back to the model as the call's result, and the model writes the call again.

```rust
use flowschema::Schema;
use serde_json::json;

let read = Schema::new(&json!({
    "type": "object",
    "required": ["path"],
    "additionalProperties": false,
    "properties": {
        "path": {"type": "string"},
        "limit": {"type": "integer", "minimum": 1}
    }
}));

let call = json!({"path": "src/lib.rs", "limit": 0});
assert!(!read.matches(&call));
assert_eq!(read.mismatches(&call), ["data.limit must be >= 1 (is 0)"]);
```

## A structured answer

Ask the model for an answer in a shape, check what it returns, and send the messages back until it fits, a bounded number of times.

```rust
use flowschema::Schema;
use serde_json::json;

let verdict = Schema::new(&json!({
    "type": "object",
    "required": ["verdict", "summary"],
    "properties": {
        "verdict": {"enum": ["approve", "request_changes"]},
        "summary": {"type": "string", "minLength": 1}
    }
}));

let answer = json!({"verdict": "maybe", "summary": "Looks fine."});
let fix = verdict.mismatches(&answer).join("\n");
assert_eq!(fix, r#"data.verdict must be one of ["approve","request_changes"] (is "maybe")"#);
// Sent to the model: "Your answer does not match the schema: <fix>. Answer again."
```

## A schema that comes with the request

Where an agent serves many users, no schema is known ahead: each request brings its own tools and its own answer shape, and a tool server may change its tools as it runs. Compile the schema from the request, check, and drop it. `Schema::new` reads the document where it is and copies nothing, and a tool's parameters compile in well under a microsecond ([the figures](benchmarks.md)), so there is no cache to key, to bound or to clear.

```rust
use flowschema::Schema;
use serde_json::json;

// The tools as a request lists them, and the call the model wrote.
let tools = json!([{"name": "read", "input_schema": {"type": "object", "required": ["path"],
    "properties": {"path": {"type": "string"}}}}]);
let (called, arguments) = ("read", json!({"path": 7}));

let tool = (tools.as_array().into_iter().flatten()).find(|tool| tool["name"] == called);
let said = tool.map(|tool| Schema::new(&tool["input_schema"]).mismatches(&arguments));
assert_eq!(said, Some(vec!["data.path must be string (is integer)".to_string()]));
```

## One compile for every request

Where the schemas are known ahead, compile each once. A compiled `Schema` is `Send + Sync` and does not change, so one compile serves every request, on any thread: keep it in an `Arc` beside the tool it belongs to.

## What agents' schemas ask

The schemas agents send are small and plain, and a call is smaller still: a flat object of two or three fields, most of them strings. Nearly every one is an object with `properties` and `required`; `additionalProperties`, `items`, an `enum` of strings, `anyOf`, `format` and `$ref` come next, and the rest of JSON Schema is rare. flowschema is made fast where these are, checks the rest as the drafts say, and a schema pays nothing for what it does not use. The repository's [guide](https://github.com/localixai/flowschema/blob/main/docs/agents.md#what-an-agents-schemas-ask) has the counts, keyword by keyword.
