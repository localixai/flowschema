# flowschema in an agent

An agent meets JSON Schema in two places: a tool's parameters, which every call the model writes
must fit before the tool runs, and the shape of a structured answer, which the program reads
back. In both, the schema comes at run time, the data comes from a model, and what the program
does with a failure is tell the model what to fix. flowschema is built for that: a schema is
compiled once and checked against many times, a check allocates nothing, and every message says
where, what rule and what the data has.

## A tool call, before the tool runs

Compile each tool's parameters once, when the tool is registered, and check every call against
them. A call that does not fit is not run: its messages go back to the model as the call's
result, and the model writes the call again.

```rust
use flowschema::Schema;
use serde_json::json;

let read = Schema::new(json!({
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

`matches` is the quick check and the common case: it stops at the first failure and writes
nothing. `mismatches` walks again only when the data failed, and writes the first twenty
messages.

## A structured answer

Ask the model for an answer in a shape, check what it returns, and send the messages back until
it fits, a bounded number of times.

```rust
use flowschema::Schema;
use serde_json::json;

let verdict = Schema::new(json!({
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

## What is not checked

A schema may ask for something flowschema does not check: a pattern's lookaround, an `idn-*`
format, a `$ref` to a document it was not given. It never takes such a schema as stricter than
it is checked: `unchecked` names each keyword once, so the program can say so, or refuse the
schema.

```rust
use flowschema::Schema;
use serde_json::json;

let schema = Schema::new(json!({"type": "string", "pattern": "^(?=a)b"}));
assert!(!schema.unchecked().is_empty());
```

## Shared across requests

A compiled `Schema` is `Send + Sync` and does not change, so one compile serves every request,
on any thread: keep it in an `Arc` beside the tool it belongs to.

## In the works

What an agent does with JSON beyond the check, planned for the next releases:

- **Streamed and cut-off calls.** A call's arguments arrive in pieces, and a reply the output
  limit cut off ends midway: what of it is whole, and whether what came so far can still match.
- **A schema fitted to a provider.** The strict modes of model providers take only part of JSON
  Schema (every property required, no `additionalProperties` but `false`, `anyOf` for `oneOf`):
  the schema rewritten for one, with what it lost said, while the check stays the original's.
- **Lenient reading of what models write.** A whole number written as `30.0`, arguments sent as a
  string of JSON, `null` for an optional field: read as the model meant them, and said so.
