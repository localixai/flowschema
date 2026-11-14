# Provider strict modes

A provider's strict mode constrains what the model can write, to a part of JSON Schema that differs from one provider to the next. This page says what each holds the model to, what is still the program's to check, and how to send a provider the schema it takes.

## What a provider holds the model to

From the providers' documentation as it read in November 2026:

| | OpenAI, strict | Anthropic, strict | Gemini | Tools without strict, MCP |
| --- | --- | --- | --- | --- |
| `anyOf`, `$ref`, `enum` | held | held | held | not held |
| `oneOf`, `allOf`, `not`, `if` | refused, or not listed | `allOf` in part | not listed | not held |
| number bounds | held | not held | held | not held |
| `minLength`, `maxLength` | not held | not held | ignored | not held |
| `additionalProperties` | must be `false` | must be `false` | any | any |

What the provider does not hold is still the schema's: a bound, a length, a pattern, a count, and all of it where the tool is not strict or comes from a tool server. That is what flowschema checks, against the schema as it was written. The sources: [OpenAI](https://developers.openai.com/api/docs/guides/structured-outputs), [Anthropic](https://platform.claude.com/docs/en/build-with-claude/structured-outputs), [Gemini](https://ai.google.dev/gemini-api/docs/structured-output), [the Model Context Protocol](https://modelcontextprotocol.io/specification/2025-11-25/server/tools).

## A schema fitted to a provider

Send a provider's strict mode the part of the schema it takes, and check the answer against the schema as it was written. `fit` gives the schema to send, with what the provider does not hold taken out and what it asks of every object set, and says each change:

```rust
use flowschema::{Provider, Schema, fit};
use serde_json::json;

let written = json!({"type": "object", "required": ["path"], "properties": {
    "path": {"type": "string"},
    "limit": {"type": "integer", "minimum": 1}}});

let sent = fit(&written, Provider::Anthropic);
assert_eq!(sent.schema["properties"]["limit"], json!({"type": "integer"}));
assert_eq!(sent.schema["additionalProperties"], json!(false));
assert_eq!(sent.changes, [
    "schema.properties.limit: `minimum` is not held: taken out",
    "schema: `additionalProperties` set to false, as the provider asks",
]);

// The provider held the answer to what it takes; the bound is still the schema's.
let answer = json!({"path": "src/lib.rs", "limit": 0});
assert!(Schema::new(&sent.schema).matches(&answer));
assert_eq!(Schema::new(&written).mismatches(&answer), ["data.limit must be >= 1 (is 0)"]);
```

## What changes for each provider

- **OpenAI**: every property becomes required, and one that was optional may be null instead; a `oneOf` is sent as an `anyOf`; every object refuses the fields it does not name (`additionalProperties: false`).
- **Anthropic**: every object refuses the fields it does not name.
- **Gemini**: a `const` is sent as an `enum` of one value.

For each of them, a keyword the provider does not hold is taken out, and `changes` says so with where it was.

## The limits of `fit`

The rules are the providers' documentation, not a trial of their APIs. A schema a provider refuses whole is not made into one it takes: a root that is not an object for OpenAI, a schema that refers to itself for Anthropic. `changes` is what to read before trusting a fitted schema: it names every keyword that was taken out, and so everything the answer must still be checked for.
