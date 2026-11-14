# What is checked

flowschema checks JSON Schema as its drafts say: every keyword of validation, every applicator, references and formats. This page lists them, says what is left unchecked and how a program learns of it, and how the library is held to all of it.

## Drafts

Drafts 4, 6, 7, 2019-09 and 2020-12, chosen by `$schema` (2020-12 when it names none), each with its own rules: draft 4's boolean `exclusiveMinimum`, the `$ref` that hides its siblings before 2019-09, `items` as a list before 2020-12, draft 7's `dependencies`; and a metaschema's `$vocabulary` when it leaves out the validation vocabulary.

## Keywords

- **Validation**: `type`, `enum`, `const`; the bounds and `multipleOf`, exact in decimal (`0.0075` is a multiple of `0.0001`), integers exact past 2^53; `minLength` and `maxLength` in characters; `pattern`; the array keywords, `uniqueItems` and `contains` with its counts; the object keywords, `required`, `dependentRequired` and the property counts.
- **Applicators**: `allOf`, `anyOf`, `oneOf`, `not`, `if`/`then`/`else`, `properties`, `patternProperties`, `additionalProperties`, `propertyNames`, `dependentSchemas`, `items`, `prefixItems`, `contains`, `unevaluatedProperties` and `unevaluatedItems`.
- **References**: `$ref` by JSON pointer, `$id`, `$anchor` and `$dynamicRef` inside the resource, and into the documents given to `Schema::with_documents`. Nothing is fetched, and a loop with no step into the data is cut.

## Formats and patterns

`format` is asserted, since a model's answer is what it is for: `date-time`, `date`, `time`, `duration`, `email`, `hostname`, `ipv4`, `ipv6`, `uri`, `uri-reference`, `iri`, `iri-reference`, `uuid`, `json-pointer`, `relative-json-pointer`, `uri-template` and `regex`.

`pattern` is read as ECMA-262 reads it (`\d` and `\w` ASCII, `\s` with Unicode's spaces, `.` stopping at a line's end), on a string's characters. Every pattern runs in time linear in its text: none backtracks.

## What is not checked

A lookaround or a back-reference in a pattern, the `idn-hostname` and `idn-email` formats (they need Unicode's IDNA tables), a `$ref` to a document not given, and a `$dynamicRef` that leaves its resource. A schema that asks for one is never taken as stricter than it is checked: `Schema::unchecked` names each such keyword once, so the program can say so or refuse the schema.

```rust
use flowschema::Schema;
use serde_json::json;

let schema = Schema::new(&json!({"type": "string", "pattern": "^(?=a)b"}));
assert!(!schema.unchecked().is_empty());
```

## How it is held to this

- **The official test suite.** Every required test of the [JSON-Schema-Test-Suite](https://github.com/json-schema-org/JSON-Schema-Test-Suite) for the five drafts, with the format tests and ECMA-262's regular expressions: 7,675 of 7,675 pass. A group whose schema asks for what is not checked is counted apart, not as passed.
- **Real LLM tasks.** [MaskBench](https://github.com/guidance-ai/jsonschemabench), the schemas of function calling, tool servers and documents with answers labelled as matching or not: flowschema gives 99.98% of them their label.
- **Safe with a schema from anywhere.** No `unsafe`, no I/O, no panic in working code; the benchmarks check that no schema of the suite or of MaskBench makes it panic.

The tables, draft by draft and source by source, are in the repository's [conformance page](https://github.com/localixai/flowschema/blob/main/docs/conformance.md), written from its benchmark results.
