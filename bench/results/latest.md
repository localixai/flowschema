# The latest run

2026-11-14, flowschema b9d3dd7 with changes not yet committed, on Apple M5 (macOS 26.6.2), rustc 1.100.0, against jsonschema 0.60.
Each check's time is the median of 31 rounds; allocations are counted exactly.

## Typical checks

| check | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict, valid | 113 ns (0 alloc) | 159 ns (0 alloc) | 1.41x |
| review verdict, invalid | 17 ns (0 alloc) | 32 ns (0 alloc) | 1.95x |
| 1,000 rows | 25.73 µs (0 alloc) | 47.53 µs (0 alloc) | 1.85x |
| 500 objects × 20 fields | 23.43 µs (0 alloc) | 158.75 µs (0 alloc) | 6.77x |
| tree of 511 nodes by `$ref` | 5.05 µs (0 alloc) | 10.18 µs (7 alloc) | 2.02x |
| 64 tool calls, `oneOf` of 8 | 1.74 µs (0 alloc) | 2.70 µs (0 alloc) | 1.55x |
| `uniqueItems`, 1,000 strings | 2.71 µs (1 alloc) | 4.53 µs (1 alloc) | 1.67x |
| `enum` of 1,000, the last | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.16x |
| `format: date-time` | 6 ns (0 alloc) | 9 ns (0 alloc) | 1.46x |
| `format: email` | 20 ns (0 alloc) | 77 ns (1 alloc) | 3.87x |
| `format: uuid` | 4 ns (0 alloc) | 4 ns (0 alloc) | 1.06x |
| `format: ipv4` | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.40x |
| `format: uri` | 14 ns (0 alloc) | 21 ns (0 alloc) | 1.52x |
| `format: regex`, `^[a-z]+\d$` | 51 ns (0 alloc) | 60 ns (3 alloc) | 1.18x |
| `format: regex`, `^\p{L}+$` | 46 ns (0 alloc) | 59 ns (3 alloc) | 1.30x |
| `format: regex`, `^\P{ASCII}+$` | 50 ns (0 alloc) | 60 ns (3 alloc) | 1.20x |
| `format: regex`, `[\P{Letter}]` | 44 ns (0 alloc) | 61 ns (3 alloc) | 1.40x |
| `format: regex`, `(?=a)b` | 51 ns (0 alloc) | 76 ns (4 alloc) | 1.48x |
| `format: regex`, `(a)\1` | 53 ns (0 alloc) | 79 ns (4 alloc) | 1.49x |
| `format: regex`, `(?<n>a)\k<n>` | 70 ns (0 alloc) | 122 ns (7 alloc) | 1.76x |
| `pattern: ^[A-Z]{3}$` | 6 ns (0 alloc) | 13 ns (0 alloc) | 2.21x |
| `pattern: ^\d{4}-\d{2}-\d{2}$` | 7 ns (0 alloc) | 15 ns (0 alloc) | 2.09x |
| `pattern: ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$` | 9 ns (0 alloc) | 19 ns (0 alloc) | 2.08x |
| `pattern: ^(small\|medium\|large)$` | 4 ns (0 alloc) | 5 ns (0 alloc) | 1.14x |

| compiled once | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict | 1.05 µs | 6.08 µs | 5.80x |
| tool calls | 5.59 µs | 25.43 µs | 4.55x |

| one request | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict: compiled, one answer checked | 1.14 µs | 5.99 µs | 5.24x |
| one tool call: compiled, one answer checked | 462 ns | 3.61 µs | 7.82x |
| nested models: compiled, one answer checked | 2.24 µs | 10.07 µs | 4.49x |

Every message of an invalid verdict: flowschema 569 ns, jsonschema 722 ns (1.27x).

## The official test suite

| draft | flowschema | jsonschema 0.60 | tests left out (a keyword not checked here) |
|---|---|---|---|
| draft4 | 928/928 | 928/928 | 20 tests |
| draft6 | 1256/1256 | 1256/1256 | 20 tests |
| draft7 | 1584/1584 | 1584/1584 | 145 tests |
| draft2019-09 | 1944/1944 | 1944/1944 | 185 tests |
| draft2020-12 | 1963/1963 | 1963/1963 | 188 tests |

Tests left out, by what they ask that is not checked: `format` (375), `pattern` (76), `$dynamicRef across resources` (73), `$ref outside the schema` (34).

Failures: 0.

## MaskBench

| | flowschema | jsonschema 0.60 |
|---|---|---|
| schemas compiled | 11,306 of 11,306 (0 panics) | 11,298 (8 refused) |
| answers judged as labelled | 36,984 of 36,990 (99.98%) | 36,970 (99.95%) |
| compiling every schema | 123.7 ms | 462.6 ms (flowschema 3.74x faster) |
| checking every answer of the 9,779 schemas both check whole | 2.98 ms | 4.22 ms (flowschema 1.41x faster) |

The two agree on 99.93% of the answers (`corpus-disagreements.tsv`). 21 schemas (0.19%) ask something not checked here: `pattern` (19), `$ref outside the schema` (1), `format` (1). flowschema is slower by over 5% on 1166 schemas (`corpus-slower.tsv`).

## Size

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
