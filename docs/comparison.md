# Comparison

flowschema beside the [`jsonschema`](https://github.com/Stranger6667/jsonschema) crate, the
reference validator in Rust: both built as `cargo build --release` builds them (`opt-level = 3`),
with fat LTO and one codegen unit, both asserting `format`; the size is measured built for size.
The method is in [bench/README.md](../bench/README.md); the tables between the `bench` marks are
written by `bench/run.sh` from `bench/results/`, and the machine they ran on is in
[bench/results/latest.md](../bench/results/latest.md).

<p align="center">
  <img alt="flowschema beside the jsonschema crate, every panel below in one picture" src="../bench/results/charts/compare.png" width="878">
</p>

## Typical checks

The checks an agent makes: a structured answer valid and not, rows and wide objects, a tree by
`$ref`, a union of tool calls, and single keywords. Each time is the median of 31 rounds, and the
allocations are counted exactly.

<!-- bench:micro -->

| check | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict, valid | 115 ns (0 alloc) | 162 ns (0 alloc) | 1.42x |
| review verdict, invalid | 20 ns (0 alloc) | 34 ns (0 alloc) | 1.70x |
| 1,000 rows | 25.57 µs (0 alloc) | 48.00 µs (0 alloc) | 1.88x |
| 500 objects × 20 fields | 30.10 µs (0 alloc) | 167.67 µs (0 alloc) | 5.57x |
| tree of 511 nodes by `$ref` | 7.55 µs (0 alloc) | 10.17 µs (7 alloc) | 1.35x |
| 64 tool calls, `oneOf` of 8 | 2.05 µs (0 alloc) | 2.70 µs (0 alloc) | 1.32x |
| `uniqueItems`, 1,000 strings | 2.73 µs (1 alloc) | 4.54 µs (1 alloc) | 1.66x |
| `enum` of 1,000, the last | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.28x |
| `format: date-time` | 6 ns (0 alloc) | 10 ns (0 alloc) | 1.57x |
| `format: email` | 20 ns (0 alloc) | 77 ns (1 alloc) | 3.89x |
| `format: uuid` | 4 ns (0 alloc) | 4 ns (0 alloc) | 1.06x |
| `format: ipv4` | 5 ns (0 alloc) | 7 ns (0 alloc) | 1.26x |
| `format: uri` | 14 ns (0 alloc) | 21 ns (0 alloc) | 1.52x |
| `format: regex`, `^[a-z]+\d$` | 52 ns (0 alloc) | 66 ns (3 alloc) | 1.26x |
| `format: regex`, `^\p{L}+$` | 48 ns (0 alloc) | 58 ns (3 alloc) | 1.22x |
| `format: regex`, `^\P{ASCII}+$` | 52 ns (0 alloc) | 62 ns (3 alloc) | 1.19x |
| `format: regex`, `[\P{Letter}]` | 46 ns (0 alloc) | 82 ns (3 alloc) | 1.78x |
| `format: regex`, `(?=a)b` | 56 ns (0 alloc) | 88 ns (4 alloc) | 1.59x |
| `format: regex`, `(a)\1` | 61 ns (0 alloc) | 86 ns (4 alloc) | 1.41x |
| `format: regex`, `(?<n>a)\k<n>` | 76 ns (0 alloc) | 165 ns (7 alloc) | 2.19x |
| `pattern: ^[A-Z]{3}$` | 6 ns (0 alloc) | 15 ns (0 alloc) | 2.36x |
| `pattern: ^\d{4}-\d{2}-\d{2}$` | 8 ns (0 alloc) | 17 ns (0 alloc) | 2.20x |
| `pattern: ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$` | 9 ns (0 alloc) | 18 ns (0 alloc) | 1.93x |
| `pattern: ^(small\|medium\|large)$` | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.15x |

| compiled once | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict | 1.08 µs | 6.43 µs | 5.93x |
| tool calls | 6.02 µs | 27.18 µs | 4.51x |

| one request | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict: compiled, one answer checked | 1.18 µs | 6.31 µs | 5.34x |
| one tool call: compiled, one answer checked | 483 ns | 3.88 µs | 8.04x |

Every message of an invalid verdict: flowschema 613 ns, jsonschema 754 ns (1.23x).

<!-- /bench:micro -->

## Real LLM tasks

<!-- bench:corpus -->

| | flowschema | jsonschema 0.60 |
|---|---|---|
| schemas compiled | 11,306 of 11,306 (0 panics) | 11,298 (8 refused) |
| answers judged as labelled | 36,984 of 36,990 (99.98%) | 36,970 (99.95%) |
| compiling every schema | 123.4 ms | 466.9 ms (flowschema 3.78x faster) |
| checking every answer of the 9,779 schemas both check whole | 3.41 ms | 4.25 ms (flowschema 1.25x faster) |

The two agree on 99.93% of the answers (`corpus-disagreements.tsv`). 21 schemas (0.19%) ask something not checked here: `pattern` (19), `$ref outside the schema` (1), `format` (1). flowschema is slower by over 5% on 1812 schemas (`corpus-slower.tsv`).

<!-- /bench:corpus -->

## Size

<!-- bench:size -->

| added to a release binary built for size | size |
|---|---|
| flowschema, all of it | 293 KB |
| of which its own code | 244 KB |
| of which regex-lite | 49 KB |
| jsonschema 0.60 | 1,884 KB |

| crates added to the build, itself included | crates |
|---|---|
| flowschema | 2 |
| jsonschema 0.60 | 57 |

Over a program that reads the JSON alone (312 KB), stripped, `opt-level = "z"`, fat LTO. regex-lite is measured alone, as a program that uses it pays for it.

<!-- /bench:size -->

## How it is fast

- **Compiled once** into nodes: the types a bit mask, each bound in the order its message says
  it, every `$ref` the node it lands on, an `enum` of many options a hash table, a schema
  with nothing of a kind (no array keyword) a bit that skips the kind.
- **A schema object read in one pass** over its fields, every keyword it says kept where the
  draft knows it; the index of `$ref`s built only for a schema that has one; the document read
  where it is, and not copied.
- **Common leaves held as what they are**: a leaf that asks only for a type is a node of its own
  with no rules allocated, and `{}` is `true`; an `enum` of strings, a `const`, an integer
  between bounds and a string's own keywords are settled where the walk stands, held or surely
  not, with no walk of their rules. A property is told what its node asks, through a `$ref`
  too, so most fields are judged with no node read.
- **Objects in one pass**: the data's fields and the schema's `properties` are both sorted by
  name, so the quick walk of an object held to `properties`, `required` and
  `additionalProperties` meets each field's schema in about one comparison, told equal by two
  loads that overlap rather than a call to `memcmp`; each required field sets a bit as it is
  met, and `required` is one comparison of masks, nothing looked up.
- **Tagged unions**: a `oneOf` or `anyOf` whose branches each require one field and hold it to
  their own strings (`"type": {"const": "read"}`), as tool calls are written, reads that field
  and tries only the branches its value names.
- **Patterns** by the cheapest matcher that takes them: a comparison or one scan for the common
  shapes (`^[A-Z]{3}$`, `^(small|medium|large)$`), else a position automaton simulated
  bit-parallel, else `regex-lite`; every one linear in the string, none backtracks.
- **`uniqueItems`** through an open-addressed set of the items' hashes (a hash that agrees with
  JSON's equality: `1` and `1.0` alike, an object's fields in any order).
- **Types alone folded**: an `anyOf` of types only, or a `oneOf` of types no two share, is one
  mask; a `$ref` to a schema that asks only a type is that type; an array whose items ask only
  a type, or are such arrays in turn, is checked in one loop over its items.
- **`unevaluated*` settled as it compiles**: where what a schema applies to every value
  (`$ref`, `allOf`) takes every field or item, nothing is left to evaluate and nothing is
  walked; elsewhere what was looked at is marked on the stack, and only once the rest holds.
- **A quick walk first**: `matches` stops at the first failure and writes nothing; only
  `mismatches` on data that failed walks again for every message.

## Where `jsonschema` suits better

- A schema it happens to walk faster: flowschema is ahead over all of MaskBench's answers, not on
  every schema, and the ones it is slower on by over 5% are listed in
  `bench/results/corpus-slower.tsv` (unions of references, as the Model Context Protocol's own
  schemas are written, among them).
- A schema that needs a pattern's lookaround or back-reference, an `idn-*` format, or a `$ref`
  fetched over the network: flowschema names these as unchecked; `jsonschema` checks them.
- Errors as structured values (a path, a keyword, a kind) for a program to branch on: flowschema
  writes messages, for a model to read.
