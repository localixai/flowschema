# API

One type, `Schema`, compiled from a JSON Schema and checked against once or many times, and two functions beside it: `fit`, for a provider's strict mode, and `whole`, for a text the output limit cut off.

```rust
use flowschema::Schema;
use serde_json::json;

let schema = Schema::new(&json!({"type": "object", "required": ["id"]}));
assert!(schema.matches(&json!({"id": 1})));
assert_eq!(schema.mismatches(&json!({})), [r#"data is missing "id""#]);
assert!(schema.unchecked().is_empty());
```

## Schema

| Item | What it does |
| --- | --- |
| `Schema::new(json: &Value) -> Schema` | Compiles a schema: an object, or `true` or `false`. The document is read where it is and not copied, and nothing of it is kept. The draft is read from `$schema`, 2020-12 when it names none. It never fails: what it cannot check is named by `unchecked`. |
| `Schema::with_documents(json: &Value, documents: &[(String, Value)]) -> Schema` | The same, with the documents its `$ref`s may land in, each given with its URI (`https://example.com/defs.json`). Nothing is fetched. |
| `matches(&self, data: &Value) -> bool` | Whether `data` matches. Stops at the first failure, writes nothing, allocates nothing. |
| `mismatches(&self, data: &Value) -> Vec<String>` | Every way `data` does not match, the first twenty; empty when it matches. Each message names where the value is (`data.issues[0].line`), the rule and what the data has. |
| `unchecked(&self) -> Vec<&'static str>` | The keywords the schema asks for that are not checked, each named once. |
| `may_match(&self, written: &str) -> bool` | Whether a call still being written can still end in a match: `written` is the text so far, and the answer is false only where no ending mends it. |
| `mend(&self, data: &mut Value) -> Vec<String>` | `data` read as the model meant it, where the schema leaves no doubt: a number, a boolean or null written as a string, an object or an array sent as a string of JSON. Mended in place; each change said with where. |

A `Schema` is `Send + Sync` and does not change: one compile serves every thread.

## Functions

| Item | What it does |
| --- | --- |
| `fit(schema: &Value, provider: Provider) -> Fitted` | The schema as a provider's strict mode takes it (`Provider::OpenAi`, `Anthropic`, `Gemini`): `Fitted::schema` to send, and `Fitted::changes`, each change with where it was made. See [Provider strict modes](provider-strict-modes.md). |
| `whole(text: &str) -> Option<Value>` | The value `text` holds, or what of it was written whole where the output limit cut it off: open strings, objects and arrays closed, what the cut left unfinished left out. `None` for what is not the start of a JSON value. See [Cut-off and streaming calls](cut-off-and-streaming.md#a-call-the-output-limit-cut-off). |

## Dependencies

Two: `serde_json`, whose `Value` the API takes, and `regex-lite`, for the few patterns flowschema's own matcher does not take. [Getting started](getting-started.md#install) has the install.
