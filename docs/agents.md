# flowschema in an agent

An agent meets JSON Schema in two places: a tool's parameters, which every call the model writes
must fit before the tool runs, and the shape of a structured answer, which the program reads
back. In both, the schema comes at run time, the data comes from a model, and what the program
does with a failure is tell the model what to fix. flowschema is built for that: a schema
compiles in about a microsecond, from the request that brings it or once for many checks, a
check allocates nothing, and every message says where, what rule and what the data has.

## A tool call, before the tool runs

Compile each tool's parameters once, when the tool is registered, and check every call against
them. A call that does not fit is not run: its messages go back to the model as the call's
result, and the model writes the call again.

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

`matches` is the quick check and the common case: it stops at the first failure and writes
nothing. `mismatches` walks again only when the data failed, and writes the first twenty
messages.

## A structured answer

Ask the model for an answer in a shape, check what it returns, and send the messages back until
it fits, a bounded number of times.

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

Where an agent serves many users, no schema is known ahead: each request brings its own tools
and its own answer shape, and a tool server may change its tools as it runs. Compile the schema
from the request, check, and drop it. `Schema::new` reads the document where it is and copies
nothing, and a tool's parameters compile in well under a microsecond
([the figures](comparison.md)), so there is no cache to key, to bound or to clear.

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

## What an agent's schemas ask

The schemas agents send are small and plain, and a call is smaller still: a flat object of two or
three fields, most of them strings. MaskBench says what the schemas hold, by where they come
from (`bench`'s `keywords`):

<!-- bench:keywords -->

| keyword | function calling | Model Context Protocol | documents |
|---|---|---|---|
| schemas | 2750 | 45 | 7951 |
| objects in a schema, the median | 9 | 12 | 16 |
| `type` | 100.0% | 100.0% | 99.8% |
| `properties` | 100.0% | 100.0% | 97.4% |
| `required` | 99.2% | 97.8% | 74.5% |
| `additionalProperties` | 38.1% | 71.1% | 53.0% |
| `items` | 26.9% | 28.9% | 53.3% |
| `enum` | 15.1% | 26.7% | 41.7% |
| `anyOf` | 13.4% | 28.9% | 11.2% |
| `format` | 5.8% | 37.8% | 25.5% |
| `$ref` | 0.0% | 100.0% | 35.0% |
| `const` | 0.3% | 84.4% | 1.7% |
| `oneOf` | 1.9% | 0.0% | 17.3% |
| `allOf` | 0.0% | 0.0% | 4.6% |
| `minimum` | 0.1% | 17.8% | 15.3% |
| `maximum` | 0.1% | 17.8% | 8.3% |
| `pattern` | 0.0% | 0.0% | 23.5% |
| `minLength` | 0.0% | 0.0% | 13.6% |
| `maxLength` | 0.0% | 0.0% | 12.3% |
| `minItems` | 0.0% | 0.0% | 12.2% |
| `maxItems` | 0.0% | 0.0% | 5.9% |
| `patternProperties` | 0.0% | 0.0% | 9.1% |
| `uniqueItems` | 0.0% | 0.0% | 1.7% |
| `multipleOf` | 0.0% | 0.0% | 1.1% |
| `not` | 0.3% | 0.0% | 2.5% |
| `if` | 0.0% | 0.0% | 0.8% |
| `dependencies` | 0.7% | 0.0% | 3.4% |
| `contains` | 0.0% | 0.0% | 0.2% |
| `propertyNames` | 0.0% | 0.0% | 0.4% |
| `prefixItems` | 0.0% | 0.0% | 0.0% |
| `unevaluatedProperties` | 0.0% | 0.0% | 0.0% |
| `dependentRequired` | 0.0% | 0.0% | 0.0% |

The share of each group's schemas that say the keyword anywhere. The sources written to test a validator (Handwritten, Synthesized) are left out.

<!-- /bench:keywords -->

So the library is made fast where these are: an object with its `properties`, `required` and
`additionalProperties`, an `enum` of strings, a `const`, a `format`, a bound, an `anyOf`, a
`$ref`. The rest of JSON Schema is checked as the drafts say and costs a schema that does not
use it nothing.

## What a provider already holds the model to

A provider's strict mode constrains what the model can write, to a part of JSON Schema that
differs from one provider to the next (their documentation, November 2026):

| | OpenAI, strict | Anthropic, strict | Gemini | Tools without strict, MCP |
|---|---|---|---|---|
| `anyOf`, `$ref`, `enum` | held | held | held | not held |
| `oneOf`, `allOf`, `not`, `if` | refused, or not listed | `allOf` in part | not listed | not held |
| number bounds | held | not held | held | not held |
| `minLength`, `maxLength` | not held | not held | ignored | not held |
| `additionalProperties` | must be `false` | must be `false` | any | any |

What the provider does not hold is still the schema's: a bound, a length, a pattern, a count, and
all of it where the tool is not strict or comes from a tool server. That is what is checked here,
against the schema as it was written. The sources:
[OpenAI](https://developers.openai.com/api/docs/guides/structured-outputs),
[Anthropic](https://platform.claude.com/docs/en/build-with-claude/structured-outputs),
[Gemini](https://ai.google.dev/gemini-api/docs/structured-output),
[the Model Context Protocol](https://modelcontextprotocol.io/specification/2025-11-25/server/tools).

## What is not checked

A schema may ask for something flowschema does not check: a pattern's lookaround, an `idn-*`
format, a `$ref` to a document it was not given. It never takes such a schema as stricter than
it is checked: `unchecked` names each keyword once, so the program can say so, or refuse the
schema.

```rust
use flowschema::Schema;
use serde_json::json;

let schema = Schema::new(&json!({"type": "string", "pattern": "^(?=a)b"}));
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
