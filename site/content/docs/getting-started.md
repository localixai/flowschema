# Getting started

flowschema is a Rust crate that checks JSON against a JSON Schema: a tool call against the tool's parameters, a structured answer against its shape. This page takes it from the install to a first check and the messages a model can fix its answer from.

## Install

The crate is taken from its repository; it is not on crates.io, and no release is tagged yet.

```sh
cargo add flowschema --git https://github.com/localixai/flowschema
cargo add serde_json
```

For a build that must not move, pin the commit in `Cargo.toml`:

```toml
[dependencies]
flowschema = { git = "https://github.com/localixai/flowschema", rev = "<a commit>" }
serde_json = "1"
```

It needs Rust 1.89 or later and brings two dependencies: `serde_json`, whose `Value` the API takes, and `regex-lite`, for the few patterns its own matcher does not take. It builds for `wasm32-unknown-unknown`.

## A first check

A schema is compiled once and checked against as often as needed.

```rust
use flowschema::Schema;
use serde_json::json;

let schema = Schema::new(&json!({
    "type": "object",
    "required": ["verdict"],
    "properties": {"verdict": {"enum": ["approve", "request_changes"]}}
}));

assert!(schema.matches(&json!({"verdict": "approve"})));
assert_eq!(
    schema.mismatches(&json!({"verdict": "maybe"})),
    [r#"data.verdict must be one of ["approve","request_changes"] (is "maybe")"#]
);
```

`matches` is the quick check: it stops at the first failure, writes nothing and allocates nothing. `mismatches` walks again only when the data failed, and writes the first twenty messages.

## What a message says

Every message names where the value is, the rule, and what the data has, in words a model can mend its answer from:

```text
data.issues[0].line must be >= 1 (is 0)
data.issues[0].severity must be one of ["low","medium","high"] (is "urgent")
data must match exactly one schema of oneOf (matches none)
```

Send them back as the tool call's result, or as the next turn, and ask for the answer again: [Tool calls and answers](tool-calls-and-answers.md) has both loops.

## A schema never fails to compile

`Schema::new` takes any JSON Schema, an object or `true` or `false`, and never fails. What a schema asks that flowschema does not check is named, each keyword once, so the program can say so or refuse the schema:

```rust
use flowschema::Schema;
use serde_json::json;

let schema = Schema::new(&json!({"type": "string", "pattern": "^(?=a)b"}));
assert!(!schema.unchecked().is_empty());
```

[What is checked](what-is-checked.md) lists the drafts, the keywords and the formats.

## Where to go next

- [Tool calls and answers](tool-calls-and-answers.md): the two places an agent checks JSON, and a schema that comes with the request.
- [Provider strict modes](provider-strict-modes.md): the schema as OpenAI, Anthropic or Gemini takes it, and what is still yours to check.
- [Cut-off and streaming calls](cut-off-and-streaming.md): a reply the output limit stopped, a call as it arrives, a number written as a string.
- [API](api.md): every item on one page.
