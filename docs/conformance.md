# Conformance

What flowschema checks, and how it is held to it: the official test suite of JSON Schema, and the
schemas and answers of real LLM tasks. The tables between the `bench` marks are written by
`bench/run.sh` from `bench/results/`.

## What it checks

- **Drafts** 4, 6, 7, 2019-09 and 2020-12, chosen by `$schema` (2020-12 when it names none),
  each with its own rules: draft 4's boolean `exclusiveMinimum`, the `$ref` that hides its
  siblings before 2019-09, `items` as a list before 2020-12, draft 7's `dependencies`; and a
  metaschema's `$vocabulary` when it leaves out the validation vocabulary.
- **Every keyword of validation**: `type`, `enum`, `const`; the bounds and `multipleOf`, exact in
  decimal (`0.0075` is a multiple of `0.0001`), integers exact past 2^53; `minLength` and
  `maxLength` in characters; `pattern`; the array keywords, `uniqueItems` and `contains` with its
  counts; the object keywords, `required`, `dependentRequired` and the property counts.
- **Every applicator**: `allOf`, `anyOf`, `oneOf`, `not`, `if`/`then`/`else`, `properties`,
  `patternProperties`, `additionalProperties`, `propertyNames`, `dependentSchemas`, `items`,
  `prefixItems`, `contains`, `unevaluatedProperties` and `unevaluatedItems`.
- **References**: `$ref` by JSON pointer, `$id`, `$anchor` and `$dynamicRef` inside the
  resource, and into the documents given; a loop with no step into the data is cut.
- **`format`, asserted**, since a model's answer is what it is for: `date-time`, `date`, `time`,
  `duration`, `email`, `hostname`, `ipv4`, `ipv6`, `uri`, `uri-reference`, `iri`,
  `iri-reference`, `uuid`, `json-pointer`, `relative-json-pointer`, `uri-template` and `regex`.
- **`pattern`** as ECMA-262 reads it (`\d` and `\w` ASCII, `\s` with Unicode's spaces, `.`
  stopping at a line's end), on a string's characters.

Not checked, and named by `Schema::unchecked`: a lookaround or a back-reference in a pattern, the
`idn-hostname` and `idn-email` formats (they need Unicode's IDNA tables), a `$ref` to a document
not given, and a `$dynamicRef` that leaves its resource. A schema that asks for one is never taken
as stricter than it is checked.

## The official test suite

The [JSON-Schema-Test-Suite](https://github.com/json-schema-org/JSON-Schema-Test-Suite): its
required tests, the format tests and ECMA-262's regular expressions, through flowschema and the
`jsonschema` crate. A group whose schema asks for what is not checked here is counted apart, not
as passed.

<!-- bench:suite -->

| draft | flowschema | jsonschema 0.60 | tests left out (a keyword not checked here) |
|---|---|---|---|
| draft4 | 928/928 | 928/928 | 20 tests |
| draft6 | 1256/1256 | 1256/1256 | 20 tests |
| draft7 | 1584/1584 | 1584/1584 | 145 tests |
| draft2019-09 | 1944/1944 | 1944/1944 | 185 tests |
| draft2020-12 | 1963/1963 | 1963/1963 | 188 tests |

Tests left out, by what they ask that is not checked: `format` (375), `pattern` (76), `$dynamicRef across resources` (73), `$ref outside the schema` (34).

Failures: 0.

<!-- /bench:suite -->

## Real LLM tasks

[MaskBench](https://github.com/guidance-ai/jsonschemabench) (guidance-ai): the schemas of real
LLM tasks (function calling, GitHub's, Kubernetes', JSON Schema Store's), each with answers
labelled valid or not.

<!-- bench:corpus -->

| | flowschema | jsonschema 0.60 |
|---|---|---|
| schemas compiled | 11,306 of 11,306 (0 panics) | 11,298 (8 refused) |
| answers judged as labelled | 36,984 of 36,990 (99.98%) | 36,970 (99.95%) |
| compiling every schema | 123.8 ms | 467.2 ms (flowschema 3.77x faster) |
| checking every answer of the 9,779 schemas both check whole | 3.10 ms | 4.25 ms (flowschema 1.37x faster) |

The two agree on 99.93% of the answers (`corpus-disagreements.tsv`). 21 schemas (0.19%) ask something not checked here: `pattern` (19), `$ref outside the schema` (1), `format` (1). flowschema is slower by over 5% on 1224 schemas (`corpus-slower.tsv`).

<!-- /bench:corpus -->
