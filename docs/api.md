# API

One type, `Schema`: compiled from a JSON Schema, in about a microsecond for a tool's, and
checked against once or many times.

```rust
use flowschema::Schema;
use serde_json::json;

let schema = Schema::new(&json!({"type": "object", "required": ["id"]}));
assert!(schema.matches(&json!({"id": 1})));
assert_eq!(schema.mismatches(&json!({})), [r#"data is missing "id""#]);
assert!(schema.unchecked().is_empty());
```

| Item | What it does |
| --- | --- |
| `Schema::new(json: &Value) -> Schema` | Compiles a schema: an object, or `true` or `false`. The document is read where it is and not copied, so a schema that comes with a request costs no more than its compile; nothing of it is kept. The draft is read from `$schema`, 2020-12 when it names none. It never fails: what it cannot check is named by `unchecked`. |
| `Schema::with_documents(json: &Value, documents: &[(String, Value)]) -> Schema` | The same, with the documents its `$ref`s may land in, each given with its URI (`https://example.com/defs.json`). Nothing is fetched. |
| `matches(&self, data: &Value) -> bool` | Whether `data` matches. Stops at the first failure, writes nothing, allocates nothing. |
| `mismatches(&self, data: &Value) -> Vec<String>` | Every way `data` does not match, the first twenty; empty when it matches. Each message names where the value is (`data.issues[0].line`), the rule and what the data has. |
| `unchecked(&self) -> Vec<&'static str>` | The keywords the schema asks for that are not checked here, each named once. |

A `Schema` is `Send + Sync`. The crate's two dependencies are `serde_json`, whose `Value` the
API takes, and `regex-lite`, for the few patterns its own matcher does not take.

## Install

```toml
[dependencies]
flowschema = { git = "https://github.com/localixai/flowschema", tag = "v0.1.0" }
serde_json = "1"
```
